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
mkuse!{use rustc_hir :: def_id :: DefId ;}
mkuse!{use rustc_hir :: lang_items :: LangItem ;}
mkuse!{use rustc_hir :: { CoroutineDesugaring , CoroutineKind , CoroutineSource , Safety } ;}
mkuse!{use rustc_index :: { Idx , IndexVec } ;}
mkuse!{use rustc_middle :: mir :: { BasicBlock , BasicBlockData , Body , Local , LocalDecl , MirSource , Operand , Place , Rvalue , SourceInfo , Statement , StatementKind , Terminator , TerminatorKind , } ;}
mkuse!{use rustc_middle :: ty :: { self , EarlyBinder , Ty , TyCtxt , TypeVisitableExt } ;}
mkuse!{use super :: * ;}
mkuse!{use crate :: patch :: MirPatch ;}

macro_rules! build_async_destructor_ctor_shim_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_async_destructor_ctor_shim in module {}", module_path!());
    };
}

mkfn!{
    build_async_destructor_ctor_shim_introspect!();
    pub (super) fn build_async_destructor_ctor_shim < 'tcx > (tcx : TyCtxt < 'tcx > , def_id : DefId , ty : Ty < 'tcx > ,) -> Body < 'tcx > { debug ! ("build_async_destructor_ctor_shim(def_id={:?}, ty={:?})" , def_id , ty) ; debug_assert_eq ! (Some (def_id) , tcx . lang_items () . async_drop_in_place_fn ()) ; let generic_body = tcx . optimized_mir (def_id) ; let args = tcx . mk_args (& [ty . into ()]) ; let mut body = EarlyBinder :: bind (generic_body . clone ()) . instantiate (tcx , args) ; pm :: run_passes (tcx , & mut body , & [& simplify :: SimplifyCfg :: MakeShim , & abort_unwinding_calls :: AbortUnwindingCalls , & add_call_guards :: CriticalCallEdges ,] , None , pm :: Optimizations :: Allowed ,) ; body }
}

macro_rules! build_async_drop_shim_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_async_drop_shim in module {}", module_path!());
    };
}

