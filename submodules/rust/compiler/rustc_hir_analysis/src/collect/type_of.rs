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
mkuse!{use core :: ops :: ControlFlow ;}
mkuse!{use rustc_errors :: { Applicability , StashKey , Suggestions } ;}
mkuse!{use rustc_hir :: def_id :: { DefId , LocalDefId } ;}
mkuse!{use rustc_hir :: intravisit :: VisitorExt ;}
mkuse!{use rustc_hir :: { self as hir , AmbigArg , HirId } ;}
mkuse!{use rustc_middle :: query :: plumbing :: CyclePlaceholder ;}
mkuse!{use rustc_middle :: ty :: print :: with_forced_trimmed_paths ;}
mkuse!{use rustc_middle :: ty :: util :: IntTypeExt ;}
mkuse!{use rustc_middle :: ty :: { self , DefiningScopeKind , IsSuggestable , Ty , TyCtxt , TypeVisitableExt , fold_regions , } ;}
mkuse!{use rustc_middle :: { bug , span_bug } ;}
mkuse!{use rustc_span :: { DUMMY_SP , Ident , Span } ;}
mkuse!{use super :: { HirPlaceholderCollector , ItemCtxt , bad_placeholder } ;}
mkuse!{use crate :: check :: wfcheck :: check_static_item ;}
mkuse!{use crate :: errors :: TypeofReservedKeywordUsed ;}
mkuse!{use crate :: hir_ty_lowering :: HirTyLowerer ;}
mkmod!{opaque, { 
                getname!(opaque);
                getsrc!(opaque);
                getpath!(opaque);
                get_deps!(opaque);
                get_crates!(opaque);
                mkinclude!(opaque);
                 
            }}

macro_rules! anon_const_type_of_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function anon_const_type_of in module {}", module_path!());
    };
}

mkfn!{
    anon_const_type_of_introspect!();
    fn anon_const_type_of < 'tcx > (icx : & ItemCtxt < 'tcx > , def_id : LocalDefId) -> Ty < 'tcx > { use hir :: * ; use rustc_middle :: ty :: Ty ; let tcx = icx . tcx ; let hir_id = tcx . local_def_id_to_hir_id (def_id) ; let node = tcx . hir_node (hir_id) ; let Node :: AnonConst (& AnonConst { span , .. }) = node else { span_bug ! (tcx . def_span (def_id) , "expected anon const in `anon_const_type_of`, got {node:?}") ; } ; let parent_node_id = tcx . parent_hir_id (hir_id) ; let parent_node = tcx . hir_node (parent_node_id) ; match parent_node { Node :: ConstArg (& ConstArg { hir_id : arg_hir_id , kind : ConstArgKind :: Anon (& AnonConst { hir_id : anon_hir_id , .. }) , .. }) if anon_hir_id == hir_id => const_arg_anon_type_of (icx , arg_hir_id , span) , Node :: Variant (Variant { disr_expr : Some (e) , .. }) if e . hir_id == hir_id => { tcx . adt_def (tcx . hir_get_parent_item (hir_id)) . repr () . discr_type () . to_ty (tcx) } Node :: Ty (& hir :: Ty { kind : TyKind :: Typeof (ref e) , span , .. }) if e . hir_id == hir_id => { let ty = tcx . typeck (def_id) . node_type (tcx . local_def_id_to_hir_id (def_id)) ; let ty = fold_regions (tcx , ty , | r , _ | { if r . is_erased () { ty :: Region :: new_error_misc (tcx) } else { r } }) ; let (ty , opt_sugg) = if let Some (ty) = ty . make_suggestable (tcx , false , None) { (ty , Some ((span , Applicability :: MachineApplicable))) } else { (ty , None) } ; tcx . dcx () . emit_err (TypeofReservedKeywordUsed { span , ty , opt_sugg }) ; return ty ; } Node :: Field (& hir :: FieldDef { default : Some (c) , def_id : field_def_id , .. }) if c . hir_id == hir_id => { tcx . type_of (field_def_id) . instantiate_identity () } _ => Ty :: new_error_with_message (tcx , span , format ! ("unexpected anon const parent in type_of(): {parent_node:?}") ,) , } }
}

