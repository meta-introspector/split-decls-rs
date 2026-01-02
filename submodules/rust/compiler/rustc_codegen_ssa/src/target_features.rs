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
mkitem!{macro_rules ! emit_message { ($ ($ arg : tt) *) => { } ; }}
mkitem!{macro_rules ! mkfn { ($ introspect : expr ; $ (# [$ attr : meta]) * pub fn $ name : ident < F : FnOnce () -> R , R > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * pub fn $ name < F : FnOnce () -> R , R > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_trait_bounds_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_trait_bounds_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn $ name : ident < F : FnOnce () -> R , R > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * fn $ name < F : FnOnce () -> R , R > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: trait_bounds_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: trait_bounds_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn catch_fatal_errors < F : FnOnce () -> R , R > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * fn catch_fatal_errors < F : FnOnce () -> R , R > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: catch_fatal_errors_specific") ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: catch_fatal_errors_specific") ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn parse_crate_attrs < $ lifetime : lifetime > ($ ($ param : tt) *) -> PResult < $ lifetime2 : lifetime , ast :: AttrVec > $ body : block) => { $ (# [$ attr]) * fn parse_crate_attrs < $ lifetime > ($ ($ param) *) -> PResult < $ lifetime2 , ast :: AttrVec > { $ introspect ; emit_message ! ("🚀 MARKER: parse_crate_attrs_specific") ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: parse_crate_attrs_specific") ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn init_logger_with_additional_layer < F , T > ($ ($ param : tt) *) where F : FnOnce () -> T , T : rustc_log :: BuildSubscriberRet , $ body : block) => { $ (# [$ attr]) * fn init_logger_with_additional_layer < F , T > ($ ($ param) *) where F : FnOnce () -> T , T : rustc_log :: BuildSubscriberRet , { $ introspect ; emit_message ! ("🚀 MARKER: init_logger_specific") ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: init_logger_specific") ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn $ name : ident < F , T > ($ ($ param : tt) *) $ (-> $ ret : ty) ? where F : FnOnce ($ ($ fnonce_args : tt) *) $ ($ where_rest : tt) * $ body : block) => { $ (# [$ attr]) * fn $ name < F , T > ($ ($ param) *) $ (-> $ ret) ? where F : FnOnce ($ ($ fnonce_args) *) $ ($ where_rest) * { $ introspect ; emit_message ! ("🚀 MARKER: two_generics_where_fnonce - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: two_generics_where_fnonce - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn $ name : ident < $ gen1 : ident , $ gen2 : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? where $ ($ where_clause : tt) * $ body : block) => { $ (# [$ attr]) * fn $ name < $ gen1 , $ gen2 > ($ ($ param) *) $ (-> $ ret) ? where $ ($ where_clause) * { $ introspect ; emit_message ! ("🚀 MARKER: two_generics_where - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: two_generics_where - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn $ name : ident < $ gen1 : ident , $ gen2 : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * fn $ name < $ gen1 , $ gen2 > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: two_generics - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: two_generics - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * pub fn $ name : ident < $ gen1 : ident , $ gen2 : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * pub fn $ name < $ gen1 , $ gen2 > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_two_generics - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_two_generics - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * pub ($ vis : ident) fn $ name : ident ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * pub ($ vis) fn $ name ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_vis - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_vis - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn $ name : ident < $ gen : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * fn $ name < $ gen > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: generic_single - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: generic_single - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * pub fn $ name : ident < $ gen : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * pub fn $ name < $ gen > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_generic_single - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_generic_single - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn $ name : ident < $ lifetime : lifetime > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * fn $ name < $ lifetime > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: lifetime - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: lifetime - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * pub fn $ name : ident < $ lifetime : lifetime > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * pub fn $ name < $ lifetime > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_lifetime - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_lifetime - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * pub fn $ name : ident ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * pub fn $ name ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_non_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_non_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn $ name : ident ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * fn $ name ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: non_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: non_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; fn $ name : ident () $ body : block) => { fn $ name () { $ introspect ; emit_message ! ("🚀 MARKER: simple - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: simple - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ ($ anything : tt) *) => { $ ($ anything) * } ; }}
mkitem!{macro_rules ! safe_println { ($ ($ arg : tt) *) => { () } ; }}
mkitem!{macro_rules ! safe_print { ($ ($ arg : tt) *) => { () } ; }}
mkitem!{# [macro_export] macro_rules ! include_rust_compiler { ($ crate_name : literal , $ subpath : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_compiler_rustc_" , $ crate_name , "_" , $ subpath , "_" , $ file , ".rs")) ; } ; ($ crate_name : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_compiler_rustc_" , $ crate_name , "_src_" , $ file , ".rs")) ; } ; ($ crate_name : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_compiler_rustc_" , $ crate_name , "_src_lib.rs")) ; } ; }}
mkitem!{# [macro_export] macro_rules ! include_rust_library { ($ lib_name : literal , $ subpath : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_library_" , $ lib_name , "_" , $ subpath , "_" , $ file , ".rs")) ; } ; ($ lib_name : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_library_" , $ lib_name , "_src_" , $ file , ".rs")) ; } ; ($ lib_name : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_library_" , $ lib_name , "_src_lib.rs")) ; } ; }}
mkitem!{# [macro_export] macro_rules ! include_processed { ($ path : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_" , $ path , ".rs")) ; } ; }}
mkitem!{macro_rules ! mkinclude { ($ path : ident) => { } ; ($ path : literal) => { include ! ($ path) } ; }}
mkitem!{macro_rules ! mkitem { (include ! ($ path : ident) ;) => { } ; ($ macro_name : ident :: $ macro_sub : ident ! { $ string_lit : literal }) => { $ macro_name :: $ macro_sub ! { $ string_lit } } ; ($ macro_name : ident :: $ macro_sub : ident ! { $ ($ args : tt) * }) => { $ macro_name :: $ macro_sub ! { $ ($ args) * } } ; ($ macro_name : ident ! { $ ($ args : tt) * }) => { $ macro_name ! { $ ($ args) * } } ; ($ item : item) => { $ item } ; }}
mkitem!{# [macro_export] macro_rules ! mkmod { ($ name : ident , { $ ($ content : tt) * }) => { compile_error ! (concat ! ("MOD|" , module_path ! () , "|" , stringify ! ($ name))) ; mod $ name { const MODULE_NAME : & str = stringify ! ($ name) ; $ ($ content) * } } ; (pub mod $ name : ident { $ ($ content : tt) * }) => { compile_error ! (concat ! ("MOD|" , module_path ! () , "|" , stringify ! ($ name))) ; pub mod $ name { const MODULE_NAME : & str = stringify ! ($ name) ; $ ($ content) * } } ; (mod $ name : ident { $ ($ content : tt) * }) => { compile_error ! (concat ! ("MOD|" , module_path ! () , "|" , stringify ! ($ name))) ; mod $ name { const MODULE_NAME : & str = stringify ! ($ name) ; $ ($ content) * } } ; }}
mkitem!{# [macro_export] macro_rules ! mkuse { ($ use_stmt : item) => { compile_error ! (concat ! ("USE|" , module_path ! () , "|" , stringify ! ($ use_stmt))) ; } ; }}
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
mkuse!{use rustc_data_structures :: fx :: { FxHashMap , FxHashSet , FxIndexSet } ;}
mkuse!{use rustc_data_structures :: unord :: { UnordMap , UnordSet } ;}
mkuse!{use rustc_hir :: attrs :: InstructionSetAttr ;}
mkuse!{use rustc_hir :: def :: DefKind ;}
mkuse!{use rustc_hir :: def_id :: { DefId , LOCAL_CRATE , LocalDefId } ;}
mkuse!{use rustc_middle :: middle :: codegen_fn_attrs :: { TargetFeature , TargetFeatureKind } ;}
mkuse!{use rustc_middle :: query :: Providers ;}
mkuse!{use rustc_middle :: ty :: TyCtxt ;}
mkuse!{use rustc_session :: Session ;}
mkuse!{use rustc_session :: lint :: builtin :: AARCH64_SOFTFLOAT_NEON ;}
mkuse!{use rustc_session :: parse :: feature_err ;}
mkuse!{use rustc_span :: { Span , Symbol , sym } ;}
mkuse!{use rustc_target :: target_features :: { RUSTC_SPECIFIC_FEATURES , Stability } ;}
mkuse!{use smallvec :: SmallVec ;}
mkuse!{use crate :: errors :: FeatureNotValid ;}
mkuse!{use crate :: { errors , target_features } ;}

macro_rules! from_target_feature_attr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function from_target_feature_attr in module {}", module_path!());
    };
}

mkfn!{
    from_target_feature_attr_introspect!();
    # [doc = " Compute the enabled target features from the `#[target_feature]` function attribute."] # [doc = " Enabled target features are added to `target_features`."] pub (crate) fn from_target_feature_attr (tcx : TyCtxt < '_ > , did : LocalDefId , features : & [(Symbol , Span)] , was_forced : bool , rust_target_features : & UnordMap < String , target_features :: Stability > , target_features : & mut Vec < TargetFeature > ,) { let rust_features = tcx . features () ; let abi_feature_constraints = tcx . sess . target . abi_required_features () ; for & (feature , feature_span) in features { let feature_str = feature . as_str () ; let Some (stability) = rust_target_features . get (feature_str) else { let plus_hint = feature_str . strip_prefix ('+') . is_some_and (| stripped | rust_target_features . contains_key (stripped)) ; tcx . dcx () . emit_err (FeatureNotValid { feature : feature_str , span : feature_span , plus_hint , }) ; continue ; } ; if let Err (reason) = stability . toggle_allowed () { tcx . dcx () . emit_err (errors :: ForbiddenTargetFeatureAttr { span : feature_span , feature : feature_str , reason , }) ; } else if let Some (nightly_feature) = stability . requires_nightly () && ! rust_features . enabled (nightly_feature) { feature_err (& tcx . sess , nightly_feature , feature_span , format ! ("the target feature `{feature}` is currently unstable") ,) . emit () ; } else { for & name in tcx . implied_target_features (feature) { if ! tcx . sess . opts . actually_rustdoc { if abi_feature_constraints . incompatible . contains (& name . as_str ()) { if tcx . sess . target . arch == "aarch64" && name . as_str () == "neon" { tcx . emit_node_span_lint (AARCH64_SOFTFLOAT_NEON , tcx . local_def_id_to_hir_id (did) , feature_span , errors :: Aarch64SoftfloatNeon ,) ; } else { tcx . dcx () . emit_err (errors :: ForbiddenTargetFeatureAttr { span : feature_span , feature : name . as_str () , reason : "this feature is incompatible with the target ABI" , }) ; } } } let kind = if name != feature { TargetFeatureKind :: Implied } else if was_forced { TargetFeatureKind :: Forced } else { TargetFeatureKind :: Enabled } ; target_features . push (TargetFeature { name , kind }) } } } }
}

macro_rules! asm_target_features_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function asm_target_features in module {}", module_path!());
    };
}

mkfn!{
    asm_target_features_introspect!();
    # [doc = " Computes the set of target features used in a function for the purposes of"] # [doc = " inline assembly."] fn asm_target_features (tcx : TyCtxt < '_ > , did : DefId) -> & FxIndexSet < Symbol > { let mut target_features = tcx . sess . unstable_target_features . clone () ; if tcx . def_kind (did) . has_codegen_attrs () { let attrs = tcx . codegen_fn_attrs (did) ; target_features . extend (attrs . target_features . iter () . map (| feature | feature . name)) ; match attrs . instruction_set { None => { } Some (InstructionSetAttr :: ArmA32) => { target_features . swap_remove (& sym :: thumb_mode) ; } Some (InstructionSetAttr :: ArmT32) => { target_features . insert (sym :: thumb_mode) ; } } } tcx . arena . alloc (target_features) }
}

macro_rules! check_target_feature_trait_unsafe_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_target_feature_trait_unsafe in module {}", module_path!());
    };
}

mkfn!{
    check_target_feature_trait_unsafe_introspect!();
    # [doc = " Checks the function annotated with `#[target_feature]` is not a safe"] # [doc = " trait method implementation, reporting an error if it is."] pub (crate) fn check_target_feature_trait_unsafe (tcx : TyCtxt < '_ > , id : LocalDefId , attr_span : Span) { if let DefKind :: AssocFn = tcx . def_kind (id) { let parent_id = tcx . local_parent (id) ; if let DefKind :: Trait | DefKind :: Impl { of_trait : true } = tcx . def_kind (parent_id) { tcx . dcx () . emit_err (errors :: TargetFeatureSafeTrait { span : attr_span , def : tcx . def_span (id) , }) ; } } }
}

macro_rules! parse_rust_feature_flag_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_rust_feature_flag in module {}", module_path!());
    };
}

mkfn!{
    parse_rust_feature_flag_introspect!();
    # [doc = " Parse the value of `-Ctarget-feature`, also expanding implied features,"] # [doc = " and call the closure for each (expanded) Rust feature. If the list contains"] # [doc = " a syntactically invalid item (not starting with `+`/`-`), the error callback is invoked."] fn parse_rust_feature_flag < 'a > (sess : & 'a Session , err_callback : impl Fn (& 'a str) , mut callback : impl FnMut (& 'a str , FxHashSet < & 'a str > , bool ,) ,) { let mut inverse_implied_features : Option < FxHashMap < & str , FxHashSet < & str > > > = None ; for feature in sess . opts . cg . target_feature . split (',') { if let Some (base_feature) = feature . strip_prefix ('+') { if RUSTC_SPECIFIC_FEATURES . contains (& base_feature) { continue ; } callback (base_feature , sess . target . implied_target_features (base_feature) , true) } else if let Some (base_feature) = feature . strip_prefix ('-') { if RUSTC_SPECIFIC_FEATURES . contains (& base_feature) { continue ; } let inverse_implied_features = inverse_implied_features . get_or_insert_with (| | { let mut set : FxHashMap < & str , FxHashSet < & str > > = FxHashMap :: default () ; for (f , _ , is) in sess . target . rust_target_features () { for i in is . iter () { set . entry (i) . or_default () . insert (f) ; } } set }) ; let mut features = FxHashSet :: default () ; let mut new_features = vec ! [base_feature] ; while let Some (new_feature) = new_features . pop () { if features . insert (new_feature) { if let Some (implied_features) = inverse_implied_features . get (& new_feature) { # [allow (rustc :: potential_query_instability)] new_features . extend (implied_features) } } } callback (base_feature , features , false) } else if ! feature . is_empty () { err_callback (feature) } } }
}

macro_rules! cfg_target_feature_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function cfg_target_feature in module {}", module_path!());
    };
}

