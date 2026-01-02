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
mkuse!{use rustc_errors :: ErrorGuaranteed ;}
mkuse!{use rustc_hir :: LangItem ;}
mkuse!{use rustc_hir :: def_id :: DefId ;}
mkuse!{use rustc_infer :: infer :: TyCtxtInferExt ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: query :: Providers ;}
mkuse!{use rustc_middle :: traits :: { BuiltinImplSource , CodegenObligationError } ;}
mkuse!{use rustc_middle :: ty :: { self , ClosureKind , GenericArgsRef , Instance , PseudoCanonicalInput , TyCtxt , TypeVisitableExt , } ;}
mkuse!{use rustc_span :: sym ;}
mkuse!{use rustc_trait_selection :: traits ;}
mkuse!{use tracing :: debug ;}
mkuse!{use traits :: translate_args ;}
mkuse!{use crate :: errors :: UnexpectedFnPtrAssociatedItem ;}

macro_rules! resolve_instance_raw_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function resolve_instance_raw in module {}", module_path!());
    };
}

mkfn!{
    resolve_instance_raw_introspect!();
    fn resolve_instance_raw < 'tcx > (tcx : TyCtxt < 'tcx > , key : ty :: PseudoCanonicalInput < 'tcx , (DefId , GenericArgsRef < 'tcx >) > ,) -> Result < Option < Instance < 'tcx > > , ErrorGuaranteed > { let PseudoCanonicalInput { typing_env , value : (def_id , args) } = key ; let result = if let Some (trait_def_id) = tcx . trait_of_assoc (def_id) { debug ! (" => associated item, attempting to find impl in typing_env {:#?}" , typing_env) ; resolve_associated_item (tcx , def_id , typing_env , trait_def_id , tcx . normalize_erasing_regions (typing_env , args) ,) } else { let def = if tcx . intrinsic (def_id) . is_some () { debug ! (" => intrinsic") ; ty :: InstanceKind :: Intrinsic (def_id) } else if tcx . is_lang_item (def_id , LangItem :: DropInPlace) { let ty = args . type_at (0) ; if ty . needs_drop (tcx , typing_env) { debug ! (" => nontrivial drop glue") ; match * ty . kind () { ty :: Coroutine (coroutine_def_id , ..) => { if tcx . optimized_mir (coroutine_def_id) . coroutine_drop_async () . is_some () { ty :: InstanceKind :: DropGlue (def_id , None) } else { ty :: InstanceKind :: DropGlue (def_id , Some (ty)) } } ty :: Closure (..) | ty :: CoroutineClosure (..) | ty :: Tuple (..) | ty :: Adt (..) | ty :: Dynamic (..) | ty :: Array (..) | ty :: Slice (..) | ty :: UnsafeBinder (..) => ty :: InstanceKind :: DropGlue (def_id , Some (ty)) , _ => return Ok (None) , } } else { debug ! (" => trivial drop glue") ; ty :: InstanceKind :: DropGlue (def_id , None) } } else if tcx . is_lang_item (def_id , LangItem :: AsyncDropInPlace) { let ty = args . type_at (0) ; if ty . needs_async_drop (tcx , typing_env) { match * ty . kind () { ty :: Closure (..) | ty :: CoroutineClosure (..) | ty :: Coroutine (..) | ty :: Tuple (..) | ty :: Adt (..) | ty :: Dynamic (..) | ty :: Array (..) | ty :: Slice (..) => { } _ => return Ok (None) , } debug ! (" => nontrivial async drop glue ctor") ; ty :: InstanceKind :: AsyncDropGlueCtorShim (def_id , ty) } else { debug ! (" => trivial async drop glue ctor") ; ty :: InstanceKind :: AsyncDropGlueCtorShim (def_id , ty) } } else if tcx . is_async_drop_in_place_coroutine (def_id) { let ty = args . type_at (0) ; ty :: InstanceKind :: AsyncDropGlue (def_id , ty) } else { debug ! (" => free item") ; ty :: InstanceKind :: Item (def_id) } ; Ok (Some (Instance { def , args })) } ; debug ! ("resolve_instance: result={:?}" , result) ; result }
}

macro_rules! resolve_associated_item_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function resolve_associated_item in module {}", module_path!());
    };
}

