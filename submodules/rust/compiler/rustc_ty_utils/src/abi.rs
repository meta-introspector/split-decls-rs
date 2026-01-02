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
mkuse!{use std :: iter ;}
mkuse!{use rustc_abi :: Primitive :: Pointer ;}
mkuse!{use rustc_abi :: { BackendRepr , ExternAbi , PointerKind , Scalar , Size } ;}
mkuse!{use rustc_hir as hir ;}
mkuse!{use rustc_hir :: lang_items :: LangItem ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: query :: Providers ;}
mkuse!{use rustc_middle :: ty :: layout :: { FnAbiError , HasTyCtxt , HasTypingEnv , LayoutCx , LayoutOf , TyAndLayout , fn_can_unwind , } ;}
mkuse!{use rustc_middle :: ty :: { self , InstanceKind , Ty , TyCtxt } ;}
mkuse!{use rustc_session :: config :: OptLevel ;}
mkuse!{use rustc_span :: DUMMY_SP ;}
mkuse!{use rustc_span :: def_id :: DefId ;}
mkuse!{use rustc_target :: callconv :: { AbiMap , ArgAbi , ArgAttribute , ArgAttributes , ArgExtension , FnAbi , PassMode , } ;}
mkuse!{use tracing :: debug ;}

macro_rules! provide_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function provide in module {}", module_path!());
    };
}

mkfn!{
    provide_introspect!();
    pub (crate) fn provide (providers : & mut Providers) { * providers = Providers { fn_abi_of_fn_ptr , fn_abi_of_instance , .. * providers } ; }
}

macro_rules! fn_sig_for_fn_abi_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fn_sig_for_fn_abi in module {}", module_path!());
    };
}