mkfn!{
    build_async_drop_shim_introspect!();
    pub (super) fn build_async_drop_shim < 'tcx > (tcx : TyCtxt < 'tcx > , def_id : DefId , ty : Ty < 'tcx > ,) -> Body < 'tcx > { debug ! ("build_async_drop_shim(def_id={:?}, ty={:?})" , def_id , ty) ; let ty :: Coroutine (_ , parent_args) = ty . kind () else { bug ! () ; } ; let typing_env = ty :: TypingEnv :: fully_monomorphized () ; let drop_ty = parent_args . first () . unwrap () . expect_ty () ; let drop_ptr_ty = Ty :: new_mut_ptr (tcx , drop_ty) ; assert ! (tcx . is_coroutine (def_id)) ; let coroutine_kind = tcx . coroutine_kind (def_id) . unwrap () ; assert ! (matches ! (coroutine_kind , CoroutineKind :: Desugared (CoroutineDesugaring :: Async , CoroutineSource :: Fn))) ; let needs_async_drop = drop_ty . needs_async_drop (tcx , typing_env) ; let needs_sync_drop = ! needs_async_drop && drop_ty . needs_drop (tcx , typing_env) ; let resume_adt = tcx . adt_def (tcx . require_lang_item (LangItem :: ResumeTy , DUMMY_SP)) ; let resume_ty = Ty :: new_adt (tcx , resume_adt , ty :: List :: empty ()) ; let fn_sig = ty :: Binder :: dummy (tcx . mk_fn_sig ([ty , resume_ty] , tcx . types . unit , false , Safety :: Safe , ExternAbi :: Rust ,)) ; let sig = tcx . instantiate_bound_regions_with_erased (fn_sig) ; assert ! (! drop_ty . is_coroutine ()) ; let span = tcx . def_span (def_id) ; let source_info = SourceInfo :: outermost (span) ; let coroutine_layout = Place :: from (Local :: new (1 + 0)) ; let coroutine_layout_dropee = tcx . mk_place_field (coroutine_layout , FieldIdx :: new (0) , drop_ptr_ty) ; let return_block = BasicBlock :: new (1) ; let mut blocks = IndexVec :: with_capacity (2) ; let block = | blocks : & mut IndexVec < _ , _ > , kind | { blocks . push (BasicBlockData :: new (Some (Terminator { source_info , kind }) , false)) } ; block (& mut blocks , if needs_sync_drop { TerminatorKind :: Drop { place : tcx . mk_place_deref (coroutine_layout_dropee) , target : return_block , unwind : UnwindAction :: Continue , replace : false , drop : None , async_fut : None , } } else { TerminatorKind :: Goto { target : return_block } } ,) ; block (& mut blocks , TerminatorKind :: Return) ; let source = MirSource :: from_instance (ty :: InstanceKind :: AsyncDropGlue (def_id , ty)) ; let mut body = new_body (source , blocks , local_decls_for_sig (& sig , span) , sig . inputs () . len () , span) ; body . coroutine = Some (Box :: new (CoroutineInfo :: initial (coroutine_kind , parent_args . as_coroutine () . yield_ty () , parent_args . as_coroutine () . resume_ty () ,))) ; body . phase = MirPhase :: Runtime (RuntimePhase :: Initial) ; if ! needs_async_drop || drop_ty . references_error () { return body ; } let mut dropee_ptr = Place :: from (body . local_decls . push (LocalDecl :: new (drop_ptr_ty , span))) ; let st_kind = StatementKind :: Assign (Box :: new ((dropee_ptr , Rvalue :: Use (Operand :: Move (coroutine_layout_dropee)) ,))) ; body . basic_blocks_mut () [START_BLOCK] . statements . push (Statement :: new (source_info , st_kind)) ; dropee_ptr = dropee_emit_retag (tcx , & mut body , dropee_ptr , span) ; let dropline = body . basic_blocks . last_index () ; let patch = { let mut elaborator = DropShimElaborator { body : & body , patch : MirPatch :: new (& body) , tcx , typing_env , produce_async_drops : true , } ; let dropee = tcx . mk_place_deref (dropee_ptr) ; let resume_block = elaborator . patch . resume_block () ; elaborate_drop (& mut elaborator , source_info , dropee , () , return_block , Unwind :: To (resume_block) , START_BLOCK , dropline ,) ; elaborator . patch } ; patch . apply (& mut body) ; body }
}

macro_rules! build_future_drop_poll_shim_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_future_drop_poll_shim in module {}", module_path!());
    };
}

mkfn!{
    build_future_drop_poll_shim_introspect!();
    pub (super) fn build_future_drop_poll_shim < 'tcx > (tcx : TyCtxt < 'tcx > , def_id : DefId , proxy_ty : Ty < 'tcx > , impl_ty : Ty < 'tcx > ,) -> Body < 'tcx > { let instance = ty :: InstanceKind :: FutureDropPollShim (def_id , proxy_ty , impl_ty) ; let ty :: Coroutine (coroutine_def_id , _) = impl_ty . kind () else { bug ! ("build_future_drop_poll_shim not for coroutine impl type: ({:?})" , instance) ; } ; let span = tcx . def_span (def_id) ; if tcx . is_async_drop_in_place_coroutine (* coroutine_def_id) { build_adrop_for_adrop_shim (tcx , proxy_ty , impl_ty , span , instance) } else { build_adrop_for_coroutine_shim (tcx , proxy_ty , impl_ty , span , instance) } }
}

macro_rules! build_adrop_for_coroutine_shim_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_adrop_for_coroutine_shim in module {}", module_path!());
    };
}

