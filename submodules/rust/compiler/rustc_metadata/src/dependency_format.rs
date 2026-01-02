mkuse!{use rustc_data_structures :: fx :: { FxHashMap , FxHashSet } ;}
mkuse!{use rustc_hir :: def_id :: { CrateNum , LOCAL_CRATE } ;}
mkuse!{use rustc_index :: IndexVec ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: middle :: dependency_format :: { Dependencies , DependencyList , Linkage } ;}
mkuse!{use rustc_middle :: ty :: TyCtxt ;}
mkuse!{use rustc_session :: config :: CrateType ;}
mkuse!{use rustc_session :: cstore :: CrateDepKind ;}
mkuse!{use rustc_session :: cstore :: LinkagePreference :: { self , RequireDynamic , RequireStatic } ;}
mkuse!{use rustc_span :: sym ;}
mkuse!{use tracing :: info ;}
mkuse!{use crate :: creader :: CStore ;}
mkuse!{use crate :: errors :: { BadPanicStrategy , CrateDepMultiple , IncompatiblePanicInDropStrategy , LibRequired , NonStaticCrateDep , RequiredPanicStrategy , RlibRequired , RustcDriverHelp , RustcLibRequired , TwoPanicRuntimes , } ;}

macro_rules! calculate_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function calculate in module {}", module_path!());
    };
}

mkfn!{
    calculate_introspect!();
    pub (crate) fn calculate (tcx : TyCtxt < '_ >) -> Dependencies { tcx . crate_types () . iter () . map (| & ty | { let linkage = calculate_type (tcx , ty) ; verify_ok (tcx , & linkage) ; (ty , linkage) }) . collect () }
}

macro_rules! calculate_type_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function calculate_type in module {}", module_path!());
    };
}

