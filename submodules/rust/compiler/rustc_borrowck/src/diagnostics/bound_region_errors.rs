mkuse!{use std :: sync :: Mutex ;}
mkuse!{use std :: collections :: HashMap ;}
mkuse!{use std :: sync :: LazyLock ;}
mkitem!{static USE_MATRIX : LazyLock < Mutex < HashMap < String , Vec < String > > > > = LazyLock :: new (| | Mutex :: new (HashMap :: new ())) ;}

macro_rules! get_use_matrix_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_use_matrix in module {}", module_path!());
    };
}

mkfn!{
    get_use_matrix_introspect!();
    pub fn get_use_matrix () -> HashMap < String , Vec < String > > { USE_MATRIX . lock () . unwrap () . clone () }
}
mkitem!{macro_rules ! emit_message { ($ ($ arg : tt) *) => { { use std :: fs :: OpenOptions ; use std :: io :: Write ; let message = format ! ($ ($ arg) *) ; if let Ok (mut file) = OpenOptions :: new () . create (true) . append (true) . open ("macro_report.txt") { let _ = writeln ! (file , "{}" , message) ; } } } ; }}
mkitem!{macro_rules ! mkfn { ($ introspect : expr ; $ (#[$ attr : meta]) * pub fn $ name : ident < F : FnOnce () -> R , R > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * pub fn $ name < F : FnOnce () -> R , R > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_trait_bounds_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_trait_bounds_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn $ name : ident < F : FnOnce () -> R , R > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * fn $ name < F : FnOnce () -> R , R > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: trait_bounds_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: trait_bounds_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn catch_fatal_errors < F : FnOnce () -> R , R > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * fn catch_fatal_errors < F : FnOnce () -> R , R > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: catch_fatal_errors_specific") ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: catch_fatal_errors_specific") ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn parse_crate_attrs < $ lifetime : lifetime > ($ ($ param : tt) *) -> PResult < $ lifetime2 : lifetime , ast :: AttrVec > $ body : block) => { $ (#[$ attr]) * fn parse_crate_attrs < $ lifetime > ($ ($ param) *) -> PResult < $ lifetime2 , ast :: AttrVec > { $ introspect ; emit_message ! ("🚀 MARKER: parse_crate_attrs_specific") ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: parse_crate_attrs_specific") ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn init_logger_with_additional_layer < F , T > ($ ($ param : tt) *) where F : FnOnce () -> T , T : rustc_log :: BuildSubscriberRet , $ body : block) => { $ (#[$ attr]) * fn init_logger_with_additional_layer < F , T > ($ ($ param) *) where F : FnOnce () -> T , T : rustc_log :: BuildSubscriberRet , { $ introspect ; emit_message ! ("🚀 MARKER: init_logger_specific") ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: init_logger_specific") ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn $ name : ident < F , T > ($ ($ param : tt) *) $ (-> $ ret : ty) ? where F : FnOnce ($ ($ fnonce_args : tt) *) $ ($ where_rest : tt) * $ body : block) => { $ (#[$ attr]) * fn $ name < F , T > ($ ($ param) *) $ (-> $ ret) ? where F : FnOnce ($ ($ fnonce_args) *) $ ($ where_rest) * { $ introspect ; emit_message ! ("🚀 MARKER: two_generics_where_fnonce - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: two_generics_where_fnonce - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn $ name : ident < $ gen1 : ident , $ gen2 : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? where $ ($ where_clause : tt) * $ body : block) => { $ (#[$ attr]) * fn $ name < $ gen1 , $ gen2 > ($ ($ param) *) $ (-> $ ret) ? where $ ($ where_clause) * { $ introspect ; emit_message ! ("🚀 MARKER: two_generics_where - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: two_generics_where - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn $ name : ident < $ gen1 : ident , $ gen2 : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * fn $ name < $ gen1 , $ gen2 > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: two_generics - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: two_generics - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * pub fn $ name : ident < $ gen1 : ident , $ gen2 : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * pub fn $ name < $ gen1 , $ gen2 > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_two_generics - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_two_generics - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * pub ($ vis : ident) fn $ name : ident ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * pub ($ vis) fn $ name ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_vis - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_vis - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn $ name : ident < $ gen : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * fn $ name < $ gen > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: generic_single - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: generic_single - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * pub fn $ name : ident < $ gen : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * pub fn $ name < $ gen > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_generic_single - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_generic_single - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn $ name : ident < $ lifetime : lifetime > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * fn $ name < $ lifetime > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: lifetime - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: lifetime - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * pub fn $ name : ident < $ lifetime : lifetime > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * pub fn $ name < $ lifetime > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_lifetime - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_lifetime - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * pub fn $ name : ident ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * pub fn $ name ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_non_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_non_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn $ name : ident ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * fn $ name ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: non_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: non_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; fn $ name : ident () $ body : block) => { fn $ name () { $ introspect ; emit_message ! ("🚀 MARKER: simple - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: simple - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ ($ anything : tt) *) => { $ ($ anything) * } ; }}
mkitem!{macro_rules ! safe_println { ($ ($ arg : tt) *) => { () } ; }}
mkitem!{macro_rules ! safe_print { ($ ($ arg : tt) *) => { () } ; }}
mkitem!{#[macro_export] macro_rules ! include_rust_compiler { ($ crate_name : literal , $ subpath : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_compiler_rustc_" , $ crate_name , "_" , $ subpath , "_" , $ file , ".rs")) ; } ; ($ crate_name : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_compiler_rustc_" , $ crate_name , "_src_" , $ file , ".rs")) ; } ; ($ crate_name : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_compiler_rustc_" , $ crate_name , "_src_lib.rs")) ; } ; }}
mkitem!{#[macro_export] macro_rules ! include_rust_library { ($ lib_name : literal , $ subpath : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_library_" , $ lib_name , "_" , $ subpath , "_" , $ file , ".rs")) ; } ; ($ lib_name : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_library_" , $ lib_name , "_src_" , $ file , ".rs")) ; } ; ($ lib_name : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_library_" , $ lib_name , "_src_lib.rs")) ; } ; }}
mkitem!{#[macro_export] macro_rules ! include_processed { ($ path : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_" , $ path , ".rs")) ; } ; }}
mkitem!{macro_rules ! mkinclude { ($ path : ident) => { } ; ($ path : literal) => { include ! ($ path) } ; }}
mkitem!{macro_rules ! mkitem { (include ! ($ path : ident) ;) => { } ; ($ macro_name : ident :: $ macro_sub : ident ! { $ string_lit : literal }) => { $ macro_name :: $ macro_sub ! { $ string_lit } } ; ($ macro_name : ident :: $ macro_sub : ident ! { $ ($ args : tt) * }) => { $ macro_name :: $ macro_sub ! { $ ($ args) * } } ; ($ macro_name : ident ! { $ ($ args : tt) * }) => { $ macro_name ! { $ ($ args) * } } ; ($ item : item) => { $ item } ; }}
mkitem!{#[macro_export] macro_rules ! mkmod { ($ name : ident , { $ ($ content : tt) * }) => { emit_message ! ("MOD|{}|{}" , module_path ! () , stringify ! ($ name)) ; mod $ name { const MODULE_NAME : & str = stringify ! ($ name) ; $ ($ content) * } } ; (pub mod $ name : ident { $ ($ content : tt) * }) => { emit_message ! ("MOD|{}|{}" , module_path ! () , stringify ! ($ name)) ; pub mod $ name { const MODULE_NAME : & str = stringify ! ($ name) ; $ ($ content) * } } ; (mod $ name : ident { $ ($ content : tt) * }) => { emit_message ! ("MOD|{}|{}" , module_path ! () , stringify ! ($ name)) ; mod $ name { const MODULE_NAME : & str = stringify ! ($ name) ; $ ($ content) * } } ; }}
mkitem!{#[macro_export] macro_rules ! mkuse { ($ use_stmt : item) => { emit_message ! ("USE|{}|{}" , module_path ! () , stringify ! ($ use_stmt)) ; $ use_stmt } ; }}
mkitem!{macro_rules ! mkstruct { ($ struct_def : item) => { $ struct_def } ; }}
mkitem!{macro_rules ! mkenum { ($ enum_def : item) => { $ enum_def } ; }}
mkitem!{macro_rules ! mktrait { ($ trait_def : item) => { $ trait_def } ; }}
mkitem!{macro_rules ! mkimpl { ($ impl_def : item) => { $ impl_def } ; }}
mkitem!{macro_rules ! getname { ($ name : ident) => { stringify ! ($ name) } ; }}
mkitem!{macro_rules ! getsrc { ($ name : ident) => { "processed file" } ; }}
mkitem!{macro_rules ! getpath { ($ name : ident) => { "processed_path" } ; }}
mkitem!{macro_rules ! get_deps { ($ name : ident) => { vec ! [] } ; }}
mkmod!{rustc_complete, { 
                getname!(rustc_complete);
                getsrc!(rustc_complete);
                getpath!(rustc_complete);
                get_deps!(rustc_complete);
                get_crates!(rustc_complete);
                mkinclude!(rustc_complete);
                mkmod!{emitter, { 
                getname!(emitter);
                getsrc!(emitter);
                getpath!(emitter);
                get_deps!(emitter);
                get_crates!(emitter);
                mkinclude!(emitter);
                
macro_rules! stderr_destination_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function stderr_destination in module {}", module_path!());
    };
}

mkfn!{
    stderr_destination_introspect!();
    pub fn stderr_destination () { }
} 
            }}
mkmod!{registry, { 
                getname!(registry);
                getsrc!(registry);
                getpath!(registry);
                get_deps!(registry);
                get_crates!(registry);
                mkinclude!(registry);
                mkitem!{mkstruct!{pub struct Registry ;}} 
            }}
mkmod!{translation, { 
                getname!(translation);
                getsrc!(translation);
                getpath!(translation);
                get_deps!(translation);
                get_crates!(translation);
                mkinclude!(translation);
                mkitem!{mkstruct!{pub struct Translator ;}} 
            }}
mkitem!{mkstruct!{pub struct ColorConfig ;}}
mkitem!{mkstruct!{pub struct DiagCtxt ;}}
mkitem!{mkstruct!{pub struct ErrCode ;}}
mkitem!{mkstruct!{pub struct FatalError ;}}
mkitem!{mkstruct!{pub struct PResult < T > (pub T) ;}}
mkmod!{markdown, { 
                getname!(markdown);
                getsrc!(markdown);
                getpath!(markdown);
                get_deps!(markdown);
                get_crates!(markdown);
                mkinclude!(markdown);
                 
            }}
mkmod!{config, { 
                getname!(config);
                getsrc!(config);
                getpath!(config);
                get_deps!(config);
                get_crates!(config);
                mkinclude!(config);
                mkitem!{mkstruct!{pub struct CG_OPTIONS ;}}
mkitem!{mkstruct!{pub struct CrateType ;}}
mkitem!{mkstruct!{pub struct ErrorOutputType ;}}
mkitem!{mkstruct!{pub struct Input ;}}
mkitem!{mkstruct!{pub struct OptionDesc ;}}
mkitem!{mkstruct!{pub struct OutFileName ;}}
mkitem!{mkstruct!{pub struct OutputType ;}}
mkitem!{mkstruct!{pub struct Sysroot ;}}
mkitem!{mkstruct!{pub struct UnstableOptions ;}}
mkitem!{mkstruct!{pub struct Z_OPTIONS ;}}

macro_rules! nightly_options_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function nightly_options in module {}", module_path!());
    };
}

mkfn!{
    nightly_options_introspect!();
    pub fn nightly_options () { }
}

macro_rules! parse_target_triple_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_target_triple in module {}", module_path!());
    };
}

mkfn!{
    parse_target_triple_introspect!();
    pub fn parse_target_triple () { }
} 
            }}
mkmod!{getopts, { 
                getname!(getopts);
                getsrc!(getopts);
                getpath!(getopts);
                get_deps!(getopts);
                get_crates!(getopts);
                mkinclude!(getopts);
                mkitem!{mkstruct!{pub struct Matches ;}} 
            }}
mkmod!{lint, { 
                getname!(lint);
                getsrc!(lint);
                getpath!(lint);
                get_deps!(lint);
                get_crates!(lint);
                mkinclude!(lint);
                mkitem!{mkstruct!{pub struct Lint ;}}
mkitem!{mkstruct!{pub struct LintId ;}} 
            }}
mkmod!{output, { 
                getname!(output);
                getsrc!(output);
                getpath!(output);
                get_deps!(output);
                get_crates!(output);
                mkinclude!(output);
                mkitem!{mkstruct!{pub struct CRATE_TYPES ;}}

macro_rules! collect_crate_types_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function collect_crate_types in module {}", module_path!());
    };
}

mkfn!{
    collect_crate_types_introspect!();
    pub fn collect_crate_types () { }
}

macro_rules! invalid_output_for_target_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function invalid_output_for_target in module {}", module_path!());
    };
}

mkfn!{
    invalid_output_for_target_introspect!();
    pub fn invalid_output_for_target () { }
} 
            }}
mkitem!{mkstruct!{pub struct EarlyDiagCtxt ;}}
mkitem!{mkstruct!{pub struct Session ;}}
mkitem!{mkstruct!{pub struct FileName ;}}
mkmod!{def_id, { 
                getname!(def_id);
                getsrc!(def_id);
                getpath!(def_id);
                get_deps!(def_id);
                get_crates!(def_id);
                mkinclude!(def_id);
                mkitem!{mkstruct!{pub struct LOCAL_CRATE ;}} 
            }}
mkmod!{ty, { 
                getname!(ty);
                getsrc!(ty);
                getpath!(ty);
                get_deps!(ty);
                get_crates!(ty);
                mkinclude!(ty);
                mkitem!{mkstruct!{pub struct TyCtxt < T > (pub T) ;}} 
            }} 
            }}