mkfn!{
    build_adrop_for_coroutine_shim_introspect!();
    fn build_adrop_for_coroutine_shim < 'tcx > (tcx : TyCtxt < 'tcx > , proxy_ty : Ty < 'tcx > , impl_ty : Ty < 'tcx > , span : Span , instance : ty :: InstanceKind < 'tcx > ,) -> Body < 'tcx > { let ty :: Coroutine (coroutine_def_id , impl_args) = impl_ty . kind () else { bug ! ("build_adrop_for_coroutine_shim not for coroutine impl type: ({:?})" , instance) ; } ; let proxy_ref = Ty :: new_mut_ref (tcx , tcx . lifetimes . re_erased , proxy_ty) ; let pin_proxy_layout_local = Local :: new (1) ; let source_info = SourceInfo :: outermost (span) ; let body = tcx . optimized_mir (* coroutine_def_id) . future_drop_poll () . unwrap () ; let mut body : Body < 'tcx > = EarlyBinder :: bind (body . clone ()) . instantiate (tcx , impl_args) ; body . source . instance = instance ; body . phase = MirPhase :: Runtime (RuntimePhase :: Initial) ; body . var_debug_info . clear () ; let pin_adt_ref = tcx . adt_def (tcx . require_lang_item (LangItem :: Pin , span)) ; let args = tcx . mk_args (& [proxy_ref . into ()]) ; let pin_proxy_ref = Ty :: new_adt (tcx , pin_adt_ref , args) ; let cor_ref = Ty :: new_mut_ref (tcx , tcx . lifetimes . re_erased , impl_ty) ; let proxy_ref_local = body . local_decls . push (LocalDecl :: new (proxy_ref , span)) ; let cor_ref_local = body . local_decls . push (LocalDecl :: new (cor_ref , span)) ; FixProxyFutureDropVisitor { tcx , replace_to : cor_ref_local } . visit_body (& mut body) ; body . local_decls [pin_proxy_layout_local] = LocalDecl :: new (pin_proxy_ref , span) ; { let mut idx : usize = 0 ; let proxy_ref_place = Place :: from (pin_proxy_layout_local) . project_deeper (& [PlaceElem :: Field (FieldIdx :: ZERO , proxy_ref)] , tcx) ; body . basic_blocks_mut () [START_BLOCK] . statements . insert (idx , Statement :: new (source_info , StatementKind :: Assign (Box :: new ((Place :: from (proxy_ref_local) , Rvalue :: CopyForDeref (proxy_ref_place) ,))) ,) ,) ; idx += 1 ; let mut cor_ptr_local = proxy_ref_local ; proxy_ty . find_async_drop_impl_coroutine (tcx , | ty | { if ty != proxy_ty { let ty_ptr = Ty :: new_mut_ptr (tcx , ty) ; let impl_ptr_place = Place :: from (cor_ptr_local) . project_deeper (& [PlaceElem :: Deref , PlaceElem :: Field (FieldIdx :: ZERO , ty_ptr)] , tcx ,) ; cor_ptr_local = body . local_decls . push (LocalDecl :: new (ty_ptr , span)) ; body . basic_blocks_mut () [START_BLOCK] . statements . insert (idx , Statement :: new (source_info , StatementKind :: Assign (Box :: new ((Place :: from (cor_ptr_local) , Rvalue :: CopyForDeref (impl_ptr_place) ,))) ,) ,) ; idx += 1 ; } }) ; let reborrow = Rvalue :: Ref (tcx . lifetimes . re_erased , BorrowKind :: Mut { kind : MutBorrowKind :: Default } , tcx . mk_place_deref (Place :: from (cor_ptr_local)) ,) ; body . basic_blocks_mut () [START_BLOCK] . statements . insert (idx , Statement :: new (source_info , StatementKind :: Assign (Box :: new ((Place :: from (cor_ref_local) , reborrow))) ,) ,) ; } body }
}

macro_rules! build_adrop_for_adrop_shim_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_adrop_for_adrop_shim in module {}", module_path!());
    };
}

