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
mkuse!{use itertools :: Itertools as _ ;}
mkuse!{use rustc_data_structures :: fx :: FxIndexSet ;}
mkuse!{use rustc_hir as hir ;}
mkuse!{use rustc_hir :: def_id :: { DefId , LocalDefId } ;}
mkuse!{use rustc_infer :: infer :: TyCtxtInferExt ;}
mkuse!{use rustc_lint_defs :: builtin :: { REFINING_IMPL_TRAIT_INTERNAL , REFINING_IMPL_TRAIT_REACHABLE } ;}
mkuse!{use rustc_middle :: span_bug ;}
mkuse!{use rustc_middle :: traits :: ObligationCause ;}
mkuse!{use rustc_middle :: ty :: { self , Ty , TyCtxt , TypeFoldable , TypeFolder , TypeSuperVisitable , TypeVisitable , TypeVisitableExt , TypeVisitor , TypingMode , } ;}
mkuse!{use rustc_span :: Span ;}
mkuse!{use rustc_trait_selection :: regions :: InferCtxtRegionExt ;}
mkuse!{use rustc_trait_selection :: traits :: { ObligationCtxt , elaborate , normalize_param_env_or_error } ;}

macro_rules! check_refining_return_position_impl_trait_in_trait_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_refining_return_position_impl_trait_in_trait in module {}", module_path!());
    };
}