mkfn!{
    resolve_associated_item_introspect!();
    fn resolve_associated_item < 'tcx > (tcx : TyCtxt < 'tcx > , trait_item_id : DefId , typing_env : ty :: TypingEnv < 'tcx > , trait_id : DefId , rcvr_args : GenericArgsRef < 'tcx > ,) -> Result < Option < Instance < 'tcx > > , ErrorGuaranteed > { debug ! (? trait_item_id , ? typing_env , ? trait_id , ? rcvr_args , "resolve_associated_item") ; let trait_ref = ty :: TraitRef :: from_assoc (tcx , trait_id , rcvr_args) ; let input = typing_env . as_query_input (trait_ref) ; let vtbl = match tcx . codegen_select_candidate (input) { Ok (vtbl) => vtbl , Err (CodegenObligationError :: Ambiguity | CodegenObligationError :: Unimplemented) => { return Ok (None) ; } Err (CodegenObligationError :: UnconstrainedParam (guar)) => return Err (guar) , } ; Ok (match vtbl { traits :: ImplSource :: UserDefined (impl_data) => { debug ! ("resolving ImplSource::UserDefined: {:?}, {:?}, {:?}, {:?}" , typing_env , trait_item_id , rcvr_args , impl_data) ; assert ! (! rcvr_args . has_infer ()) ; assert ! (! trait_ref . has_infer ()) ; let trait_def_id = tcx . trait_id_of_impl (impl_data . impl_def_id) . unwrap () ; let trait_def = tcx . trait_def (trait_def_id) ; let leaf_def = trait_def . ancestors (tcx , impl_data . impl_def_id) ? . leaf_def (tcx , trait_item_id) . unwrap_or_else (| | { bug ! ("{:?} not found in {:?}" , trait_item_id , impl_data . impl_def_id) ; }) ; let eligible = if leaf_def . is_final () { true } else { match typing_env . typing_mode { ty :: TypingMode :: Coherence | ty :: TypingMode :: Analysis { .. } | ty :: TypingMode :: Borrowck { .. } | ty :: TypingMode :: PostBorrowckAnalysis { .. } => false , ty :: TypingMode :: PostAnalysis => ! trait_ref . still_further_specializable () , } } ; if ! eligible { return Ok (None) ; } let typing_env = typing_env . with_post_analysis_normalized (tcx) ; let (infcx , param_env) = tcx . infer_ctxt () . build_with_typing_env (typing_env) ; let args = rcvr_args . rebase_onto (tcx , trait_def_id , impl_data . args) ; let args = translate_args (& infcx , param_env , impl_data . impl_def_id , args , leaf_def . defining_node ,) ; let args = infcx . tcx . erase_and_anonymize_regions (args) ; let self_ty = rcvr_args . type_at (0) ; if ! self_ty . is_known_rigid () { let predicates = tcx . predicates_of (impl_data . impl_def_id) . instantiate (tcx , impl_data . args) . predicates ; let sized_def_id = tcx . lang_items () . sized_trait () ; if ! predicates . into_iter () . filter_map (ty :: Clause :: as_trait_clause) . any (| clause | { Some (clause . def_id ()) == sized_def_id && clause . skip_binder () . self_ty () == self_ty }) { return Ok (None) ; } } if ! leaf_def . item . defaultness (tcx) . has_value () { let guar = tcx . dcx () . span_delayed_bug (tcx . def_span (leaf_def . item . def_id) , "missing value for assoc item in impl" ,) ; return Err (guar) ; } if ! tcx . check_args_compatible (leaf_def . item . def_id , args) { let guar = tcx . dcx () . span_delayed_bug (tcx . def_span (leaf_def . item . def_id) , "missing value for assoc item in impl" ,) ; return Err (guar) ; } let args = tcx . erase_and_anonymize_regions (args) ; if trait_item_id != leaf_def . item . def_id && let Some (leaf_def_item) = leaf_def . item . def_id . as_local () { tcx . ensure_ok () . compare_impl_item (leaf_def_item) ? ; } Some (ty :: Instance :: new_raw (leaf_def . item . def_id , args)) } traits :: ImplSource :: Builtin (BuiltinImplSource :: Object (_) , _) => { let trait_ref = ty :: TraitRef :: from_assoc (tcx , trait_id , rcvr_args) ; if trait_ref . has_non_region_infer () || trait_ref . has_non_region_param () { None } else { let vtable_base = tcx . first_method_vtable_slot (trait_ref) ; let offset = tcx . own_existential_vtable_entries (trait_id) . iter () . copied () . position (| def_id | def_id == trait_item_id) ; offset . map (| offset | Instance { def : ty :: InstanceKind :: Virtual (trait_item_id , vtable_base + offset) , args : rcvr_args , }) } } traits :: ImplSource :: Builtin (BuiltinImplSource :: Misc | BuiltinImplSource :: Trivial , _) => { if tcx . is_lang_item (trait_ref . def_id , LangItem :: Clone) { let name = tcx . item_name (trait_item_id) ; if name == sym :: clone { let self_ty = trait_ref . self_ty () ; match self_ty . kind () { ty :: FnDef (..) | ty :: FnPtr (..) => () , ty :: Coroutine (..) | ty :: CoroutineWitness (..) | ty :: Closure (..) | ty :: CoroutineClosure (..) | ty :: Tuple (..) => { } _ => return Ok (None) , } ; Some (Instance { def : ty :: InstanceKind :: CloneShim (trait_item_id , self_ty) , args : rcvr_args , }) } else { assert_eq ! (name , sym :: clone_from) ; let args = tcx . erase_and_anonymize_regions (rcvr_args) ; Some (ty :: Instance :: new_raw (trait_item_id , args)) } } else if tcx . is_lang_item (trait_ref . def_id , LangItem :: FnPtrTrait) { if tcx . is_lang_item (trait_item_id , LangItem :: FnPtrAddr) { let self_ty = trait_ref . self_ty () ; if ! matches ! (self_ty . kind () , ty :: FnPtr (..)) { return Ok (None) ; } Some (Instance { def : ty :: InstanceKind :: FnPtrAddrShim (trait_item_id , self_ty) , args : rcvr_args , }) } else { tcx . dcx () . emit_fatal (UnexpectedFnPtrAssociatedItem { span : tcx . def_span (trait_item_id) , }) } } else if let Some (target_kind) = tcx . fn_trait_kind_from_def_id (trait_ref . def_id) { if cfg ! (debug_assertions) && ! [sym :: call , sym :: call_mut , sym :: call_once] . contains (& tcx . item_name (trait_item_id)) { bug ! ("no definition for `{trait_ref}::{}` for built-in callable type" , tcx . item_name (trait_item_id)) } match * rcvr_args . type_at (0) . kind () { ty :: Closure (closure_def_id , args) => { Some (Instance :: resolve_closure (tcx , closure_def_id , args , target_kind)) } ty :: FnDef (..) | ty :: FnPtr (..) => Some (Instance { def : ty :: InstanceKind :: FnPtrShim (trait_item_id , rcvr_args . type_at (0)) , args : rcvr_args , }) , ty :: CoroutineClosure (coroutine_closure_def_id , args) => { if ty :: ClosureKind :: FnOnce == args . as_coroutine_closure () . kind () { Some (Instance :: new_raw (coroutine_closure_def_id , args)) } else { Some (Instance { def : ty :: InstanceKind :: ConstructCoroutineInClosureShim { coroutine_closure_def_id , receiver_by_ref : target_kind != ty :: ClosureKind :: FnOnce , } , args , }) } } _ => bug ! ("no built-in definition for `{trait_ref}::{}` for non-fn type" , tcx . item_name (trait_item_id)) , } } else if let Some (target_kind) = tcx . async_fn_trait_kind_from_def_id (trait_ref . def_id) { match * rcvr_args . type_at (0) . kind () { ty :: CoroutineClosure (coroutine_closure_def_id , args) => { if target_kind == ClosureKind :: FnOnce && args . as_coroutine_closure () . kind () != ClosureKind :: FnOnce { Some (Instance { def : ty :: InstanceKind :: ConstructCoroutineInClosureShim { coroutine_closure_def_id , receiver_by_ref : false , } , args , }) } else { Some (Instance :: new_raw (coroutine_closure_def_id , args)) } } ty :: Closure (closure_def_id , args) => { Some (Instance :: resolve_closure (tcx , closure_def_id , args , target_kind)) } ty :: FnDef (..) | ty :: FnPtr (..) => Some (Instance { def : ty :: InstanceKind :: FnPtrShim (trait_item_id , rcvr_args . type_at (0)) , args : rcvr_args , }) , _ => bug ! ("no built-in definition for `{trait_ref}::{}` for non-lending-closure type" , tcx . item_name (trait_item_id)) , } } else if tcx . is_lang_item (trait_ref . def_id , LangItem :: TransmuteTrait) { let name = tcx . item_name (trait_item_id) ; assert_eq ! (name , sym :: transmute) ; let args = tcx . erase_and_anonymize_regions (rcvr_args) ; Some (ty :: Instance :: new_raw (trait_item_id , args)) } else { Instance :: try_resolve_item_for_coroutine (tcx , trait_item_id , trait_id , rcvr_args) } } traits :: ImplSource :: Param (..) | traits :: ImplSource :: Builtin (BuiltinImplSource :: TraitUpcasting { .. } , _) => None , }) }
}

macro_rules! provide_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function provide in module {}", module_path!());
    };
}

mkfn!{
    provide_introspect!();
    pub (crate) fn provide (providers : & mut Providers) { * providers = Providers { resolve_instance_raw , .. * providers } ; }
}