macro_rules! const_arg_anon_type_of_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function const_arg_anon_type_of in module {}", module_path!());
    };
}

mkfn!{
    const_arg_anon_type_of_introspect!();
    fn const_arg_anon_type_of < 'tcx > (icx : & ItemCtxt < 'tcx > , arg_hir_id : HirId , span : Span) -> Ty < 'tcx > { use hir :: * ; use rustc_middle :: ty :: Ty ; let tcx = icx . tcx ; match tcx . parent_hir_node (arg_hir_id) { Node :: Ty (& hir :: Ty { kind : TyKind :: Array (_ , ref constant) , .. }) | Node :: Expr (& Expr { kind : ExprKind :: Repeat (_ , ref constant) , .. }) if constant . hir_id == arg_hir_id => { tcx . types . usize } Node :: TyPat (pat) => { let node = match tcx . parent_hir_node (pat . hir_id) { Node :: TyPat (p) => tcx . parent_hir_node (p . hir_id) , other => other , } ; let hir :: TyKind :: Pat (ty , _) = node . expect_ty () . kind else { bug ! () } ; icx . lower_ty (ty) } _ => Ty :: new_error_with_message (tcx , span , "`type_of` called on const argument's anon const before the const argument was lowered" ,) , } }
}

macro_rules! type_of_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function type_of in module {}", module_path!());
    };
}