mkfn!{
    calculate_type_introspect!();
    fn calculate_type (tcx : TyCtxt < '_ > , ty : CrateType) -> DependencyList { let sess = & tcx . sess ; if ! sess . opts . output_types . should_link () { return IndexVec :: new () ; } let preferred_linkage = match ty { CrateType :: Dylib | CrateType :: Cdylib | CrateType :: Sdylib => { if sess . opts . cg . prefer_dynamic { Linkage :: Dynamic } else { Linkage :: Static } } CrateType :: Staticlib => { if sess . opts . unstable_opts . staticlib_prefer_dynamic { Linkage :: Dynamic } else { Linkage :: Static } } CrateType :: Executable if ! sess . opts . cg . prefer_dynamic || sess . crt_static (Some (ty)) => { Linkage :: Static } CrateType :: Executable => Linkage :: Dynamic , CrateType :: ProcMacro => Linkage :: Static , CrateType :: Rlib => Linkage :: NotLinked , } ; let mut unavailable_as_static = Vec :: new () ; match preferred_linkage { Linkage :: NotLinked => return IndexVec :: new () , Linkage :: Static => { if let Some (v) = attempt_static (tcx , & mut unavailable_as_static) { return v ; } if (ty == CrateType :: Staticlib && ! sess . opts . unstable_opts . staticlib_allow_rdylib_deps) || (ty == CrateType :: Executable && sess . crt_static (Some (ty)) && ! sess . target . crt_static_allows_dylibs) { for & cnum in tcx . crates (()) . iter () { if tcx . dep_kind (cnum) . macros_only () { continue ; } let src = tcx . used_crate_source (cnum) ; if src . rlib . is_some () { continue ; } sess . dcx () . emit_err (RlibRequired { crate_name : tcx . crate_name (cnum) }) ; } return IndexVec :: new () ; } } Linkage :: Dynamic | Linkage :: IncludedFromDylib => { } } let all_dylibs = | | { tcx . crates (()) . iter () . filter (| & & cnum | { ! tcx . dep_kind (cnum) . macros_only () && (tcx . used_crate_source (cnum) . dylib . is_some () || tcx . used_crate_source (cnum) . sdylib_interface . is_some ()) }) } ; let mut upstream_in_dylibs = FxHashSet :: default () ; if tcx . features () . rustc_private () { for & cnum in all_dylibs () { let deps = tcx . dylib_dependency_formats (cnum) ; for & (depnum , style) in deps . iter () { if let RequireStatic = style { upstream_in_dylibs . insert (depnum) ; } } } } let mut formats = FxHashMap :: default () ; for & cnum in all_dylibs () { if upstream_in_dylibs . contains (& cnum) { info ! ("skipping dylib: {}" , tcx . crate_name (cnum)) ; continue ; } let name = tcx . crate_name (cnum) ; info ! ("adding dylib: {}" , name) ; add_library (tcx , cnum , RequireDynamic , & mut formats , & mut unavailable_as_static) ; let deps = tcx . dylib_dependency_formats (cnum) ; for & (depnum , style) in deps . iter () { info ! ("adding {:?}: {}" , style , tcx . crate_name (depnum)) ; add_library (tcx , depnum , style , & mut formats , & mut unavailable_as_static) ; } } let last_crate = tcx . crates (()) . len () ; let mut ret = IndexVec :: new () ; assert_eq ! (ret . push (Linkage :: Static) , LOCAL_CRATE) ; for cnum in 1 .. last_crate + 1 { let cnum = CrateNum :: new (cnum) ; assert_eq ! (ret . push (match formats . get (& cnum) { Some (& RequireDynamic) => Linkage :: Dynamic , Some (& RequireStatic) => Linkage :: IncludedFromDylib , None => Linkage :: NotLinked , }) , cnum) ; } for & cnum in tcx . crates (()) . iter () { let src = tcx . used_crate_source (cnum) ; if src . dylib . is_none () && ! formats . contains_key (& cnum) && tcx . dep_kind (cnum) == CrateDepKind :: Explicit { assert ! (src . rlib . is_some () || src . rmeta . is_some ()) ; info ! ("adding staticlib: {}" , tcx . crate_name (cnum)) ; add_library (tcx , cnum , RequireStatic , & mut formats , & mut unavailable_as_static) ; ret [cnum] = Linkage :: Static ; } } activate_injected_dep (CStore :: from_tcx (tcx) . injected_panic_runtime () , & mut ret , & | cnum | { tcx . is_panic_runtime (cnum) }) ; for (cnum , kind) in ret . iter_enumerated () { if cnum == LOCAL_CRATE { continue ; } let src = tcx . used_crate_source (cnum) ; match * kind { Linkage :: NotLinked | Linkage :: IncludedFromDylib => { } Linkage :: Static if src . rlib . is_some () => continue , Linkage :: Dynamic if src . dylib . is_some () || src . sdylib_interface . is_some () => continue , kind => { let kind = match kind { Linkage :: Static => "rlib" , _ => "dylib" , } ; let crate_name = tcx . crate_name (cnum) ; if crate_name . as_str () . starts_with ("rustc_") { sess . dcx () . emit_err (RustcLibRequired { crate_name , kind }) ; } else { sess . dcx () . emit_err (LibRequired { crate_name , kind }) ; } } } } ret }
}

macro_rules! add_library_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function add_library in module {}", module_path!());
    };
}

mkfn!{
    add_library_introspect!();
    fn add_library (tcx : TyCtxt < '_ > , cnum : CrateNum , link : LinkagePreference , m : & mut FxHashMap < CrateNum , LinkagePreference > , unavailable_as_static : & mut Vec < CrateNum > ,) { match m . get (& cnum) { Some (& link2) => { if link2 != link || link == RequireStatic { let linking_to_rustc_driver = tcx . sess . psess . unstable_features . is_nightly_build () && tcx . crates (()) . iter () . any (| & cnum | tcx . crate_name (cnum) == sym :: rustc_driver) ; tcx . dcx () . emit_err (CrateDepMultiple { crate_name : tcx . crate_name (cnum) , non_static_deps : unavailable_as_static . drain (..) . map (| cnum | NonStaticCrateDep { crate_name_ : tcx . crate_name (cnum) }) . collect () , rustc_driver_help : linking_to_rustc_driver . then_some (RustcDriverHelp) , }) ; } } None => { m . insert (cnum , link) ; } } }
}

macro_rules! attempt_static_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function attempt_static in module {}", module_path!());
    };
}