mkfn!{
    cfg_target_feature_introspect!();
    # [doc = " Utility function for a codegen backend to compute `cfg(target_feature)`, or more specifically,"] # [doc = " to populate `sess.unstable_target_features` and `sess.target_features` (these are the first and"] # [doc = " 2nd component of the return value, respectively)."] # [doc = ""] # [doc = " `target_base_has_feature` should check whether the given feature (a Rust feature name!) is"] # [doc = " enabled in the \"base\" target machine, i.e., without applying `-Ctarget-feature`. Note that LLVM"] # [doc = " may consider features to be implied that we do not and vice-versa. We want `cfg` to be entirely"] # [doc = " consistent with Rust feature implications, and thus only consult LLVM to expand the target CPU"] # [doc = " to target features."] # [doc = ""] # [doc = " We do not have to worry about RUSTC_SPECIFIC_FEATURES here, those are handled elsewhere."] pub fn cfg_target_feature (sess : & Session , mut target_base_has_feature : impl FnMut (& str) -> bool ,) -> (Vec < Symbol > , Vec < Symbol >) { let mut features : UnordSet < Symbol > = sess . target . rust_target_features () . iter () . filter (| (feature , _ , _) | target_base_has_feature (feature)) . flat_map (| (base_feature , _ , _) | { # [allow (rustc :: potential_query_instability)] sess . target . implied_target_features (base_feature) . into_iter () . map (| f | Symbol :: intern (f)) }) . collect () ; parse_rust_feature_flag (sess , | _ | { } , | _base_feature , new_features , enabled | { # [allow (rustc :: potential_query_instability)] if enabled { features . extend (new_features . into_iter () . map (| f | Symbol :: intern (f))) ; } else { for new in new_features { features . remove (& Symbol :: intern (new)) ; } } } ,) ; let f = | allow_unstable | { sess . target . rust_target_features () . iter () . filter_map (| (feature , gate , _) | { if allow_unstable || (gate . in_cfg () && (sess . is_nightly_build () || gate . requires_nightly () . is_none ())) { Some (Symbol :: intern (feature)) } else { None } }) . filter (| feature | features . contains (& feature)) . collect () } ; (f (true) , f (false)) }
}