mkfn!{
    type_of_introspect!();
    pub (super) fn type_of (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> ty :: EarlyBinder < '_ , Ty < '_ > > { use rustc_hir :: * ; use rustc_middle :: ty :: Ty ; match tcx . opt_rpitit_info (def_id . to_def_id ()) { Some (ty :: ImplTraitInTraitData :: Impl { fn_def_id }) => { match tcx . collect_return_position_impl_trait_in_trait_tys (fn_def_id) { Ok (map) => { let trait_item_def_id = tcx . trait_item_of (def_id) . unwrap () ; return map [& trait_item_def_id] ; } Err (_) => { return ty :: EarlyBinder :: bind (Ty :: new_error_with_message (tcx , DUMMY_SP , "Could not collect return position impl trait in trait tys" ,)) ; } } } Some (ty :: ImplTraitInTraitData :: Trait { opaque_def_id , .. }) => { return ty :: EarlyBinder :: bind (Ty :: new_opaque (tcx , opaque_def_id , ty :: GenericArgs :: identity_for_item (tcx , opaque_def_id) ,)) ; } None => { } } let hir_id = tcx . local_def_id_to_hir_id (def_id) ; let icx = ItemCtxt :: new (tcx , def_id) ; let output = match tcx . hir_node (hir_id) { Node :: TraitItem (item) => match item . kind { TraitItemKind :: Fn (..) => { let args = ty :: GenericArgs :: identity_for_item (tcx , def_id) ; Ty :: new_fn_def (tcx , def_id . to_def_id () , args) } TraitItemKind :: Const (ty , body_id) => body_id . and_then (| body_id | { ty . is_suggestable_infer_ty () . then (| | { infer_placeholder_type (icx . lowerer () , def_id , body_id , ty . span , item . ident , "associated constant" ,) }) }) . unwrap_or_else (| | icx . lower_ty (ty)) , TraitItemKind :: Type (_ , Some (ty)) => icx . lower_ty (ty) , TraitItemKind :: Type (_ , None) => { span_bug ! (item . span , "associated type missing default") ; } } , Node :: ImplItem (item) => match item . kind { ImplItemKind :: Fn (..) => { let args = ty :: GenericArgs :: identity_for_item (tcx , def_id) ; Ty :: new_fn_def (tcx , def_id . to_def_id () , args) } ImplItemKind :: Const (ty , body_id) => { if ty . is_suggestable_infer_ty () { infer_placeholder_type (icx . lowerer () , def_id , body_id , ty . span , item . ident , "associated constant" ,) } else { icx . lower_ty (ty) } } ImplItemKind :: Type (ty) => { if let ImplItemImplKind :: Inherent { .. } = item . impl_kind { check_feature_inherent_assoc_ty (tcx , item . span) ; } icx . lower_ty (ty) } } , Node :: Item (item) => match item . kind { ItemKind :: Static (_ , ident , ty , body_id) => { if ty . is_suggestable_infer_ty () { infer_placeholder_type (icx . lowerer () , def_id , body_id , ty . span , ident , "static variable" ,) } else { let ty = icx . lower_ty (ty) ; match check_static_item (tcx , def_id , ty , false) { Ok (()) => ty , Err (guar) => Ty :: new_error (tcx , guar) , } } } ItemKind :: Const (ident , _ , ty , body_id) => { if ty . is_suggestable_infer_ty () { infer_placeholder_type (icx . lowerer () , def_id , body_id , ty . span , ident , "constant" ,) } else { icx . lower_ty (ty) } } ItemKind :: TyAlias (_ , _ , self_ty) => icx . lower_ty (self_ty) , ItemKind :: Impl (hir :: Impl { self_ty , .. }) => match self_ty . find_self_aliases () { spans if spans . len () > 0 => { let guar = tcx . dcx () . emit_err (crate :: errors :: SelfInImplSelf { span : spans . into () , note : () }) ; Ty :: new_error (tcx , guar) } _ => icx . lower_ty (self_ty) , } , ItemKind :: Fn { .. } => { let args = ty :: GenericArgs :: identity_for_item (tcx , def_id) ; Ty :: new_fn_def (tcx , def_id . to_def_id () , args) } ItemKind :: Enum (..) | ItemKind :: Struct (..) | ItemKind :: Union (..) => { let def = tcx . adt_def (def_id) ; let args = ty :: GenericArgs :: identity_for_item (tcx , def_id) ; Ty :: new_adt (tcx , def , args) } ItemKind :: GlobalAsm { .. } => tcx . typeck (def_id) . node_type (hir_id) , ItemKind :: Trait (..) | ItemKind :: TraitAlias (..) | ItemKind :: Macro (..) | ItemKind :: Mod (..) | ItemKind :: ForeignMod { .. } | ItemKind :: ExternCrate (..) | ItemKind :: Use (..) => { span_bug ! (item . span , "compute_type_of_item: unexpected item type: {:?}" , item . kind) ; } } , Node :: OpaqueTy (..) => tcx . type_of_opaque (def_id) . map_or_else (| CyclePlaceholder (guar) | Ty :: new_error (tcx , guar) , | ty | ty . instantiate_identity () ,) , Node :: ForeignItem (foreign_item) => match foreign_item . kind { ForeignItemKind :: Fn (..) => { let args = ty :: GenericArgs :: identity_for_item (tcx , def_id) ; Ty :: new_fn_def (tcx , def_id . to_def_id () , args) } ForeignItemKind :: Static (ty , _ , _) => { let ty = icx . lower_ty (ty) ; match check_static_item (tcx , def_id , ty , false) { Ok (()) => ty , Err (guar) => Ty :: new_error (tcx , guar) , } } ForeignItemKind :: Type => Ty :: new_foreign (tcx , def_id . to_def_id ()) , } , Node :: Ctor (def) | Node :: Variant (Variant { data : def , .. }) => match def { VariantData :: Unit (..) | VariantData :: Struct { .. } => { tcx . type_of (tcx . hir_get_parent_item (hir_id)) . instantiate_identity () } VariantData :: Tuple (_ , _ , ctor) => { let args = ty :: GenericArgs :: identity_for_item (tcx , def_id) ; Ty :: new_fn_def (tcx , ctor . to_def_id () , args) } } , Node :: Field (field) => icx . lower_ty (field . ty) , Node :: Expr (& Expr { kind : ExprKind :: Closure { .. } , .. }) => { tcx . typeck (def_id) . node_type (hir_id) } Node :: AnonConst (_) => anon_const_type_of (& icx , def_id) , Node :: ConstBlock (_) => { let args = ty :: GenericArgs :: identity_for_item (tcx , def_id . to_def_id ()) ; args . as_inline_const () . ty () } Node :: GenericParam (param) => match & param . kind { GenericParamKind :: Type { default : Some (ty) , .. } | GenericParamKind :: Const { ty , .. } => icx . lower_ty (ty) , x => bug ! ("unexpected non-type Node::GenericParam: {:?}" , x) , } , x => { bug ! ("unexpected sort of node in type_of(): {:?}" , x) ; } } ; if let Err (e) = icx . check_tainted_by_errors () && ! output . references_error () { ty :: EarlyBinder :: bind (Ty :: new_error (tcx , e)) } else { ty :: EarlyBinder :: bind (output) } }
}

macro_rules! type_of_opaque_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function type_of_opaque in module {}", module_path!());
    };
}