mkfn!{
    fn_sig_for_fn_abi_introspect!();
    # [tracing :: instrument (level = "debug" , skip (tcx , typing_env))] fn fn_sig_for_fn_abi < 'tcx > (tcx : TyCtxt < 'tcx > , instance : ty :: Instance < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > ,) -> ty :: FnSig < 'tcx > { if let InstanceKind :: ThreadLocalShim (..) = instance . def { return tcx . mk_fn_sig ([] , tcx . thread_local_ptr_ty (instance . def_id ()) , false , hir :: Safety :: Safe , rustc_abi :: ExternAbi :: Rust ,) ; } let ty = instance . ty (tcx , typing_env) ; match * ty . kind () { ty :: FnDef (def_id , args) => { let mut sig = tcx . instantiate_bound_regions_with_erased (tcx . fn_sig (def_id) . instantiate (tcx , args)) ; if let ty :: InstanceKind :: VTableShim (..) = instance . def { let mut inputs_and_output = sig . inputs_and_output . to_vec () ; inputs_and_output [0] = Ty :: new_mut_ptr (tcx , inputs_and_output [0]) ; sig . inputs_and_output = tcx . mk_type_list (& inputs_and_output) ; } sig } ty :: Closure (def_id , args) => { let sig = tcx . instantiate_bound_regions_with_erased (args . as_closure () . sig ()) ; let env_ty = tcx . closure_env_ty (Ty :: new_closure (tcx , def_id , args) , args . as_closure () . kind () , tcx . lifetimes . re_erased ,) ; tcx . mk_fn_sig (iter :: once (env_ty) . chain (sig . inputs () . iter () . cloned ()) , sig . output () , sig . c_variadic , sig . safety , sig . abi ,) } ty :: CoroutineClosure (def_id , args) => { let coroutine_ty = Ty :: new_coroutine_closure (tcx , def_id , args) ; let sig = args . as_coroutine_closure () . coroutine_closure_sig () ; let mut coroutine_kind = args . as_coroutine_closure () . kind () ; let env_ty = if let InstanceKind :: ConstructCoroutineInClosureShim { receiver_by_ref , .. } = instance . def { coroutine_kind = ty :: ClosureKind :: FnOnce ; if receiver_by_ref { Ty :: new_imm_ref (tcx , tcx . lifetimes . re_erased , coroutine_ty) } else { coroutine_ty } } else { tcx . closure_env_ty (coroutine_ty , coroutine_kind , tcx . lifetimes . re_erased) } ; let sig = tcx . instantiate_bound_regions_with_erased (sig) ; tcx . mk_fn_sig (iter :: once (env_ty) . chain ([sig . tupled_inputs_ty]) , sig . to_coroutine_given_kind_and_upvars (tcx , args . as_coroutine_closure () . parent_args () , tcx . coroutine_for_closure (def_id) , coroutine_kind , tcx . lifetimes . re_erased , args . as_coroutine_closure () . tupled_upvars_ty () , args . as_coroutine_closure () . coroutine_captures_by_ref_ty () ,) , sig . c_variadic , sig . safety , sig . abi ,) } ty :: Coroutine (did , args) => { let coroutine_kind = tcx . coroutine_kind (did) . unwrap () ; let sig = args . as_coroutine () . sig () ; let env_ty = Ty :: new_mut_ref (tcx , tcx . lifetimes . re_erased , ty) ; let pin_did = tcx . require_lang_item (LangItem :: Pin , DUMMY_SP) ; let pin_adt_ref = tcx . adt_def (pin_did) ; let pin_args = tcx . mk_args (& [env_ty . into ()]) ; let env_ty = match coroutine_kind { hir :: CoroutineKind :: Desugared (hir :: CoroutineDesugaring :: Gen , _) => { env_ty } hir :: CoroutineKind :: Desugared (hir :: CoroutineDesugaring :: Async , _) | hir :: CoroutineKind :: Desugared (hir :: CoroutineDesugaring :: AsyncGen , _) | hir :: CoroutineKind :: Coroutine (_) => Ty :: new_adt (tcx , pin_adt_ref , pin_args) , } ; let (resume_ty , ret_ty) = match coroutine_kind { hir :: CoroutineKind :: Desugared (hir :: CoroutineDesugaring :: Async , _) => { assert_eq ! (sig . yield_ty , tcx . types . unit) ; let poll_did = tcx . require_lang_item (LangItem :: Poll , DUMMY_SP) ; let poll_adt_ref = tcx . adt_def (poll_did) ; let poll_args = tcx . mk_args (& [sig . return_ty . into ()]) ; let ret_ty = Ty :: new_adt (tcx , poll_adt_ref , poll_args) ; # [cfg (debug_assertions)] { if let ty :: Adt (resume_ty_adt , _) = sig . resume_ty . kind () { let expected_adt = tcx . adt_def (tcx . require_lang_item (LangItem :: ResumeTy , DUMMY_SP)) ; assert_eq ! (* resume_ty_adt , expected_adt) ; } else { panic ! ("expected `ResumeTy`, found `{:?}`" , sig . resume_ty) ; } ; } let context_mut_ref = Ty :: new_task_context (tcx) ; (Some (context_mut_ref) , ret_ty) } hir :: CoroutineKind :: Desugared (hir :: CoroutineDesugaring :: Gen , _) => { let option_did = tcx . require_lang_item (LangItem :: Option , DUMMY_SP) ; let option_adt_ref = tcx . adt_def (option_did) ; let option_args = tcx . mk_args (& [sig . yield_ty . into ()]) ; let ret_ty = Ty :: new_adt (tcx , option_adt_ref , option_args) ; assert_eq ! (sig . return_ty , tcx . types . unit) ; assert_eq ! (sig . resume_ty , tcx . types . unit) ; (None , ret_ty) } hir :: CoroutineKind :: Desugared (hir :: CoroutineDesugaring :: AsyncGen , _) => { assert_eq ! (sig . return_ty , tcx . types . unit) ; let ret_ty = sig . yield_ty ; # [cfg (debug_assertions)] { if let ty :: Adt (resume_ty_adt , _) = sig . resume_ty . kind () { let expected_adt = tcx . adt_def (tcx . require_lang_item (LangItem :: ResumeTy , DUMMY_SP)) ; assert_eq ! (* resume_ty_adt , expected_adt) ; } else { panic ! ("expected `ResumeTy`, found `{:?}`" , sig . resume_ty) ; } ; } let context_mut_ref = Ty :: new_task_context (tcx) ; (Some (context_mut_ref) , ret_ty) } hir :: CoroutineKind :: Coroutine (_) => { let state_did = tcx . require_lang_item (LangItem :: CoroutineState , DUMMY_SP) ; let state_adt_ref = tcx . adt_def (state_did) ; let state_args = tcx . mk_args (& [sig . yield_ty . into () , sig . return_ty . into ()]) ; let ret_ty = Ty :: new_adt (tcx , state_adt_ref , state_args) ; (Some (sig . resume_ty) , ret_ty) } } ; if let Some (resume_ty) = resume_ty { tcx . mk_fn_sig ([env_ty , resume_ty] , ret_ty , false , hir :: Safety :: Safe , rustc_abi :: ExternAbi :: Rust ,) } else { tcx . mk_fn_sig ([env_ty] , ret_ty , false , hir :: Safety :: Safe , rustc_abi :: ExternAbi :: Rust ,) } } _ => bug ! ("unexpected type {:?} in Instance::fn_sig" , ty) , } }
}