mkfn!{
    attempt_static_introspect!();
    fn attempt_static (tcx : TyCtxt < '_ > , unavailable : & mut Vec < CrateNum >) -> Option < DependencyList > { let all_crates_available_as_rlib = tcx . crates (()) . iter () . copied () . filter_map (| cnum | { if tcx . dep_kind (cnum) . macros_only () { return None ; } let is_rlib = tcx . used_crate_source (cnum) . rlib . is_some () ; if ! is_rlib { unavailable . push (cnum) ; } Some (is_rlib) }) . all (| is_rlib | is_rlib) ; if ! all_crates_available_as_rlib { return None ; } let mut ret = IndexVec :: new () ; assert_eq ! (ret . push (Linkage :: Static) , LOCAL_CRATE) ; for & cnum in tcx . crates (()) { assert_eq ! (ret . push (match tcx . dep_kind (cnum) { CrateDepKind :: Explicit => Linkage :: Static , CrateDepKind :: MacrosOnly | CrateDepKind :: Implicit => Linkage :: NotLinked , }) , cnum) ; } activate_injected_dep (CStore :: from_tcx (tcx) . injected_panic_runtime () , & mut ret , & | cnum | { tcx . is_panic_runtime (cnum) }) ; Some (ret) }
}

macro_rules! activate_injected_dep_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function activate_injected_dep in module {}", module_path!());
    };
}

mkfn!{
    activate_injected_dep_introspect!();
    # [doc = " Given a list of how to link upstream dependencies so far, ensure that an"] # [doc = " injected dependency is activated. This will not do anything if one was"] # [doc = " transitively included already (e.g., via a dylib or explicitly so)."] # [doc = ""] # [doc = " If an injected dependency was not found then we're guaranteed the"] # [doc = " metadata::creader module has injected that dependency (not listed as"] # [doc = " a required dependency) in one of the session's field. If this field is not"] # [doc = " set then this compilation doesn't actually need the dependency and we can"] # [doc = " also skip this step entirely."] fn activate_injected_dep (injected : Option < CrateNum > , list : & mut DependencyList , replaces_injected : & dyn Fn (CrateNum) -> bool ,) { for (cnum , slot) in list . iter_enumerated () { if ! replaces_injected (cnum) { continue ; } if * slot != Linkage :: NotLinked { return ; } } if let Some (injected) = injected { assert_eq ! (list [injected] , Linkage :: NotLinked) ; list [injected] = Linkage :: Static ; } }
}

macro_rules! verify_ok_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function verify_ok in module {}", module_path!());
    };
}

mkfn!{
    verify_ok_introspect!();
    # [doc = " After the linkage for a crate has been determined we need to verify that"] # [doc = " there's only going to be one panic runtime in the output."] fn verify_ok (tcx : TyCtxt < '_ > , list : & DependencyList) { let sess = & tcx . sess ; if list . is_empty () { return ; } let mut panic_runtime = None ; for (cnum , linkage) in list . iter_enumerated () { if let Linkage :: NotLinked = * linkage { continue ; } if tcx . is_panic_runtime (cnum) { if let Some ((prev , _)) = panic_runtime { let prev_name = tcx . crate_name (prev) ; let cur_name = tcx . crate_name (cnum) ; sess . dcx () . emit_err (TwoPanicRuntimes { prev_name , cur_name }) ; } panic_runtime = Some ((cnum , tcx . required_panic_strategy (cnum) . unwrap_or_else (| | { bug ! ("cannot determine panic strategy of a panic runtime") ; }) ,)) ; } } if let Some ((runtime_cnum , found_strategy)) = panic_runtime { let desired_strategy = sess . panic_strategy () ; if found_strategy != desired_strategy { sess . dcx () . emit_err (BadPanicStrategy { runtime : tcx . crate_name (runtime_cnum) , strategy : desired_strategy , }) ; } for (cnum , linkage) in list . iter_enumerated () { if let Linkage :: NotLinked = * linkage { continue ; } if cnum == runtime_cnum || tcx . is_compiler_builtins (cnum) { continue ; } if let Some (found_strategy) = tcx . required_panic_strategy (cnum) && desired_strategy != found_strategy { sess . dcx () . emit_err (RequiredPanicStrategy { crate_name : tcx . crate_name (cnum) , found_strategy , desired_strategy , }) ; } if cnum != LOCAL_CRATE { let found_drop_strategy = tcx . panic_in_drop_strategy (cnum) ; if tcx . sess . opts . unstable_opts . panic_in_drop != found_drop_strategy { sess . dcx () . emit_err (IncompatiblePanicInDropStrategy { crate_name : tcx . crate_name (cnum) , found_strategy : found_drop_strategy , desired_strategy : tcx . sess . opts . unstable_opts . panic_in_drop , }) ; } } } } }
}