mkfn!{
    check_refining_return_position_impl_trait_in_trait_introspect!();
    #[doc = " Check that an implementation does not refine an RPITIT from a trait method signature."] pub (crate) fn check_refining_return_position_impl_trait_in_trait < 'tcx > (tcx : TyCtxt < 'tcx > , impl_m : ty :: AssocItem , trait_m : ty :: AssocItem , impl_trait_ref : ty :: TraitRef < 'tcx > ,) { if ! tcx . impl_method_has_trait_impl_trait_tys (impl_m . def_id) { return ; } let is_internal = trait_m . container_id (tcx) . as_local () . is_some_and (| trait_def_id | ! tcx . effective_visibilities (()) . is_reachable (trait_def_id)) || impl_trait_ref . args . iter () . any (| arg | { if let Some (ty) = arg . as_type () && let Some (self_visibility) = type_visibility (tcx , ty) { return ! self_visibility . is_public () ; } false }) ; let impl_def_id = impl_m . container_id (tcx) ; let impl_m_args = ty :: GenericArgs :: identity_for_item (tcx , impl_m . def_id) ; let trait_m_to_impl_m_args = impl_m_args . rebase_onto (tcx , impl_def_id , impl_trait_ref . args) ; let bound_trait_m_sig = tcx . fn_sig (trait_m . def_id) . instantiate (tcx , trait_m_to_impl_m_args) ; let trait_m_sig = tcx . liberate_late_bound_regions (impl_m . def_id , bound_trait_m_sig) ; let trait_m_sig_with_self_for_diag = tcx . liberate_late_bound_regions (impl_m . def_id , tcx . fn_sig (trait_m . def_id) . instantiate (tcx , tcx . mk_args_from_iter ([tcx . types . self_param . into ()] . into_iter () . chain (trait_m_to_impl_m_args . iter () . skip (1)) ,) ,) ,) ; let Ok (hidden_tys) = tcx . collect_return_position_impl_trait_in_trait_tys (impl_m . def_id) else { return ; } ; if hidden_tys . items () . any (| (_ , & ty) | ty . skip_binder () . references_error ()) { return ; } let mut collector = ImplTraitInTraitCollector { tcx , types : FxIndexSet :: default () } ; trait_m_sig . visit_with (& mut collector) ; let mut trait_bounds = vec ! [] ; let mut impl_bounds = vec ! [] ; let mut pairs = vec ! [] ; for trait_projection in collector . types . into_iter () . rev () { let impl_opaque_args = trait_projection . args . rebase_onto (tcx , trait_m . def_id , impl_m_args) ; let hidden_ty = hidden_tys [& trait_projection . def_id] . instantiate (tcx , impl_opaque_args) ; let ty :: Alias (ty :: Opaque , impl_opaque) = * hidden_ty . kind () else { report_mismatched_rpitit_signature (tcx , trait_m_sig_with_self_for_diag , trait_m . def_id , impl_m . def_id , None , is_internal ,) ; return ; } ; if ! tcx . hir_get_if_local (impl_opaque . def_id) . is_some_and (| node | { matches ! (node . expect_opaque_ty () . origin , hir :: OpaqueTyOrigin :: AsyncFn { parent , .. } | hir :: OpaqueTyOrigin :: FnReturn { parent , .. } if parent == impl_m . def_id . expect_local ()) }) { report_mismatched_rpitit_signature (tcx , trait_m_sig_with_self_for_diag , trait_m . def_id , impl_m . def_id , None , is_internal ,) ; return ; } trait_bounds . extend (tcx . item_bounds (trait_projection . def_id) . iter_instantiated (tcx , trait_projection . args) ,) ; impl_bounds . extend (elaborate (tcx , tcx . explicit_item_bounds (impl_opaque . def_id) . iter_instantiated_copied (tcx , impl_opaque . args) ,)) ; pairs . push ((trait_projection , impl_opaque)) ; } let hybrid_preds = tcx . predicates_of (impl_def_id) . instantiate_identity (tcx) . into_iter () . chain (tcx . predicates_of (trait_m . def_id) . instantiate_own (tcx , trait_m_to_impl_m_args)) . map (| (clause , _) | clause) ; let param_env = ty :: ParamEnv :: new (tcx . mk_clauses_from_iter (hybrid_preds)) ; let param_env = normalize_param_env_or_error (tcx , param_env , ObligationCause :: dummy ()) ; let ref infcx = tcx . infer_ctxt () . build (TypingMode :: non_body_analysis ()) ; let ocx = ObligationCtxt :: new (infcx) ; let Ok ((trait_bounds , impl_bounds)) = ocx . deeply_normalize (& ObligationCause :: dummy () , param_env , (trait_bounds , impl_bounds)) else { tcx . dcx () . delayed_bug ("encountered errors when checking RPITIT refinement (selection)") ; return ; } ; let mut implied_wf_types = FxIndexSet :: default () ; implied_wf_types . extend (trait_m_sig . inputs_and_output) ; implied_wf_types . extend (ocx . normalize (& ObligationCause :: dummy () , param_env , trait_m_sig . inputs_and_output ,)) ; if ! ocx . select_all_or_error () . is_empty () { tcx . dcx () . delayed_bug ("encountered errors when checking RPITIT refinement (selection)") ; return ; } let errors = infcx . resolve_regions (impl_m . def_id . expect_local () , param_env , implied_wf_types) ; if ! errors . is_empty () { tcx . dcx () . delayed_bug ("encountered errors when checking RPITIT refinement (regions)") ; return ; } let Ok ((trait_bounds , impl_bounds)) = infcx . fully_resolve ((trait_bounds , impl_bounds)) else { tcx . dcx () . delayed_bug ("encountered errors when checking RPITIT refinement (resolution)") ; return ; } ; if trait_bounds . references_error () || impl_bounds . references_error () { return ; } let trait_bounds = FxIndexSet :: from_iter (trait_bounds . fold_with (& mut Anonymize { tcx })) ; let impl_bounds = impl_bounds . fold_with (& mut Anonymize { tcx }) ; for (clause , span) in impl_bounds { if ! trait_bounds . contains (& clause) { report_mismatched_rpitit_signature (tcx , trait_m_sig_with_self_for_diag , trait_m . def_id , impl_m . def_id , Some (span) , is_internal ,) ; return ; } } for (trait_projection , impl_opaque) in pairs { let impl_variances = tcx . variances_of (impl_opaque . def_id) ; let impl_captures : FxIndexSet < _ > = impl_opaque . args . iter () . zip_eq (impl_variances) . filter (| (_ , v) | * * v == ty :: Invariant) . map (| (arg , _) | arg) . collect () ; let trait_variances = tcx . variances_of (trait_projection . def_id) ; let mut trait_captures = FxIndexSet :: default () ; for (arg , variance) in trait_projection . args . iter () . zip_eq (trait_variances) { if * variance != ty :: Invariant { continue ; } arg . visit_with (& mut CollectParams { params : & mut trait_captures }) ; } if ! trait_captures . iter () . all (| arg | impl_captures . contains (arg)) { report_mismatched_rpitit_captures (tcx , impl_opaque . def_id . expect_local () , trait_captures , is_internal ,) ; } } }
}
mkitem!{mkstruct!{struct ImplTraitInTraitCollector < 'tcx > { tcx : TyCtxt < 'tcx > , types : FxIndexSet < ty :: AliasTy < 'tcx > > , }}}
mkitem!{mkimpl!{impl < 'tcx > TypeVisitor < TyCtxt < 'tcx > > for ImplTraitInTraitCollector < 'tcx > { fn visit_ty (& mut self , ty : Ty < 'tcx >) { if let ty :: Alias (ty :: Projection , proj) = * ty . kind () && self . tcx . is_impl_trait_in_trait (proj . def_id) { if self . types . insert (proj) { for (pred , _) in self . tcx . explicit_item_bounds (proj . def_id) . iter_instantiated_copied (self . tcx , proj . args) { pred . visit_with (self) ; } } } else { ty . super_visit_with (self) ; } } }}}