mkmod!{session_diagnostics, { 
                getname!(session_diagnostics);
                getsrc!(session_diagnostics);
                getpath!(session_diagnostics);
                get_deps!(session_diagnostics);
                get_crates!(session_diagnostics);
                mkinclude!(session_diagnostics);
                mkitem!{mkstruct!{pub struct CantEmitMIR ;}}
mkitem!{mkstruct!{pub struct RLinkEmptyVersionNumber ;}}
mkitem!{mkstruct!{pub struct RLinkEncodingVersionMismatch ;}}
mkitem!{mkstruct!{pub struct RLinkRustcVersionMismatch ;}}
mkitem!{mkstruct!{pub struct RLinkWrongFileType ;}}
mkitem!{mkstruct!{pub struct RlinkCorruptFile ;}}
mkitem!{mkstruct!{pub struct RlinkNotAFile ;}}
mkitem!{mkstruct!{pub struct RlinkUnableToRead ;}}
mkitem!{mkstruct!{pub struct UnstableFeatureUsage ;}} 
            }}
mkitem!{macro_rules ! do_not_use_print { ($ ($ t : tt) *) => { compile_error ! ("Don't use print") } ; }}
mkitem!{macro_rules ! do_not_use_safe_print { ($ ($ t : tt) *) => { compile_error ! ("Don't use safe_print") } ; }}
mkitem!{macro_rules ! mktrait { ($ trait_def : item) => { $ trait_def } ; }}
mkitem!{macro_rules ! mkimpl { ($ impl_def : item) => { $ impl_def } ; }}
mkitem!{macro_rules ! getname { ($ name : ident) => { pub fn get_module_name () -> &'static str { stringify ! ($ name) } } ; }}
mkitem!{macro_rules ! getsrc { ($ name : ident) => { pub fn get_source_info () -> &'static str { concat ! ("Module: " , stringify ! ($ name)) } } ; }}
mkitem!{macro_rules ! getpath { ($ name : ident) => { pub fn get_module_path () -> &'static str { module_path ! () } } ; }}
mkitem!{macro_rules ! get_deps { ($ name : ident) => { pub fn get_dependencies () -> &'static [&'static str] { & [] } } ; }}
mkitem!{macro_rules ! get_crates { ($ name : ident) => { pub fn get_required_crates () -> &'static [&'static str] { & [] } } ; }}
mkitem!{macro_rules ! forall_crates { ($ ($ crate_name : ident) ,*) => { $ (extern crate $ crate_name ;) * } ; }}
mkitem!{macro_rules ! emit_extern { ($ crate_name : ident) => { extern crate $ crate_name ; } ; }}
mkitem!{macro_rules ! get_externs { ($ crate_name : ident) => { stringify ! ($ crate_name) } ; }}
mkuse!{use std :: fmt ;}
mkuse!{use std :: rc :: Rc ;}
mkuse!{use rustc_errors :: Diag ;}
mkuse!{use rustc_hir :: def_id :: LocalDefId ;}
mkuse!{use rustc_infer :: infer :: region_constraints :: { Constraint , ConstraintKind , RegionConstraintData } ;}
mkuse!{use rustc_infer :: infer :: { InferCtxt , RegionResolutionError , RegionVariableOrigin , SubregionOrigin , TyCtxtInferExt as _ , } ;}
mkuse!{use rustc_infer :: traits :: ObligationCause ;}
mkuse!{use rustc_infer :: traits :: query :: { CanonicalTypeOpAscribeUserTypeGoal , CanonicalTypeOpDeeplyNormalizeGoal , CanonicalTypeOpNormalizeGoal , CanonicalTypeOpProvePredicateGoal , } ;}
mkuse!{use rustc_middle :: ty :: error :: TypeError ;}
mkuse!{use rustc_middle :: ty :: { self , RePlaceholder , Region , RegionVid , Ty , TyCtxt , TypeFoldable , UniverseIndex , } ;}
mkuse!{use rustc_span :: Span ;}
mkuse!{use rustc_trait_selection :: error_reporting :: InferCtxtErrorExt ;}
mkuse!{use rustc_trait_selection :: error_reporting :: infer :: nice_region_error :: NiceRegionError ;}
mkuse!{use rustc_trait_selection :: traits :: ObligationCtxt ;}
mkuse!{use rustc_traits :: { type_op_ascribe_user_type_with_span , type_op_prove_predicate_with_cause } ;}
mkuse!{use tracing :: { debug , instrument } ;}
mkuse!{use crate :: MirBorrowckCtxt ;}
mkuse!{use crate :: region_infer :: values :: RegionElement ;}
mkuse!{use crate :: session_diagnostics :: { HigherRankedErrorCause , HigherRankedLifetimeError , HigherRankedSubtypeError , } ;}
mkitem!{mkenum!{#[doc = " What operation a universe was created for."] #[derive (Clone)] pub (crate) enum UniverseInfo < 'tcx > { #[doc = " Relating two types which have binders."] RelateTys { expected : Ty < 'tcx > , found : Ty < 'tcx > } , #[doc = " Created from performing a `TypeOp`."] TypeOp (Rc < dyn TypeOpInfo < 'tcx > + 'tcx >) , #[doc = " Any other reason."] Other , }}}
mkitem!{mkimpl!{impl < 'tcx > UniverseInfo < 'tcx > { pub (crate) fn other () -> UniverseInfo < 'tcx > { UniverseInfo :: Other } pub (crate) fn relate (expected : Ty < 'tcx > , found : Ty < 'tcx >) -> UniverseInfo < 'tcx > { UniverseInfo :: RelateTys { expected , found } } pub (crate) fn report_erroneous_element (& self , mbcx : & mut MirBorrowckCtxt < '_ , '_ , 'tcx > , placeholder : ty :: PlaceholderRegion , error_element : RegionElement , cause : ObligationCause < 'tcx > ,) { match * self { UniverseInfo :: RelateTys { expected , found } => { let err = mbcx . infcx . err_ctxt () . report_mismatched_types (& cause , mbcx . infcx . param_env , expected , found , TypeError :: RegionsPlaceholderMismatch ,) ; mbcx . buffer_error (err) ; } UniverseInfo :: TypeOp (ref type_op_info) => { type_op_info . report_erroneous_element (mbcx , placeholder , error_element , cause) ; } UniverseInfo :: Other => { mbcx . buffer_error (mbcx . dcx () . create_err (HigherRankedSubtypeError { span : cause . span }) ,) ; } } } }}}
mkitem!{mktrait!{pub (crate) trait ToUniverseInfo < 'tcx > { fn to_universe_info (self , base_universe : ty :: UniverseIndex) -> UniverseInfo < 'tcx > ; }}}
mkitem!{mkimpl!{impl < 'tcx > ToUniverseInfo < 'tcx > for crate :: type_check :: InstantiateOpaqueType < 'tcx > { fn to_universe_info (self , base_universe : ty :: UniverseIndex) -> UniverseInfo < 'tcx > { UniverseInfo :: TypeOp (Rc :: new (crate :: type_check :: InstantiateOpaqueType { base_universe : Some (base_universe) , .. self })) } }}}
mkitem!{mkimpl!{impl < 'tcx > ToUniverseInfo < 'tcx > for CanonicalTypeOpProvePredicateGoal < 'tcx > { fn to_universe_info (self , base_universe : ty :: UniverseIndex) -> UniverseInfo < 'tcx > { UniverseInfo :: TypeOp (Rc :: new (PredicateQuery { canonical_query : self , base_universe })) } }}}
mkitem!{mkimpl!{impl < 'tcx , T : Copy + fmt :: Display + TypeFoldable < TyCtxt < 'tcx > > + 'tcx > ToUniverseInfo < 'tcx > for CanonicalTypeOpNormalizeGoal < 'tcx , T > { fn to_universe_info (self , base_universe : ty :: UniverseIndex) -> UniverseInfo < 'tcx > { UniverseInfo :: TypeOp (Rc :: new (NormalizeQuery { canonical_query : self , base_universe })) } }}}
mkitem!{mkimpl!{impl < 'tcx , T : Copy + fmt :: Display + TypeFoldable < TyCtxt < 'tcx > > + 'tcx > ToUniverseInfo < 'tcx > for CanonicalTypeOpDeeplyNormalizeGoal < 'tcx , T > { fn to_universe_info (self , base_universe : ty :: UniverseIndex) -> UniverseInfo < 'tcx > { UniverseInfo :: TypeOp (Rc :: new (DeeplyNormalizeQuery { canonical_query : self , base_universe })) } }}}
mkitem!{mkimpl!{impl < 'tcx > ToUniverseInfo < 'tcx > for CanonicalTypeOpAscribeUserTypeGoal < 'tcx > { fn to_universe_info (self , base_universe : ty :: UniverseIndex) -> UniverseInfo < 'tcx > { UniverseInfo :: TypeOp (Rc :: new (AscribeUserTypeQuery { canonical_query : self , base_universe })) } }}}
mkitem!{mkimpl!{impl < 'tcx > ToUniverseInfo < 'tcx > for ! { fn to_universe_info (self , _base_universe : ty :: UniverseIndex) -> UniverseInfo < 'tcx > { self } }}}
mkitem!{mktrait!{#[allow (unused_lifetimes)] pub (crate) trait TypeOpInfo < 'tcx > { #[doc = " Returns an error to be reported if rerunning the type op fails to"] #[doc = " recover the error's cause."] fn fallback_error (& self , tcx : TyCtxt < 'tcx > , span : Span) -> Diag < 'tcx > ; fn base_universe (& self) -> ty :: UniverseIndex ; fn nice_error < 'infcx > (& self , mbcx : & mut MirBorrowckCtxt < '_ , 'infcx , 'tcx > , cause : ObligationCause < 'tcx > , placeholder_region : ty :: Region < 'tcx > , error_region : Option < ty :: Region < 'tcx > > ,) -> Option < Diag < 'infcx > > ; #[doc = " Constraints require that `error_element` appear in the"] #[doc = "  values of `placeholder`, but this cannot be proven to"] #[doc = " hold. Report an error."] #[instrument (level = "debug" , skip (self , mbcx))] fn report_erroneous_element (& self , mbcx : & mut MirBorrowckCtxt < '_ , '_ , 'tcx > , placeholder : ty :: PlaceholderRegion , error_element : RegionElement , cause : ObligationCause < 'tcx > ,) { let tcx = mbcx . infcx . tcx ; let base_universe = self . base_universe () ; debug ! (? base_universe) ; let Some (adjusted_universe) = placeholder . universe . as_u32 () . checked_sub (base_universe . as_u32 ()) else { mbcx . buffer_error (self . fallback_error (tcx , cause . span)) ; return ; } ; let placeholder_region = ty :: Region :: new_placeholder (tcx , ty :: Placeholder { universe : adjusted_universe . into () , bound : placeholder . bound } ,) ; let error_region = if let RegionElement :: PlaceholderRegion (error_placeholder) = error_element { let adjusted_universe = error_placeholder . universe . as_u32 () . checked_sub (base_universe . as_u32 ()) ; adjusted_universe . map (| adjusted | { ty :: Region :: new_placeholder (tcx , ty :: Placeholder { universe : adjusted . into () , bound : error_placeholder . bound } ,) }) } else { None } ; debug ! (? placeholder_region) ; let span = cause . span ; let nice_error = self . nice_error (mbcx , cause , placeholder_region , error_region) ; debug ! (? nice_error) ; mbcx . buffer_error (nice_error . unwrap_or_else (| | self . fallback_error (tcx , span))) ; } }}}
mkitem!{mkstruct!{struct PredicateQuery < 'tcx > { canonical_query : CanonicalTypeOpProvePredicateGoal < 'tcx > , base_universe : ty :: UniverseIndex , }}}
mkitem!{mkimpl!{impl < 'tcx > TypeOpInfo < 'tcx > for PredicateQuery < 'tcx > { fn fallback_error (& self , tcx : TyCtxt < 'tcx > , span : Span) -> Diag < 'tcx > { tcx . dcx () . create_err (HigherRankedLifetimeError { cause : Some (HigherRankedErrorCause :: CouldNotProve { predicate : self . canonical_query . canonical . value . value . predicate . to_string () , }) , span , }) } fn base_universe (& self) -> ty :: UniverseIndex { self . base_universe } fn nice_error < 'infcx > (& self , mbcx : & mut MirBorrowckCtxt < '_ , 'infcx , 'tcx > , cause : ObligationCause < 'tcx > , placeholder_region : ty :: Region < 'tcx > , error_region : Option < ty :: Region < 'tcx > > ,) -> Option < Diag < 'infcx > > { let (infcx , key , _) = mbcx . infcx . tcx . infer_ctxt () . build_with_canonical (cause . span , & self . canonical_query) ; let ocx = ObligationCtxt :: new (& infcx) ; type_op_prove_predicate_with_cause (& ocx , key , cause) ; let diag = try_extract_error_from_fulfill_cx (& ocx , mbcx . mir_def_id () , placeholder_region , error_region ,) ? . with_dcx (mbcx . dcx ()) ; Some (diag) } }}}
mkitem!{mkstruct!{struct NormalizeQuery < 'tcx , T > { canonical_query : CanonicalTypeOpNormalizeGoal < 'tcx , T > , base_universe : ty :: UniverseIndex , }}}
mkitem!{mkimpl!{impl < 'tcx , T > TypeOpInfo < 'tcx > for NormalizeQuery < 'tcx , T > where T : Copy + fmt :: Display + TypeFoldable < TyCtxt < 'tcx > > + 'tcx , { fn fallback_error (& self , tcx : TyCtxt < 'tcx > , span : Span) -> Diag < 'tcx > { tcx . dcx () . create_err (HigherRankedLifetimeError { cause : Some (HigherRankedErrorCause :: CouldNotNormalize { value : self . canonical_query . canonical . value . value . value . to_string () , }) , span , }) } fn base_universe (& self) -> ty :: UniverseIndex { self . base_universe } fn nice_error < 'infcx > (& self , mbcx : & mut MirBorrowckCtxt < '_ , 'infcx , 'tcx > , cause : ObligationCause < 'tcx > , placeholder_region : ty :: Region < 'tcx > , error_region : Option < ty :: Region < 'tcx > > ,) -> Option < Diag < 'infcx > > { let (infcx , key , _) = mbcx . infcx . tcx . infer_ctxt () . build_with_canonical (cause . span , & self . canonical_query) ; let ocx = ObligationCtxt :: new (& infcx) ; let ty :: ParamEnvAnd { param_env , value } = key ; let _ = ocx . normalize (& cause , param_env , value . value) ; let diag = try_extract_error_from_fulfill_cx (& ocx , mbcx . mir_def_id () , placeholder_region , error_region ,) ? . with_dcx (mbcx . dcx ()) ; Some (diag) } }}}
mkitem!{mkstruct!{struct DeeplyNormalizeQuery < 'tcx , T > { canonical_query : CanonicalTypeOpDeeplyNormalizeGoal < 'tcx , T > , base_universe : ty :: UniverseIndex , }}}
mkitem!{mkimpl!{impl < 'tcx , T > TypeOpInfo < 'tcx > for DeeplyNormalizeQuery < 'tcx , T > where T : Copy + fmt :: Display + TypeFoldable < TyCtxt < 'tcx > > + 'tcx , { fn fallback_error (& self , tcx : TyCtxt < 'tcx > , span : Span) -> Diag < 'tcx > { tcx . dcx () . create_err (HigherRankedLifetimeError { cause : Some (HigherRankedErrorCause :: CouldNotNormalize { value : self . canonical_query . canonical . value . value . value . to_string () , }) , span , }) } fn base_universe (& self) -> ty :: UniverseIndex { self . base_universe } fn nice_error < 'infcx > (& self , mbcx : & mut MirBorrowckCtxt < '_ , 'infcx , 'tcx > , cause : ObligationCause < 'tcx > , placeholder_region : ty :: Region < 'tcx > , error_region : Option < ty :: Region < 'tcx > > ,) -> Option < Diag < 'infcx > > { let (infcx , key , _) = mbcx . infcx . tcx . infer_ctxt () . build_with_canonical (cause . span , & self . canonical_query) ; let ocx = ObligationCtxt :: new (& infcx) ; let ty :: ParamEnvAnd { param_env , value } = key ; let _ = ocx . deeply_normalize (& cause , param_env , value . value) ; let diag = try_extract_error_from_fulfill_cx (& ocx , mbcx . mir_def_id () , placeholder_region , error_region ,) ? . with_dcx (mbcx . dcx ()) ; Some (diag) } }}}
mkitem!{mkstruct!{struct AscribeUserTypeQuery < 'tcx > { canonical_query : CanonicalTypeOpAscribeUserTypeGoal < 'tcx > , base_universe : ty :: UniverseIndex , }}}
mkitem!{mkimpl!{impl < 'tcx > TypeOpInfo < 'tcx > for AscribeUserTypeQuery < 'tcx > { fn fallback_error (& self , tcx : TyCtxt < 'tcx > , span : Span) -> Diag < 'tcx > { tcx . dcx () . create_err (HigherRankedLifetimeError { cause : None , span }) } fn base_universe (& self) -> ty :: UniverseIndex { self . base_universe } fn nice_error < 'infcx > (& self , mbcx : & mut MirBorrowckCtxt < '_ , 'infcx , 'tcx > , cause : ObligationCause < 'tcx > , placeholder_region : ty :: Region < 'tcx > , error_region : Option < ty :: Region < 'tcx > > ,) -> Option < Diag < 'infcx > > { let (infcx , key , _) = mbcx . infcx . tcx . infer_ctxt () . build_with_canonical (cause . span , & self . canonical_query) ; let ocx = ObligationCtxt :: new (& infcx) ; type_op_ascribe_user_type_with_span (& ocx , key , cause . span) . ok () ? ; let diag = try_extract_error_from_fulfill_cx (& ocx , mbcx . mir_def_id () , placeholder_region , error_region ,) ? . with_dcx (mbcx . dcx ()) ; Some (diag) } }}}
mkitem!{mkimpl!{impl < 'tcx > TypeOpInfo < 'tcx > for crate :: type_check :: InstantiateOpaqueType < 'tcx > { fn fallback_error (& self , tcx : TyCtxt < 'tcx > , span : Span) -> Diag < 'tcx > { tcx . dcx () . create_err (HigherRankedLifetimeError { cause : None , span }) } fn base_universe (& self) -> ty :: UniverseIndex { self . base_universe . unwrap () } fn nice_error < 'infcx > (& self , mbcx : & mut MirBorrowckCtxt < '_ , 'infcx , 'tcx > , _cause : ObligationCause < 'tcx > , placeholder_region : ty :: Region < 'tcx > , error_region : Option < ty :: Region < 'tcx > > ,) -> Option < Diag < 'infcx > > { try_extract_error_from_region_constraints (mbcx . infcx , mbcx . mir_def_id () , placeholder_region , error_region , self . region_constraints . as_ref () . unwrap () , | vid | RegionVariableOrigin :: Nll (mbcx . regioncx . definitions [vid] . origin) , | vid | mbcx . regioncx . definitions [vid] . universe ,) } }}}

macro_rules! try_extract_error_from_fulfill_cx_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function try_extract_error_from_fulfill_cx in module {}", module_path!());
    };
}

mkfn!{
    try_extract_error_from_fulfill_cx_introspect!();
    #[instrument (skip (ocx) , level = "debug")] fn try_extract_error_from_fulfill_cx < 'a , 'tcx > (ocx : & ObligationCtxt < 'a , 'tcx > , generic_param_scope : LocalDefId , placeholder_region : ty :: Region < 'tcx > , error_region : Option < ty :: Region < 'tcx > > ,) -> Option < Diag < 'a > > { let _errors = ocx . select_all_or_error () ; let region_constraints = ocx . infcx . with_region_constraints (| r | r . clone ()) ; try_extract_error_from_region_constraints (ocx . infcx , generic_param_scope , placeholder_region , error_region , & region_constraints , | vid | ocx . infcx . region_var_origin (vid) , | vid | ocx . infcx . universe_of_region (ty :: Region :: new_var (ocx . infcx . tcx , vid)) ,) }
}

macro_rules! try_extract_error_from_region_constraints_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function try_extract_error_from_region_constraints in module {}", module_path!());
    };
}

mkfn!{
    try_extract_error_from_region_constraints_introspect!();
    #[instrument (level = "debug" , skip (infcx , region_var_origin , universe_of_region))] fn try_extract_error_from_region_constraints < 'a , 'tcx > (infcx : & 'a InferCtxt < 'tcx > , generic_param_scope : LocalDefId , placeholder_region : ty :: Region < 'tcx > , error_region : Option < ty :: Region < 'tcx > > , region_constraints : & RegionConstraintData < 'tcx > , mut region_var_origin : impl FnMut (RegionVid) -> RegionVariableOrigin , mut universe_of_region : impl FnMut (RegionVid) -> UniverseIndex ,) -> Option < Diag < 'a > > { let placeholder_universe = match placeholder_region . kind () { ty :: RePlaceholder (p) => p . universe , ty :: ReVar (vid) => universe_of_region (vid) , _ => ty :: UniverseIndex :: ROOT , } ; let regions_the_same = | a_region : Region < 'tcx > , b_region : Region < 'tcx > | match (a_region . kind () , b_region . kind ()) { (RePlaceholder (a_p) , RePlaceholder (b_p)) => a_p . bound == b_p . bound , _ => a_region == b_region , } ; let mut check = | c : & Constraint < 'tcx > , cause : & SubregionOrigin < 'tcx > , exact | match c . kind { ConstraintKind :: RegSubReg if ((exact && c . sup == placeholder_region) || (! exact && regions_the_same (c . sup , placeholder_region))) && c . sup != c . sub => { Some ((c . sub , cause . clone ())) } ConstraintKind :: VarSubReg if (exact && c . sup == placeholder_region && ! universe_of_region (c . sub . as_var ()) . can_name (placeholder_universe)) || (! exact && regions_the_same (c . sup , placeholder_region)) => { Some ((c . sub , cause . clone ())) } _ => None , } ; let mut find_culprit = | exact_match : bool | { region_constraints . constraints . iter () . find_map (| (constraint , cause) | check (constraint , cause , exact_match)) } ; let (sub_region , cause) = find_culprit (true) . or_else (| | find_culprit (false)) ? ; debug ! (? sub_region , "cause = {:#?}" , cause) ; let error = match (error_region , sub_region . kind ()) { (Some (error_region) , ty :: ReVar (vid)) => RegionResolutionError :: SubSupConflict (vid , region_var_origin (vid) , cause . clone () , error_region , cause . clone () , placeholder_region , vec ! [] ,) , (Some (error_region) , _) => { RegionResolutionError :: ConcreteFailure (cause . clone () , error_region , placeholder_region) } (None , ty :: ReVar (vid)) => RegionResolutionError :: UpperBoundUniverseConflict (vid , region_var_origin (vid) , universe_of_region (vid) , cause . clone () , placeholder_region ,) , (None , _) => { RegionResolutionError :: ConcreteFailure (cause . clone () , sub_region , placeholder_region) } } ; NiceRegionError :: new (& infcx . err_ctxt () , generic_param_scope , error) . try_report_from_nll () . or_else (| | { if let SubregionOrigin :: Subtype (trace) = cause { Some (infcx . err_ctxt () . report_and_explain_type_error (* trace , infcx . tcx . param_env (generic_param_scope) , TypeError :: RegionsPlaceholderMismatch ,)) } else { None } }) }
}