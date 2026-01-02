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
mkuse!{use std :: sync :: Arc ;}
mkuse!{use rustc_ast :: { self as ast , * } ;}
mkuse!{use rustc_hir :: def :: { DefKind , PartialRes , PerNS , Res } ;}
mkuse!{use rustc_hir :: def_id :: DefId ;}
mkuse!{use rustc_hir :: { self as hir , GenericArg } ;}
mkuse!{use rustc_middle :: { span_bug , ty } ;}
mkuse!{use rustc_session :: parse :: add_feature_diagnostics ;}
mkuse!{use rustc_span :: { BytePos , DUMMY_SP , DesugaringKind , Ident , Span , Symbol , sym } ;}
mkuse!{use smallvec :: smallvec ;}
mkuse!{use tracing :: { debug , instrument } ;}
mkuse!{use super :: errors :: { AsyncBoundNotOnTrait , AsyncBoundOnlyForFnTraits , BadReturnTypeNotation , GenericTypeWithParentheses , RTNSuggestion , UseAngleBrackets , } ;}
mkuse!{use super :: { AllowReturnTypeNotation , GenericArgsCtor , GenericArgsMode , ImplTraitContext , ImplTraitPosition , LifetimeRes , LoweringContext , ParamMode , ResolverAstLoweringExt , } ;}
mkitem!{mkimpl!{impl < 'a , 'hir > LoweringContext < 'a , 'hir > { # [instrument (level = "trace" , skip (self))] pub (crate) fn lower_qpath (& mut self , id : NodeId , qself : & Option < Box < QSelf > > , p : & Path , param_mode : ParamMode , allow_return_type_notation : AllowReturnTypeNotation , itctx : ImplTraitContext , modifiers : Option < ast :: TraitBoundModifiers > ,) -> hir :: QPath < 'hir > { let qself_position = qself . as_ref () . map (| q | q . position) ; let qself = qself . as_ref () . map (| q | self . lower_ty (& q . ty , ImplTraitContext :: Disallowed (ImplTraitPosition :: Path))) ; let partial_res = self . resolver . get_partial_res (id) . unwrap_or_else (| | PartialRes :: new (Res :: Err)) ; let base_res = partial_res . base_res () ; let unresolved_segments = partial_res . unresolved_segments () ; let mut res = self . lower_res (base_res) ; if let Some (TraitBoundModifiers { asyncness : BoundAsyncness :: Async (_) , .. }) = modifiers { match res { Res :: Def (DefKind :: Trait , def_id) => { if let Some (async_def_id) = self . map_trait_to_async_trait (def_id) { res = Res :: Def (DefKind :: Trait , async_def_id) ; } else { self . dcx () . emit_err (AsyncBoundOnlyForFnTraits { span : p . span }) ; } } Res :: Err => { } _ => { self . dcx () . emit_err (AsyncBoundNotOnTrait { span : p . span , descr : res . descr () }) ; } } } let bound_modifier_allowed_features = if let Res :: Def (DefKind :: Trait , async_def_id) = res && self . tcx . async_fn_trait_kind_from_def_id (async_def_id) . is_some () { Some (Arc :: clone (& self . allow_async_fn_traits)) } else { None } ; let itctx = | i | { if i + 1 == p . segments . len () { itctx } else { ImplTraitContext :: Disallowed (ImplTraitPosition :: Path) } } ; let path_span_lo = p . span . shrink_to_lo () ; let proj_start = p . segments . len () - unresolved_segments ; let path = self . arena . alloc (hir :: Path { res , segments : self . arena . alloc_from_iter (p . segments [.. proj_start] . iter () . enumerate () . map (| (i , segment) | { let param_mode = match (qself_position , param_mode) { (Some (j) , ParamMode :: Optional) if i < j => { ParamMode :: Explicit } _ => param_mode , } ; let generic_args_mode = match base_res { Res :: Def (DefKind :: Trait , _) if i + 1 == proj_start => { GenericArgsMode :: ParenSugar } Res :: Def (DefKind :: AssocFn , _) | Res :: Def (DefKind :: AssocConst , _) | Res :: Def (DefKind :: AssocTy , _) if i + 2 == proj_start => { GenericArgsMode :: ParenSugar } Res :: Def (DefKind :: AssocFn , _) if i + 1 == proj_start => { match allow_return_type_notation { AllowReturnTypeNotation :: Yes => GenericArgsMode :: ReturnTypeNotation , AllowReturnTypeNotation :: No => GenericArgsMode :: Err , } } Res :: Err => GenericArgsMode :: Silence , _ => GenericArgsMode :: Err , } ; self . lower_path_segment (p . span , segment , param_mode , generic_args_mode , itctx (i) , bound_modifier_allowed_features . clone () ,) } ,)) , span : self . lower_span (p . segments [.. proj_start] . last () . map_or (path_span_lo , | segment | path_span_lo . to (segment . span ())) ,) , }) ; if let Some (bound_modifier_allowed_features) = bound_modifier_allowed_features { path . span = self . mark_span_with_reason (DesugaringKind :: BoundModifier , path . span , Some (bound_modifier_allowed_features) ,) ; } if unresolved_segments == 0 { return hir :: QPath :: Resolved (qself , path) ; } let mut ty = if path . segments . is_empty () { qself . expect ("missing QSelf for <T>::...") } else { let new_id = self . next_id () ; self . arena . alloc (self . ty_path (new_id , path . span , hir :: QPath :: Resolved (qself , path))) } ; for (i , segment) in p . segments . iter () . enumerate () . skip (proj_start) { let generic_args_mode = if i + 1 == p . segments . len () && matches ! (allow_return_type_notation , AllowReturnTypeNotation :: Yes) { GenericArgsMode :: ReturnTypeNotation } else { GenericArgsMode :: Err } ; let hir_segment = self . arena . alloc (self . lower_path_segment (p . span , segment , param_mode , generic_args_mode , itctx (i) , None ,)) ; let qpath = hir :: QPath :: TypeRelative (ty , hir_segment) ; if i == p . segments . len () - 1 { return qpath ; } let new_id = self . next_id () ; ty = self . arena . alloc (self . ty_path (new_id , path_span_lo . to (segment . span ()) , qpath)) ; } self . dcx () . span_bug (p . span , format ! ("lower_qpath: no final extension segment in {}..{}" , proj_start , p . segments . len ()) ,) ; } pub (crate) fn lower_use_path (& mut self , res : PerNS < Option < Res > > , p : & Path , param_mode : ParamMode ,) -> & 'hir hir :: UsePath < 'hir > { assert ! (! res . is_empty ()) ; self . arena . alloc (hir :: UsePath { res , segments : self . arena . alloc_from_iter (p . segments . iter () . map (| segment | { self . lower_path_segment (p . span , segment , param_mode , GenericArgsMode :: Err , ImplTraitContext :: Disallowed (ImplTraitPosition :: Path) , None ,) })) , span : self . lower_span (p . span) , }) } pub (crate) fn lower_path_segment (& mut self , path_span : Span , segment : & PathSegment , param_mode : ParamMode , generic_args_mode : GenericArgsMode , itctx : ImplTraitContext , bound_modifier_allowed_features : Option < Arc < [Symbol] > > ,) -> hir :: PathSegment < 'hir > { debug ! ("path_span: {:?}, lower_path_segment(segment: {:?})" , path_span , segment) ; let (mut generic_args , infer_args) = if let Some (generic_args) = segment . args . as_deref () { match generic_args { GenericArgs :: AngleBracketed (data) => { self . lower_angle_bracketed_parameter_data (data , param_mode , itctx) } GenericArgs :: Parenthesized (data) => match generic_args_mode { GenericArgsMode :: ReturnTypeNotation => { let err = match (& data . inputs [..] , & data . output) { ([_ , ..] , FnRetTy :: Default (_)) => { BadReturnTypeNotation :: Inputs { span : data . inputs_span } } ([] , FnRetTy :: Default (_)) => { BadReturnTypeNotation :: NeedsDots { span : data . inputs_span } } (_ , FnRetTy :: Ty (ty)) => { let span = data . inputs_span . shrink_to_hi () . to (ty . span) ; BadReturnTypeNotation :: Output { span , suggestion : RTNSuggestion { output : span , input : data . inputs_span , } , } } } ; let mut err = self . dcx () . create_err (err) ; if ! self . tcx . features () . return_type_notation () && self . tcx . sess . is_nightly_build () { add_feature_diagnostics (& mut err , & self . tcx . sess , sym :: return_type_notation ,) ; } err . emit () ; (GenericArgsCtor { args : Default :: default () , constraints : & [] , parenthesized : hir :: GenericArgsParentheses :: ReturnTypeNotation , span : path_span , } , false ,) } GenericArgsMode :: ParenSugar | GenericArgsMode :: Silence => self . lower_parenthesized_parameter_data (data , itctx , bound_modifier_allowed_features ,) , GenericArgsMode :: Err => { let sub = if ! data . inputs . is_empty () { let open_param = data . inputs_span . shrink_to_lo () . to (data . inputs . first () . unwrap () . span . shrink_to_lo ()) ; let close_param = data . inputs . last () . unwrap () . span . shrink_to_hi () . to (data . inputs_span . shrink_to_hi ()) ; Some (UseAngleBrackets { open_param , close_param }) } else { None } ; self . dcx () . emit_err (GenericTypeWithParentheses { span : data . span , sub }) ; (self . lower_angle_bracketed_parameter_data (& data . as_angle_bracketed_args () , param_mode , itctx ,) . 0 , false ,) } } , GenericArgs :: ParenthesizedElided (span) => { match generic_args_mode { GenericArgsMode :: ReturnTypeNotation | GenericArgsMode :: Silence => { } GenericArgsMode :: ParenSugar | GenericArgsMode :: Err => { self . dcx () . emit_err (BadReturnTypeNotation :: Position { span : * span }) ; } } (GenericArgsCtor { args : Default :: default () , constraints : & [] , parenthesized : hir :: GenericArgsParentheses :: ReturnTypeNotation , span : * span , } , false ,) } } } else { (GenericArgsCtor { args : Default :: default () , constraints : & [] , parenthesized : hir :: GenericArgsParentheses :: No , span : path_span . shrink_to_hi () , } , param_mode == ParamMode :: Optional ,) } ; let has_lifetimes = generic_args . args . iter () . any (| arg | matches ! (arg , GenericArg :: Lifetime (_))) ; if generic_args . parenthesized != hir :: GenericArgsParentheses :: ParenSugar && ! has_lifetimes { self . maybe_insert_elided_lifetimes_in_path (path_span , segment . id , segment . ident . span , & mut generic_args ,) ; } let res = self . expect_full_res (segment . id) ; let hir_id = self . lower_node_id (segment . id) ; debug ! ("lower_path_segment: ident={:?} original-id={:?} new-id={:?}" , segment . ident , segment . id , hir_id ,) ; hir :: PathSegment { ident : self . lower_ident (segment . ident) , hir_id , res : self . lower_res (res) , infer_args , args : if generic_args . is_empty () && generic_args . span . is_empty () { None } else { Some (generic_args . into_generic_args (self)) } , } } fn maybe_insert_elided_lifetimes_in_path (& mut self , path_span : Span , segment_id : NodeId , segment_ident_span : Span , generic_args : & mut GenericArgsCtor < 'hir > ,) { let (start , end) = match self . resolver . get_lifetime_res (segment_id) { Some (LifetimeRes :: ElidedAnchor { start , end }) => (start , end) , None => return , Some (res) => { span_bug ! (path_span , "expected an elided lifetime to insert. found {res:?}") } } ; let expected_lifetimes = end . as_usize () - start . as_usize () ; debug ! (expected_lifetimes) ; let (elided_lifetime_span , angle_brackets) = if generic_args . span . is_empty () { (segment_ident_span . find_ancestor_inside (path_span) . unwrap_or (path_span) , hir :: AngleBrackets :: Missing ,) } else { (generic_args . span . with_lo (generic_args . span . lo () + BytePos (1)) . shrink_to_lo () , if generic_args . is_empty () { hir :: AngleBrackets :: Empty } else { hir :: AngleBrackets :: Full } ,) } ; generic_args . args . insert_many (0 , (start .. end) . map (| id | { let l = self . lower_lifetime_hidden_in_path (id , elided_lifetime_span , angle_brackets) ; GenericArg :: Lifetime (l) }) ,) ; } pub (crate) fn lower_angle_bracketed_parameter_data (& mut self , data : & AngleBracketedArgs , param_mode : ParamMode , itctx : ImplTraitContext ,) -> (GenericArgsCtor < 'hir > , bool) { let has_non_lt_args = data . args . iter () . any (| arg | match arg { AngleBracketedArg :: Arg (ast :: GenericArg :: Lifetime (_)) | AngleBracketedArg :: Constraint (_) => false , AngleBracketedArg :: Arg (ast :: GenericArg :: Type (_) | ast :: GenericArg :: Const (_)) => true , }) ; let args = data . args . iter () . filter_map (| arg | match arg { AngleBracketedArg :: Arg (arg) => Some (self . lower_generic_arg (arg , itctx)) , AngleBracketedArg :: Constraint (_) => None , }) . collect () ; let constraints = self . arena . alloc_from_iter (data . args . iter () . filter_map (| arg | match arg { AngleBracketedArg :: Constraint (c) => { Some (self . lower_assoc_item_constraint (c , itctx)) } AngleBracketedArg :: Arg (_) => None , })) ; let ctor = GenericArgsCtor { args , constraints , parenthesized : hir :: GenericArgsParentheses :: No , span : data . span , } ; (ctor , ! has_non_lt_args && param_mode == ParamMode :: Optional) } fn lower_parenthesized_parameter_data (& mut self , data : & ParenthesizedArgs , itctx : ImplTraitContext , bound_modifier_allowed_features : Option < Arc < [Symbol] > > ,) -> (GenericArgsCtor < 'hir > , bool) { let ParenthesizedArgs { span , inputs , inputs_span , output } = data ; let inputs = self . arena . alloc_from_iter (inputs . iter () . map (| ty | { self . lower_ty_direct (ty , ImplTraitContext :: Disallowed (ImplTraitPosition :: FnTraitParam)) })) ; let output_ty = match output { FnRetTy :: Ty (ty) if matches ! (itctx , ImplTraitContext :: OpaqueTy { .. }) => { if self . tcx . features () . impl_trait_in_fn_trait_return () { self . lower_ty (ty , itctx) } else { self . lower_ty (ty , ImplTraitContext :: FeatureGated (ImplTraitPosition :: FnTraitReturn , sym :: impl_trait_in_fn_trait_return ,) ,) } } FnRetTy :: Ty (ty) => { self . lower_ty (ty , ImplTraitContext :: Disallowed (ImplTraitPosition :: FnTraitReturn)) } FnRetTy :: Default (_) => self . arena . alloc (self . ty_tup (* span , & [])) , } ; let args = smallvec ! [GenericArg :: Type (self . arena . alloc (self . ty_tup (* inputs_span , inputs)) . try_as_ambig_ty () . unwrap ())] ; let mut output_span = output_ty . span ; if let Some (bound_modifier_allowed_features) = bound_modifier_allowed_features { output_span = self . mark_span_with_reason (DesugaringKind :: BoundModifier , output_span , Some (bound_modifier_allowed_features) ,) ; } let constraint = self . assoc_ty_binding (sym :: Output , output_span , output_ty) ; (GenericArgsCtor { args , constraints : arena_vec ! [self ; constraint] , parenthesized : hir :: GenericArgsParentheses :: ParenSugar , span : data . inputs_span , } , false ,) } # [doc = " An associated type binding (i.e., associated type equality constraint)."] pub (crate) fn assoc_ty_binding (& mut self , assoc_ty_name : rustc_span :: Symbol , span : Span , ty : & 'hir hir :: Ty < 'hir > ,) -> hir :: AssocItemConstraint < 'hir > { let ident = Ident :: with_dummy_span (assoc_ty_name) ; let kind = hir :: AssocItemConstraintKind :: Equality { term : ty . into () } ; let args = arena_vec ! [self ;] ; let constraints = arena_vec ! [self ;] ; let gen_args = self . arena . alloc (hir :: GenericArgs { args , constraints , parenthesized : hir :: GenericArgsParentheses :: No , span_ext : DUMMY_SP , }) ; hir :: AssocItemConstraint { hir_id : self . next_id () , gen_args , span : self . lower_span (span) , ident , kind , } } # [doc = " When a bound is annotated with `async`, it signals to lowering that the trait"] # [doc = " that the bound refers to should be mapped to the \"async\" flavor of the trait."] # [doc = ""] # [doc = " This only needs to be done until we unify `AsyncFn` and `Fn` traits into one"] # [doc = " that is generic over `async`ness, if that's ever possible, or modify the"] # [doc = " lowering of `async Fn()` bounds to desugar to another trait like `LendingFn`."] fn map_trait_to_async_trait (& self , def_id : DefId) -> Option < DefId > { let lang_items = self . tcx . lang_items () ; match self . tcx . fn_trait_kind_from_def_id (def_id) ? { ty :: ClosureKind :: Fn => lang_items . async_fn_trait () , ty :: ClosureKind :: FnMut => lang_items . async_fn_mut_trait () , ty :: ClosureKind :: FnOnce => lang_items . async_fn_once_trait () , } } }}}