macro_rules! report_mismatched_rpitit_signature_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function report_mismatched_rpitit_signature in module {}", module_path!());
    };
}

mkfn!{
    report_mismatched_rpitit_signature_introspect!();
    fn report_mismatched_rpitit_signature < 'tcx > (tcx : TyCtxt < 'tcx > , trait_m_sig : ty :: FnSig < 'tcx > , trait_m_def_id : DefId , impl_m_def_id : DefId , unmatched_bound : Option < Span > , is_internal : bool ,) { let mapping = std :: iter :: zip (tcx . fn_sig (trait_m_def_id) . skip_binder () . bound_vars () , tcx . fn_sig (impl_m_def_id) . skip_binder () . bound_vars () ,) . enumerate () . filter_map (| (idx , (impl_bv , trait_bv)) | { if let ty :: BoundVariableKind :: Region (impl_bv) = impl_bv && let ty :: BoundVariableKind :: Region (trait_bv) = trait_bv { let var = ty :: BoundVar :: from_usize (idx) ; Some ((ty :: LateParamRegionKind :: from_bound (var , impl_bv) , ty :: LateParamRegionKind :: from_bound (var , trait_bv) ,)) } else { None } }) . collect () ; let mut return_ty = trait_m_sig . output () . fold_with (& mut super :: RemapLateParam { tcx , mapping }) ; if tcx . asyncness (impl_m_def_id) . is_async () && tcx . asyncness (trait_m_def_id) . is_async () { let ty :: Alias (ty :: Projection , future_ty) = return_ty . kind () else { span_bug ! (tcx . def_span (trait_m_def_id) , "expected return type of async fn in trait to be a AFIT projection") ; } ; let Some (future_output_ty) = tcx . explicit_item_bounds (future_ty . def_id) . iter_instantiated_copied (tcx , future_ty . args) . find_map (| (clause , _) | match clause . kind () . no_bound_vars () ? { ty :: ClauseKind :: Projection (proj) => proj . term . as_type () , _ => None , }) else { span_bug ! (tcx . def_span (trait_m_def_id) , "expected `Future` projection bound in AFIT") ; } ; return_ty = future_output_ty ; } let (span , impl_return_span , pre , post) = match tcx . hir_node_by_def_id (impl_m_def_id . expect_local ()) . fn_decl () . unwrap () . output { hir :: FnRetTy :: DefaultReturn (span) => (tcx . def_span (impl_m_def_id) , span , "-> " , " ") , hir :: FnRetTy :: Return (ty) => (ty . span , ty . span , "" , "") , } ; let trait_return_span = tcx . hir_get_if_local (trait_m_def_id) . map (| node | match node . fn_decl () . unwrap () . output { hir :: FnRetTy :: DefaultReturn (_) => tcx . def_span (trait_m_def_id) , hir :: FnRetTy :: Return (ty) => ty . span , }) ; let span = unmatched_bound . unwrap_or (span) ; tcx . emit_node_span_lint (if is_internal { REFINING_IMPL_TRAIT_INTERNAL } else { REFINING_IMPL_TRAIT_REACHABLE } , tcx . local_def_id_to_hir_id (impl_m_def_id . expect_local ()) , span , crate :: errors :: ReturnPositionImplTraitInTraitRefined { impl_return_span , trait_return_span , pre , post , return_ty , unmatched_bound , } ,) ; }
}