macro_rules! fn_abi_of_fn_ptr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fn_abi_of_fn_ptr in module {}", module_path!());
    };
}

mkfn!{
    fn_abi_of_fn_ptr_introspect!();
    fn fn_abi_of_fn_ptr < 'tcx > (tcx : TyCtxt < 'tcx > , query : ty :: PseudoCanonicalInput < 'tcx , (ty :: PolyFnSig < 'tcx > , & 'tcx ty :: List < Ty < 'tcx > >) > ,) -> Result < & 'tcx FnAbi < 'tcx , Ty < 'tcx > > , & 'tcx FnAbiError < 'tcx > > { let ty :: PseudoCanonicalInput { typing_env , value : (sig , extra_args) } = query ; fn_abi_new_uncached (& LayoutCx :: new (tcx , typing_env) , tcx . instantiate_bound_regions_with_erased (sig) , extra_args , None ,) }
}

macro_rules! fn_abi_of_instance_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fn_abi_of_instance in module {}", module_path!());
    };
}

mkfn!{
    fn_abi_of_instance_introspect!();
    fn fn_abi_of_instance < 'tcx > (tcx : TyCtxt < 'tcx > , query : ty :: PseudoCanonicalInput < 'tcx , (ty :: Instance < 'tcx > , & 'tcx ty :: List < Ty < 'tcx > >) > ,) -> Result < & 'tcx FnAbi < 'tcx , Ty < 'tcx > > , & 'tcx FnAbiError < 'tcx > > { let ty :: PseudoCanonicalInput { typing_env , value : (instance , extra_args) } = query ; fn_abi_new_uncached (& LayoutCx :: new (tcx , typing_env) , fn_sig_for_fn_abi (tcx , instance , typing_env) , extra_args , Some (instance) ,) }
}

macro_rules! arg_attrs_for_rust_scalar_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function arg_attrs_for_rust_scalar in module {}", module_path!());
    };
}