mkfn!{
    type_of_opaque_introspect!();
    pub (super) fn type_of_opaque (tcx : TyCtxt < '_ > , def_id : DefId ,) -> Result < ty :: EarlyBinder < '_ , Ty < '_ > > , CyclePlaceholder > { if let Some (def_id) = def_id . as_local () { Ok (ty :: EarlyBinder :: bind (match tcx . hir_node_by_def_id (def_id) . expect_opaque_ty () . origin { hir :: OpaqueTyOrigin :: TyAlias { in_assoc_ty : false , .. } => { opaque :: find_opaque_ty_constraints_for_tait (tcx , def_id , DefiningScopeKind :: MirBorrowck ,) } hir :: OpaqueTyOrigin :: TyAlias { in_assoc_ty : true , .. } => { opaque :: find_opaque_ty_constraints_for_impl_trait_in_assoc_type (tcx , def_id , DefiningScopeKind :: MirBorrowck ,) } hir :: OpaqueTyOrigin :: FnReturn { parent : owner , in_trait_or_impl } | hir :: OpaqueTyOrigin :: AsyncFn { parent : owner , in_trait_or_impl } => { if in_trait_or_impl == Some (hir :: RpitContext :: Trait) && ! tcx . defaultness (owner) . has_value () { span_bug ! (tcx . def_span (def_id) , "tried to get type of this RPITIT with no definition") ; } opaque :: find_opaque_ty_constraints_for_rpit (tcx , def_id , owner , DefiningScopeKind :: MirBorrowck ,) } })) } else { Ok (tcx . type_of (def_id)) } }
}

macro_rules! type_of_opaque_hir_typeck_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function type_of_opaque_hir_typeck in module {}", module_path!());
    };
}

mkfn!{
    type_of_opaque_hir_typeck_introspect!();
    pub (super) fn type_of_opaque_hir_typeck (tcx : TyCtxt < '_ > , def_id : LocalDefId ,) -> ty :: EarlyBinder < '_ , Ty < '_ > > { ty :: EarlyBinder :: bind (match tcx . hir_node_by_def_id (def_id) . expect_opaque_ty () . origin { hir :: OpaqueTyOrigin :: TyAlias { in_assoc_ty : false , .. } => { opaque :: find_opaque_ty_constraints_for_tait (tcx , def_id , DefiningScopeKind :: HirTypeck) } hir :: OpaqueTyOrigin :: TyAlias { in_assoc_ty : true , .. } => { opaque :: find_opaque_ty_constraints_for_impl_trait_in_assoc_type (tcx , def_id , DefiningScopeKind :: HirTypeck ,) } hir :: OpaqueTyOrigin :: FnReturn { parent : owner , in_trait_or_impl } | hir :: OpaqueTyOrigin :: AsyncFn { parent : owner , in_trait_or_impl } => { if in_trait_or_impl == Some (hir :: RpitContext :: Trait) && ! tcx . defaultness (owner) . has_value () { span_bug ! (tcx . def_span (def_id) , "tried to get type of this RPITIT with no definition") ; } opaque :: find_opaque_ty_constraints_for_rpit (tcx , def_id , owner , DefiningScopeKind :: HirTypeck ,) } }) }
}

macro_rules! infer_placeholder_type_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function infer_placeholder_type in module {}", module_path!());
    };
}