macro_rules! check_tied_features_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_tied_features in module {}", module_path!());
    };
}

mkfn!{
    check_tied_features_introspect!();
    # [doc = " Given a map from target_features to whether they are enabled or disabled, ensure only valid"] # [doc = " combinations are allowed."] pub fn check_tied_features (sess : & Session , features : & FxHashMap < & str , bool > ,) -> Option < & 'static [& 'static str] > { if ! features . is_empty () { for tied in sess . target . tied_target_features () { let mut tied_iter = tied . iter () ; let enabled = features . get (tied_iter . next () . unwrap ()) ; if tied_iter . any (| f | enabled != features . get (f)) { return Some (tied) ; } } } None }
}

macro_rules! flag_to_backend_features_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function flag_to_backend_features in module {}", module_path!());
    };
}

mkfn!{
    flag_to_backend_features_introspect!();
    # [doc = " Translates the `-Ctarget-feature` flag into a backend target feature list."] # [doc = ""] # [doc = " `to_backend_features` converts a Rust feature name into a list of backend feature names; this is"] # [doc = " used for diagnostic purposes only."] # [doc = ""] # [doc = " `extend_backend_features` extends the set of backend features (assumed to be in mutable state"] # [doc = " accessible by that closure) to enable/disable the given Rust feature name."] pub fn flag_to_backend_features < 'a , const N : usize > (sess : & 'a Session , diagnostics : bool , to_backend_features : impl Fn (& 'a str) -> SmallVec < [& 'a str ; N] > , mut extend_backend_features : impl FnMut (& 'a str , bool) ,) { let known_features = sess . target . rust_target_features () ; let mut rust_features = vec ! [] ; parse_rust_feature_flag (sess , | feature | { if diagnostics { sess . dcx () . emit_warn (errors :: UnknownCTargetFeaturePrefix { feature }) ; } } , | base_feature , new_features , enable | { rust_features . extend (UnordSet :: from (new_features) . to_sorted_stable_ord () . iter () . map (| & & s | (enable , s)) ,) ; if diagnostics { let feature_state = known_features . iter () . find (| & & (v , _ , _) | v == base_feature) ; match feature_state { None => { let rust_feature = known_features . iter () . find_map (| & (rust_feature , _ , _) | { let backend_features = to_backend_features (rust_feature) ; if backend_features . contains (& base_feature) && ! backend_features . contains (& rust_feature) { Some (rust_feature) } else { None } }) ; let unknown_feature = if let Some (rust_feature) = rust_feature { errors :: UnknownCTargetFeature { feature : base_feature , rust_feature : errors :: PossibleFeature :: Some { rust_feature } , } } else { errors :: UnknownCTargetFeature { feature : base_feature , rust_feature : errors :: PossibleFeature :: None , } } ; sess . dcx () . emit_warn (unknown_feature) ; } Some ((_ , stability , _)) => { if let Err (reason) = stability . toggle_allowed () { sess . dcx () . emit_warn (errors :: ForbiddenCTargetFeature { feature : base_feature , enabled : if enable { "enabled" } else { "disabled" } , reason , }) ; } else if stability . requires_nightly () . is_some () { sess . dcx () . emit_warn (errors :: UnstableCTargetFeature { feature : base_feature , }) ; } } } } } ,) ; if diagnostics { if let Some (f) = check_tied_features (sess , & FxHashMap :: from_iter (rust_features . iter () . map (| & (enable , feature) | (feature , enable))) ,) { sess . dcx () . emit_err (errors :: TargetFeatureDisableOrEnable { features : f , span : None , missing_features : None , }) ; } } for (enable , feature) in rust_features { extend_backend_features (feature , enable) ; } }
}