mkfn!{
    arg_attrs_for_rust_scalar_introspect!();
    fn arg_attrs_for_rust_scalar < 'tcx > (cx : LayoutCx < 'tcx > , scalar : Scalar , layout : TyAndLayout < 'tcx > , offset : Size , is_return : bool , drop_target_pointee : Option < Ty < 'tcx > > ,) -> ArgAttributes { let mut attrs = ArgAttributes :: new () ; if scalar . is_bool () { attrs . ext (ArgExtension :: Zext) ; attrs . set (ArgAttribute :: NoUndef) ; return attrs ; } if ! scalar . is_uninit_valid () { attrs . set (ArgAttribute :: NoUndef) ; } let Scalar :: Initialized { value : Pointer (_) , valid_range } = scalar else { return attrs } ; if ! valid_range . contains (0) || drop_target_pointee . is_some () { attrs . set (ArgAttribute :: NonNull) ; } let tcx = cx . tcx () ; if let Some (pointee) = layout . pointee_info_at (& cx , offset) { let kind = if let Some (kind) = pointee . safe { Some (kind) } else if let Some (pointee) = drop_target_pointee { Some (PointerKind :: MutableRef { unpin : pointee . is_unpin (tcx , cx . typing_env) }) } else { None } ; if let Some (kind) = kind { attrs . pointee_align = Some (pointee . align . min (cx . tcx () . sess . target . max_reliable_alignment ())) ; attrs . pointee_size = match kind { PointerKind :: Box { .. } | PointerKind :: SharedRef { frozen : false } | PointerKind :: MutableRef { unpin : false } => Size :: ZERO , PointerKind :: SharedRef { frozen : true } | PointerKind :: MutableRef { unpin : true } => pointee . size , } ; let noalias_for_box = tcx . sess . opts . unstable_opts . box_noalias ; let noalias_mut_ref = tcx . sess . opts . unstable_opts . mutable_noalias ; let no_alias = match kind { PointerKind :: SharedRef { frozen } => frozen , PointerKind :: MutableRef { unpin } => unpin && noalias_mut_ref , PointerKind :: Box { unpin , global } => unpin && global && noalias_for_box , } ; if no_alias && ! is_return { attrs . set (ArgAttribute :: NoAlias) ; } if matches ! (kind , PointerKind :: SharedRef { frozen : true }) && ! is_return { attrs . set (ArgAttribute :: ReadOnly) ; attrs . set (ArgAttribute :: CapturesReadOnly) ; } } } attrs }
}

macro_rules! fn_abi_sanity_check_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fn_abi_sanity_check in module {}", module_path!());
    };
}

mkfn!{
    fn_abi_sanity_check_introspect!();
    # [doc = " Ensure that the ABI makes basic sense."] fn fn_abi_sanity_check < 'tcx > (cx : & LayoutCx < 'tcx > , fn_abi : & FnAbi < 'tcx , Ty < 'tcx > > , spec_abi : ExternAbi ,) { fn fn_arg_sanity_check < 'tcx > (cx : & LayoutCx < 'tcx > , fn_abi : & FnAbi < 'tcx , Ty < 'tcx > > , spec_abi : ExternAbi , arg : & ArgAbi < 'tcx , Ty < 'tcx > > ,) { let tcx = cx . tcx () ; if spec_abi . is_rustic_abi () { if arg . layout . is_zst () { assert ! (arg . is_ignore ()) ; } if let PassMode :: Indirect { on_stack , .. } = arg . mode { assert ! (! on_stack , "rust abi shouldn't use on_stack") ; } } match & arg . mode { PassMode :: Ignore => { assert ! (arg . layout . is_zst ()) ; } PassMode :: Direct (_) => { match arg . layout . backend_repr { BackendRepr :: Scalar (_) | BackendRepr :: SimdVector { .. } => { } BackendRepr :: ScalarPair (..) => { panic ! ("`PassMode::Direct` used for ScalarPair type {}" , arg . layout . ty) } BackendRepr :: Memory { sized } => { assert ! (sized , "`PassMode::Direct` for unsized type in ABI: {:#?}" , fn_abi) ; assert ! (matches ! (spec_abi , ExternAbi :: Unadjusted) , "`PassMode::Direct` for aggregates only allowed for \"unadjusted\"\n\
                             Problematic type: {:#?}" , arg . layout ,) ; } } } PassMode :: Pair (_ , _) => { assert ! (matches ! (arg . layout . backend_repr , BackendRepr :: ScalarPair (..)) , "PassMode::Pair for type {}" , arg . layout . ty) ; } PassMode :: Cast { .. } => { assert ! (arg . layout . is_sized ()) ; } PassMode :: Indirect { meta_attrs : None , .. } => { assert ! (arg . layout . is_sized ()) ; } PassMode :: Indirect { meta_attrs : Some (_) , on_stack , .. } => { assert ! (arg . layout . is_unsized () && ! on_stack) ; let tail = tcx . struct_tail_for_codegen (arg . layout . ty , cx . typing_env) ; if matches ! (tail . kind () , ty :: Foreign (..)) { panic ! ("unsized arguments must not be `extern` types") ; } } } } for arg in fn_abi . args . iter () { fn_arg_sanity_check (cx , fn_abi , spec_abi , arg) ; } fn_arg_sanity_check (cx , fn_abi , spec_abi , & fn_abi . ret) ; }
}

macro_rules! fn_abi_new_uncached_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fn_abi_new_uncached in module {}", module_path!());
    };
}