mkfn!{
    infer_placeholder_type_introspect!();
    fn infer_placeholder_type < 'tcx > (cx : & dyn HirTyLowerer < 'tcx > , def_id : LocalDefId , body_id : hir :: BodyId , span : Span , item_ident : Ident , kind : & 'static str ,) -> Ty < 'tcx > { let tcx = cx . tcx () ; let ty = tcx . typeck (def_id) . node_type (body_id . hir_id) ; let guar = cx . dcx () . try_steal_modify_and_emit_err (span , StashKey :: ItemNoType , | err | { if ! ty . references_error () { let colon = if span == item_ident . span . shrink_to_hi () { ":" } else { "" } ; if let Suggestions :: Enabled (suggestions) = & mut err . suggestions { suggestions . clear () ; } if let Some (ty) = ty . make_suggestable (tcx , false , None) { err . span_suggestion (span , format ! ("provide a type for the {kind}") , format ! ("{colon} {ty}") , Applicability :: MachineApplicable ,) ; } else { with_forced_trimmed_paths ! (err . span_note (tcx . hir_body (body_id) . value . span , format ! ("however, the inferred type `{ty}` cannot be named") ,)) ; } } }) . unwrap_or_else (| | { let mut visitor = HirPlaceholderCollector :: default () ; let node = tcx . hir_node_by_def_id (def_id) ; if let Some (ty) = node . ty () { visitor . visit_ty_unambig (ty) ; } if visitor . spans . is_empty () { visitor . spans . push (span) ; } let mut diag = bad_placeholder (cx , visitor . spans , kind) ; if span . is_empty () && span . from_expansion () { diag . primary_message ("missing type for item") ; } else if ! ty . references_error () { if let Some (ty) = ty . make_suggestable (tcx , false , None) { diag . span_suggestion_verbose (span , "replace this with a fully-specified type" , ty , Applicability :: MachineApplicable ,) ; } else { with_forced_trimmed_paths ! (diag . span_note (tcx . hir_body (body_id) . value . span , format ! ("however, the inferred type `{ty}` cannot be named") ,)) ; } } diag . emit () }) ; Ty :: new_error (tcx , guar) }
}

macro_rules! check_feature_inherent_assoc_ty_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_feature_inherent_assoc_ty in module {}", module_path!());
    };
}

mkfn!{
    check_feature_inherent_assoc_ty_introspect!();
    fn check_feature_inherent_assoc_ty (tcx : TyCtxt < '_ > , span : Span) { if ! tcx . features () . inherent_associated_types () { use rustc_session :: parse :: feature_err ; use rustc_span :: sym ; feature_err (& tcx . sess , sym :: inherent_associated_types , span , "inherent associated types are unstable" ,) . emit () ; } }
}

macro_rules! type_alias_is_lazy_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function type_alias_is_lazy in module {}", module_path!());
    };
}

mkfn!{
    type_alias_is_lazy_introspect!();
    pub (crate) fn type_alias_is_lazy < 'tcx > (tcx : TyCtxt < 'tcx > , def_id : LocalDefId) -> bool { use hir :: intravisit :: Visitor ; if tcx . features () . lazy_type_alias () { return true ; } struct HasTait ; impl < 'tcx > Visitor < 'tcx > for HasTait { type Result = ControlFlow < () > ; fn visit_ty (& mut self , t : & 'tcx hir :: Ty < 'tcx , AmbigArg >) -> Self :: Result { if let hir :: TyKind :: OpaqueDef (..) = t . kind { ControlFlow :: Break (()) } else { hir :: intravisit :: walk_ty (self , t) } } } HasTait . visit_ty_unambig (tcx . hir_expect_item (def_id) . expect_ty_alias () . 2) . is_break () }
}