mkfn!{
    build_adrop_for_adrop_shim_introspect!();
    fn build_adrop_for_adrop_shim < 'tcx > (tcx : TyCtxt < 'tcx > , proxy_ty : Ty < 'tcx > , impl_ty : Ty < 'tcx > , span : Span , instance : ty :: InstanceKind < 'tcx > ,) -> Body < 'tcx > { let source_info = SourceInfo :: outermost (span) ; let proxy_ref = Ty :: new_mut_ref (tcx , tcx . lifetimes . re_erased , proxy_ty) ; let pin_proxy_layout_local = Local :: new (1) ; let proxy_ref_place = Place :: from (pin_proxy_layout_local) . project_deeper (& [PlaceElem :: Field (FieldIdx :: ZERO , proxy_ref)] , tcx) ; let cor_ref = Ty :: new_mut_ref (tcx , tcx . lifetimes . re_erased , impl_ty) ; let poll_adt_ref = tcx . adt_def (tcx . require_lang_item (LangItem :: Poll , span)) ; let ret_ty = Ty :: new_adt (tcx , poll_adt_ref , tcx . mk_args (& [tcx . types . unit . into ()])) ; let pin_adt_ref = tcx . adt_def (tcx . require_lang_item (LangItem :: Pin , span)) ; let env_ty = Ty :: new_adt (tcx , pin_adt_ref , tcx . mk_args (& [proxy_ref . into ()])) ; let sig = tcx . mk_fn_sig ([env_ty , Ty :: new_task_context (tcx)] , ret_ty , false , hir :: Safety :: Safe , ExternAbi :: Rust ,) ; let mut locals = local_decls_for_sig (& sig , span) ; let mut blocks = IndexVec :: with_capacity (3) ; let proxy_ref_local = locals . push (LocalDecl :: new (proxy_ref , span)) ; let call_bb = BasicBlock :: new (1) ; let return_bb = BasicBlock :: new (2) ; let mut statements = Vec :: new () ; statements . push (Statement :: new (source_info , StatementKind :: Assign (Box :: new ((Place :: from (proxy_ref_local) , Rvalue :: CopyForDeref (proxy_ref_place) ,))) ,)) ; let mut cor_ptr_local = proxy_ref_local ; proxy_ty . find_async_drop_impl_coroutine (tcx , | ty | { if ty != proxy_ty { let ty_ptr = Ty :: new_mut_ptr (tcx , ty) ; let impl_ptr_place = Place :: from (cor_ptr_local) . project_deeper (& [PlaceElem :: Deref , PlaceElem :: Field (FieldIdx :: ZERO , ty_ptr)] , tcx) ; cor_ptr_local = locals . push (LocalDecl :: new (ty_ptr , span)) ; statements . push (Statement :: new (source_info , StatementKind :: Assign (Box :: new ((Place :: from (cor_ptr_local) , Rvalue :: CopyForDeref (impl_ptr_place) ,))) ,)) ; } }) ; let reborrow = Rvalue :: Ref (tcx . lifetimes . re_erased , BorrowKind :: Mut { kind : MutBorrowKind :: Default } , tcx . mk_place_deref (Place :: from (cor_ptr_local)) ,) ; let cor_ref_place = Place :: from (locals . push (LocalDecl :: new (cor_ref , span))) ; statements . push (Statement :: new (source_info , StatementKind :: Assign (Box :: new ((cor_ref_place , reborrow))) ,)) ; let cor_pin_ty = Ty :: new_adt (tcx , pin_adt_ref , tcx . mk_args (& [cor_ref . into ()])) ; let cor_pin_place = Place :: from (locals . push (LocalDecl :: new (cor_pin_ty , span))) ; let pin_fn = tcx . require_lang_item (LangItem :: PinNewUnchecked , span) ; blocks . push (BasicBlockData :: new_stmts (statements , Some (Terminator { source_info , kind : TerminatorKind :: Call { func : Operand :: function_handle (tcx , pin_fn , [cor_ref . into ()] , span) , args : [dummy_spanned (Operand :: Move (cor_ref_place))] . into () , destination : cor_pin_place , target : Some (call_bb) , unwind : UnwindAction :: Continue , call_source : CallSource :: Misc , fn_span : span , } , }) , false ,)) ; let poll_fn = tcx . require_lang_item (LangItem :: FuturePoll , span) ; let resume_ctx = Place :: from (Local :: new (2)) ; blocks . push (BasicBlockData :: new (Some (Terminator { source_info , kind : TerminatorKind :: Call { func : Operand :: function_handle (tcx , poll_fn , [impl_ty . into ()] , span) , args : [dummy_spanned (Operand :: Move (cor_pin_place)) , dummy_spanned (Operand :: Move (resume_ctx)) ,] . into () , destination : Place :: return_place () , target : Some (return_bb) , unwind : UnwindAction :: Continue , call_source : CallSource :: Misc , fn_span : span , } , }) , false ,)) ; blocks . push (BasicBlockData :: new (Some (Terminator { source_info , kind : TerminatorKind :: Return }) , false ,)) ; let source = MirSource :: from_instance (instance) ; let mut body = new_body (source , blocks , locals , sig . inputs () . len () , span) ; body . phase = MirPhase :: Runtime (RuntimePhase :: Initial) ; return body ; }
}