mkfn!{
    fn_abi_new_uncached_introspect!();
    # [tracing :: instrument (level = "debug" , skip (cx , instance))] fn fn_abi_new_uncached < 'tcx > (cx : & LayoutCx < 'tcx > , sig : ty :: FnSig < 'tcx > , extra_args : & [Ty < 'tcx >] , instance : Option < ty :: Instance < 'tcx > > ,) -> Result < & 'tcx FnAbi < 'tcx , Ty < 'tcx > > , & 'tcx FnAbiError < 'tcx > > { let tcx = cx . tcx () ; let (caller_location , determined_fn_def_id , is_virtual_call) = if let Some (instance) = instance { let is_virtual_call = matches ! (instance . def , ty :: InstanceKind :: Virtual (..)) ; let is_tls_shim_call = matches ! (instance . def , ty :: InstanceKind :: ThreadLocalShim (_)) ; (instance . def . requires_caller_location (tcx) . then (| | tcx . caller_location_ty ()) , if is_virtual_call || is_tls_shim_call { None } else { Some (instance . def_id ()) } , is_virtual_call ,) } else { (None , None , false) } ; let sig = tcx . normalize_erasing_regions (cx . typing_env , sig) ; let abi_map = AbiMap :: from_target (& tcx . sess . target) ; let conv = abi_map . canonize_abi (sig . abi , sig . c_variadic) . unwrap () ; let mut inputs = sig . inputs () ; let extra_args = if sig . abi == ExternAbi :: RustCall { assert ! (! sig . c_variadic && extra_args . is_empty ()) ; if let Some (input) = sig . inputs () . last () && let ty :: Tuple (tupled_arguments) = input . kind () { inputs = & sig . inputs () [0 .. sig . inputs () . len () - 1] ; tupled_arguments } else { bug ! ("argument to function with \"rust-call\" ABI \
                    is not a tuple") ; } } else { assert ! (sig . c_variadic || extra_args . is_empty ()) ; extra_args } ; let is_drop_in_place = determined_fn_def_id . is_some_and (| def_id | { tcx . is_lang_item (def_id , LangItem :: DropInPlace) || tcx . is_lang_item (def_id , LangItem :: AsyncDropInPlace) }) ; let arg_of = | ty : Ty < 'tcx > , arg_idx : Option < usize > | -> Result < _ , & 'tcx FnAbiError < 'tcx > > { let span = tracing :: debug_span ! ("arg_of") ; let _entered = span . enter () ; let is_return = arg_idx . is_none () ; let is_drop_target = is_drop_in_place && arg_idx == Some (0) ; let drop_target_pointee = is_drop_target . then (| | match ty . kind () { ty :: RawPtr (ty , _) => * ty , _ => bug ! ("argument to drop_in_place is not a raw ptr: {:?}" , ty) , }) ; let layout = cx . layout_of (ty) . map_err (| err | & * tcx . arena . alloc (FnAbiError :: Layout (* err))) ? ; let layout = if is_virtual_call && arg_idx == Some (0) { make_thin_self_ptr (cx , layout) } else { layout } ; let mut arg = ArgAbi :: new (cx , layout , | layout , scalar , offset | { arg_attrs_for_rust_scalar (* cx , scalar , * layout , offset , is_return , drop_target_pointee) }) ; if arg . layout . is_zst () { arg . mode = PassMode :: Ignore ; } Ok (arg) } ; let mut fn_abi = FnAbi { ret : arg_of (sig . output () , None) ? , args : inputs . iter () . copied () . chain (extra_args . iter () . copied ()) . chain (caller_location) . enumerate () . map (| (i , ty) | arg_of (ty , Some (i))) . collect :: < Result < _ , _ > > () ? , c_variadic : sig . c_variadic , fixed_count : inputs . len () as u32 , conv , can_unwind : fn_can_unwind (tcx , determined_fn_def_id , sig . abi ,) , } ; fn_abi_adjust_for_abi (cx , & mut fn_abi , sig . abi , determined_fn_def_id ,) ; debug ! ("fn_abi_new_uncached = {:?}" , fn_abi) ; fn_abi_sanity_check (cx , & fn_abi , sig . abi) ; Ok (tcx . arena . alloc (fn_abi)) }
}