macro_rules! retpoline_features_by_flags_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function retpoline_features_by_flags in module {}", module_path!());
    };
}

mkfn!{
    retpoline_features_by_flags_introspect!();
    # [doc = " Computes the backend target features to be added to account for retpoline flags."] # [doc = " Used by both LLVM and GCC since their target features are, conveniently, the same."] pub fn retpoline_features_by_flags (sess : & Session , features : & mut Vec < String >) { let unstable_opts = & sess . opts . unstable_opts ; if unstable_opts . retpoline && ! unstable_opts . retpoline_external_thunk { features . push ("+retpoline-indirect-branches" . into ()) ; features . push ("+retpoline-indirect-calls" . into ()) ; } if unstable_opts . retpoline_external_thunk { features . push ("+retpoline-external-thunk" . into ()) ; features . push ("+retpoline-indirect-branches" . into ()) ; features . push ("+retpoline-indirect-calls" . into ()) ; } }
}

macro_rules! provide_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function provide in module {}", module_path!());
    };
}

mkfn!{
    provide_introspect!();
    pub (crate) fn provide (providers : & mut Providers) { * providers = Providers { rust_target_features : | tcx , cnum | { assert_eq ! (cnum , LOCAL_CRATE) ; if tcx . sess . opts . actually_rustdoc { let mut result : UnordMap < String , Stability > = Default :: default () ; for (name , stability) in rustc_target :: target_features :: all_rust_features () { use std :: collections :: hash_map :: Entry ; match result . entry (name . to_owned ()) { Entry :: Vacant (vacant_entry) => { vacant_entry . insert (stability) ; } Entry :: Occupied (mut occupied_entry) => { match (occupied_entry . get () , stability) { (Stability :: Stable , _) | (Stability :: Unstable { .. } , Stability :: Unstable { .. } | Stability :: Forbidden { .. } ,) | (Stability :: Forbidden { .. } , Stability :: Forbidden { .. }) => { } _ => { occupied_entry . insert (stability) ; } } } } } result } else { tcx . sess . target . rust_target_features () . iter () . map (| (a , b , _) | (a . to_string () , * b)) . collect () } } , implied_target_features : | tcx , feature : Symbol | { let feature = feature . as_str () ; UnordSet :: from (tcx . sess . target . implied_target_features (feature)) . into_sorted_stable_ord () . into_iter () . map (| s | Symbol :: intern (s)) . collect () } , asm_target_features , .. * providers } }
}