macro_rules! type_visibility_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function type_visibility in module {}", module_path!());
    };
}

mkfn!{
    type_visibility_introspect!();
    fn type_visibility < 'tcx > (tcx : TyCtxt < 'tcx > , ty : Ty < 'tcx >) -> Option < ty :: Visibility < DefId > > { match * ty . kind () { ty :: Ref (_ , ty , _) => type_visibility (tcx , ty) , ty :: Adt (def , args) => { if def . is_fundamental () { type_visibility (tcx , args . type_at (0)) } else { Some (tcx . visibility (def . did ())) } } _ => None , } }
}
mkitem!{mkstruct!{struct Anonymize < 'tcx > { tcx : TyCtxt < 'tcx > , }}}
mkitem!{mkimpl!{impl < 'tcx > TypeFolder < TyCtxt < 'tcx > > for Anonymize < 'tcx > { fn cx (& self) -> TyCtxt < 'tcx > { self . tcx } fn fold_binder < T > (& mut self , t : ty :: Binder < 'tcx , T >) -> ty :: Binder < 'tcx , T > where T : TypeFoldable < TyCtxt < 'tcx > > , { self . tcx . anonymize_bound_vars (t) } }}}
mkitem!{mkstruct!{struct CollectParams < 'a , 'tcx > { params : & 'a mut FxIndexSet < ty :: GenericArg < 'tcx > > , }}}
mkitem!{mkimpl!{impl < 'tcx > TypeVisitor < TyCtxt < 'tcx > > for CollectParams < '_ , 'tcx > { fn visit_ty (& mut self , ty : Ty < 'tcx >) { if let ty :: Param (_) = ty . kind () { self . params . insert (ty . into ()) ; } else { ty . super_visit_with (self) ; } } fn visit_region (& mut self , r : ty :: Region < 'tcx >) { match r . kind () { ty :: ReEarlyParam (_) | ty :: ReLateParam (_) => { self . params . insert (r . into ()) ; } _ => { } } } fn visit_const (& mut self , ct : ty :: Const < 'tcx >) { if let ty :: ConstKind :: Param (_) = ct . kind () { self . params . insert (ct . into ()) ; } else { ct . super_visit_with (self) ; } } }}}

macro_rules! report_mismatched_rpitit_captures_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function report_mismatched_rpitit_captures in module {}", module_path!());
    };
}

mkfn!{
    report_mismatched_rpitit_captures_introspect!();
    fn report_mismatched_rpitit_captures < 'tcx > (tcx : TyCtxt < 'tcx > , impl_opaque_def_id : LocalDefId , mut trait_captured_args : FxIndexSet < ty :: GenericArg < 'tcx > > , is_internal : bool ,) { let Some (use_bound_span) = tcx . hir_node_by_def_id (impl_opaque_def_id) . expect_opaque_ty () . bounds . iter () . find_map (| bound | match * bound { rustc_hir :: GenericBound :: Use (_ , span) => Some (span) , hir :: GenericBound :: Trait (_) | hir :: GenericBound :: Outlives (_) => None , } ,) else { tcx . dcx () . delayed_bug ("expected use<..> to undercapture in an impl opaque") ; return ; } ; trait_captured_args . sort_by_cached_key (| arg | ! matches ! (arg . kind () , ty :: GenericArgKind :: Lifetime (_))) ; let suggestion = format ! ("use<{}>" , trait_captured_args . iter () . join (", ")) ; tcx . emit_node_span_lint (if is_internal { REFINING_IMPL_TRAIT_INTERNAL } else { REFINING_IMPL_TRAIT_REACHABLE } , tcx . local_def_id_to_hir_id (impl_opaque_def_id) , use_bound_span , crate :: errors :: ReturnPositionImplTraitInTraitRefinedLifetimes { suggestion_span : use_bound_span , suggestion , } ,) ; }
}