macro_rules! fn_abi_adjust_for_abi_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fn_abi_adjust_for_abi in module {}", module_path!());
    };
}

mkfn!{
    fn_abi_adjust_for_abi_introspect!();
    # [tracing :: instrument (level = "trace" , skip (cx))] fn fn_abi_adjust_for_abi < 'tcx > (cx : & LayoutCx < 'tcx > , fn_abi : & mut FnAbi < 'tcx , Ty < 'tcx > > , abi : ExternAbi , fn_def_id : Option < DefId > ,) { if abi == ExternAbi :: Unadjusted { fn unadjust < 'tcx > (arg : & mut ArgAbi < 'tcx , Ty < 'tcx > >) { if matches ! (arg . layout . backend_repr , BackendRepr :: Memory { .. }) { assert ! (arg . layout . backend_repr . is_sized () , "'unadjusted' ABI does not support unsized arguments") ; } arg . make_direct_deprecated () ; } unadjust (& mut fn_abi . ret) ; for arg in fn_abi . args . iter_mut () { unadjust (arg) ; } return ; } let tcx = cx . tcx () ; if abi . is_rustic_abi () { fn_abi . adjust_for_rust_abi (cx) ; let deduced_param_attrs = if tcx . sess . opts . optimize != OptLevel :: No && tcx . sess . opts . incremental . is_none () { fn_def_id . map (| fn_def_id | tcx . deduced_param_attrs (fn_def_id)) . unwrap_or_default () } else { & [] } ; for (arg_idx , arg) in fn_abi . args . iter_mut () . enumerate () { if arg . is_ignore () { continue ; } if let & mut PassMode :: Indirect { ref mut attrs , .. } = & mut arg . mode { if let Some (deduced_param_attrs) = deduced_param_attrs . get (arg_idx) && deduced_param_attrs . read_only { attrs . regular . insert (ArgAttribute :: ReadOnly) ; debug ! ("added deduced read-only attribute") ; } } } } else { fn_abi . adjust_for_foreign_abi (cx , abi) ; } }
}

macro_rules! make_thin_self_ptr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function make_thin_self_ptr in module {}", module_path!());
    };
}

mkfn!{
    make_thin_self_ptr_introspect!();
    # [tracing :: instrument (level = "debug" , skip (cx))] fn make_thin_self_ptr < 'tcx > (cx : & (impl HasTyCtxt < 'tcx > + HasTypingEnv < 'tcx >) , layout : TyAndLayout < 'tcx > ,) -> TyAndLayout < 'tcx > { let tcx = cx . tcx () ; let wide_pointer_ty = if layout . is_unsized () { Ty :: new_mut_ptr (tcx , layout . ty) } else { match layout . backend_repr { BackendRepr :: ScalarPair (..) | BackendRepr :: Scalar (..) => () , _ => bug ! ("receiver type has unsupported layout: {:?}" , layout) , } let mut wide_pointer_layout = layout ; while ! wide_pointer_layout . ty . is_raw_ptr () && ! wide_pointer_layout . ty . is_ref () { wide_pointer_layout = wide_pointer_layout . non_1zst_field (cx) . expect ("not exactly one non-1-ZST field in a `DispatchFromDyn` type") . 1 } wide_pointer_layout . ty } ; let unit_ptr_ty = Ty :: new_mut_ptr (tcx , tcx . types . unit) ; TyAndLayout { ty : wide_pointer_ty , .. tcx . layout_of (ty :: TypingEnv :: fully_monomorphized () . as_query_input (unit_ptr_ty)) . unwrap () } }
}