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
mkuse!{use rustc_ast :: ast :: ParamKindOrd ;}
mkuse!{use rustc_errors :: codes :: * ;}
mkuse!{use rustc_errors :: { Applicability , Diag , ErrorGuaranteed , MultiSpan , struct_span_code_err } ;}
mkuse!{use rustc_hir :: def :: { DefKind , Res } ;}
mkuse!{use rustc_hir :: def_id :: DefId ;}
mkuse!{use rustc_hir :: { self as hir , GenericArg } ;}
mkuse!{use rustc_middle :: ty :: { self , GenericArgsRef , GenericParamDef , GenericParamDefKind , IsSuggestable , Ty , } ;}
mkuse!{use rustc_session :: lint :: builtin :: LATE_BOUND_LIFETIME_ARGUMENTS ;}
mkuse!{use rustc_span :: kw ;}
mkuse!{use smallvec :: SmallVec ;}
mkuse!{use tracing :: { debug , instrument } ;}
mkuse!{use super :: { HirTyLowerer , IsMethodCall } ;}
mkuse!{use crate :: errors :: wrong_number_of_generic_args :: { GenericArgsInfo , WrongNumberOfGenericArgs } ;}
mkuse!{use crate :: hir_ty_lowering :: errors :: prohibit_assoc_item_constraint ;}
mkuse!{use crate :: hir_ty_lowering :: { ExplicitLateBound , GenericArgCountMismatch , GenericArgCountResult , GenericArgPosition , GenericArgsLowerer , } ;}

macro_rules! generic_arg_mismatch_err_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function generic_arg_mismatch_err in module {}", module_path!());
    };
}

mkfn!{
    generic_arg_mismatch_err_introspect!();
    # [doc = " Report an error that a generic argument did not match the generic parameter that was"] # [doc = " expected."] fn generic_arg_mismatch_err (cx : & dyn HirTyLowerer < '_ > , arg : & GenericArg < '_ > , param : & GenericParamDef , possible_ordering_error : bool , help : Option < String > ,) -> ErrorGuaranteed { let tcx = cx . tcx () ; let sess = tcx . sess ; let mut err = struct_span_code_err ! (cx . dcx () , arg . span () , E0747 , "{} provided when a {} was expected" , arg . descr () , param . kind . descr () ,) ; let add_braces_suggestion = | arg : & GenericArg < '_ > , err : & mut Diag < '_ > | { let suggestions = vec ! [(arg . span () . shrink_to_lo () , String :: from ("{ ")) , (arg . span () . shrink_to_hi () , String :: from (" }")) ,] ; err . multipart_suggestion ("if this generic argument was intended as a const parameter, \
                 surround it with braces" , suggestions , Applicability :: MaybeIncorrect ,) ; } ; match (arg , & param . kind) { (GenericArg :: Type (hir :: Ty { kind : hir :: TyKind :: Path (rustc_hir :: QPath :: Resolved (_ , path)) , .. }) , GenericParamDefKind :: Const { .. } ,) => match path . res { Res :: Err => { add_braces_suggestion (arg , & mut err) ; return err . with_primary_message ("unresolved item provided when a constant was expected") . emit () ; } Res :: Def (DefKind :: TyParam , src_def_id) => { if let Some (param_local_id) = param . def_id . as_local () { let param_name = tcx . hir_ty_param_name (param_local_id) ; let param_type = tcx . type_of (param . def_id) . instantiate_identity () ; if param_type . is_suggestable (tcx , false) { err . span_suggestion_verbose (tcx . def_span (src_def_id) , "consider changing this type parameter to a const parameter" , format ! ("const {param_name}: {param_type}") , Applicability :: MaybeIncorrect ,) ; } ; } } _ => add_braces_suggestion (arg , & mut err) , } , (GenericArg :: Type (hir :: Ty { kind : hir :: TyKind :: Path (_) , .. }) , GenericParamDefKind :: Const { .. } ,) => add_braces_suggestion (arg , & mut err) , (GenericArg :: Type (hir :: Ty { kind : hir :: TyKind :: Array (_ , len) , .. }) , GenericParamDefKind :: Const { .. } ,) if tcx . type_of (param . def_id) . skip_binder () == tcx . types . usize => { let snippet = sess . source_map () . span_to_snippet (tcx . hir_span (len . hir_id)) ; if let Ok (snippet) = snippet { err . span_suggestion (arg . span () , "array type provided where a `usize` was expected, try" , format ! ("{{ {snippet} }}") , Applicability :: MaybeIncorrect ,) ; } } (GenericArg :: Const (cnst) , GenericParamDefKind :: Type { .. }) => { if let hir :: ConstArgKind :: Path (qpath) = cnst . kind && let rustc_hir :: QPath :: Resolved (_ , path) = qpath && let Res :: Def (DefKind :: Fn { .. } , id) = path . res { err . help (format ! ("`{}` is a function item, not a type" , tcx . item_name (id))) ; err . help ("function item types cannot be named directly") ; } else if let hir :: ConstArgKind :: Anon (anon) = cnst . kind && let body = tcx . hir_body (anon . body) && let rustc_hir :: ExprKind :: Path (rustc_hir :: QPath :: Resolved (_ , path)) = body . value . kind && let Res :: Def (DefKind :: Fn { .. } , id) = path . res { err . help (format ! ("`{}` is a function item, not a type" , tcx . item_name (id))) ; err . help ("function item types cannot be named directly") ; } } _ => { } } let kind_ord = param . kind . to_ord () ; let arg_ord = arg . to_ord () ; if possible_ordering_error && kind_ord . cmp (& arg_ord) != core :: cmp :: Ordering :: Equal { let (first , last) = if kind_ord < arg_ord { (param . kind . descr () , arg . descr ()) } else { (arg . descr () , param . kind . descr ()) } ; err . note (format ! ("{first} arguments must be provided before {last} arguments")) ; if let Some (help) = help { err . help (help) ; } } err . emit () }
}

macro_rules! lower_generic_args_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lower_generic_args in module {}", module_path!());
    };
}

mkfn!{
    lower_generic_args_introspect!();
    # [doc = " Lower generic arguments from the HIR to the [`rustc_middle::ty`] representation."] # [doc = ""] # [doc = " This is a rather complex function. Let us try to explain the role"] # [doc = " of each of its parameters:"] # [doc = ""] # [doc = " To start, we are given the `def_id` of the thing whose generic parameters we"] # [doc = " are creating, and a partial set of arguments `parent_args`. In general,"] # [doc = " the generic arguments for an item begin with arguments for all the \"parents\""] # [doc = " of that item -- e.g., for a method it might include the parameters from the impl."] # [doc = ""] # [doc = " Therefore, the method begins by walking down these parents,"] # [doc = " starting with the outermost parent and proceed inwards until"] # [doc = " it reaches `def_id`. For each parent `P`, it will check `parent_args`"] # [doc = " first to see if the parent's arguments are listed in there. If so,"] # [doc = " we can append those and move on. Otherwise, it uses the provided"] # [doc = " [`GenericArgsLowerer`] `ctx` which has the following methods:"] # [doc = ""] # [doc = " - `args_for_def_id`: given the `DefId` `P`, supplies back the"] # [doc = "   generic arguments that were given to that parent from within"] # [doc = "   the path; so e.g., if you have `<T as Foo>::Bar`, the `DefId`"] # [doc = "   might refer to the trait `Foo`, and the arguments might be"] # [doc = "   `[T]`. The boolean value indicates whether to infer values"] # [doc = "   for arguments whose values were not explicitly provided."] # [doc = " - `provided_kind`: given the generic parameter and the value"] # [doc = "   from `args_for_def_id`, creating a `GenericArg`."] # [doc = " - `inferred_kind`: if no parameter was provided, and inference"] # [doc = "   is enabled, then creates a suitable inference variable."] pub fn lower_generic_args < 'tcx : 'a , 'a > (cx : & dyn HirTyLowerer < 'tcx > , def_id : DefId , parent_args : & [ty :: GenericArg < 'tcx >] , has_self : bool , self_ty : Option < Ty < 'tcx > > , arg_count : & GenericArgCountResult , ctx : & mut impl GenericArgsLowerer < 'a , 'tcx > ,) -> GenericArgsRef < 'tcx > { let tcx = cx . tcx () ; let mut parent_defs = tcx . generics_of (def_id) ; let count = parent_defs . count () ; let mut stack = vec ! [(def_id , parent_defs)] ; while let Some (def_id) = parent_defs . parent { parent_defs = tcx . generics_of (def_id) ; stack . push ((def_id , parent_defs)) ; } let mut args : SmallVec < [ty :: GenericArg < 'tcx > ; 8] > = SmallVec :: with_capacity (count) ; while let Some ((def_id , defs)) = stack . pop () { let mut params = defs . own_params . iter () . peekable () ; while let Some (& param) = params . peek () { if let Some (& kind) = parent_args . get (param . index as usize) { args . push (kind) ; params . next () ; } else { break ; } } if has_self { if let Some (& param) = params . peek () { if param . index == 0 { if let GenericParamDefKind :: Type { .. } = param . kind { assert_eq ! (& args [..] , & []) ; args . push (self_ty . map (| ty | ty . into ()) . unwrap_or_else (| | ctx . inferred_kind (& args , param , true)) ,) ; params . next () ; } } } } let (generic_args , infer_args) = ctx . args_for_def_id (def_id) ; let mut args_iter = generic_args . iter () . flat_map (| generic_args | generic_args . args . iter ()) . peekable () ; let mut force_infer_lt = None ; loop { match (args_iter . peek () , params . peek ()) { (Some (& arg) , Some (& param)) => { match (arg , & param . kind , arg_count . explicit_late_bound) { (GenericArg :: Lifetime (_) , GenericParamDefKind :: Lifetime , _) | (GenericArg :: Type (_) | GenericArg :: Infer (_) , GenericParamDefKind :: Type { .. } , _ ,) | (GenericArg :: Const (_) | GenericArg :: Infer (_) , GenericParamDefKind :: Const { .. } , _ ,) => { args . push (ctx . provided_kind (& args , param , arg)) ; args_iter . next () ; params . next () ; } (GenericArg :: Infer (_) | GenericArg :: Type (_) | GenericArg :: Const (_) , GenericParamDefKind :: Lifetime , _ ,) => { args . push (ctx . inferred_kind (& args , param , infer_args)) ; force_infer_lt = Some ((arg , param)) ; params . next () ; } (GenericArg :: Lifetime (_) , _ , ExplicitLateBound :: Yes) => { args_iter . next () ; } (_ , _ , _) => { if arg_count . correct . is_ok () { let mut param_types_present = defs . own_params . iter () . map (| param | (param . kind . to_ord () , param . clone ())) . collect :: < Vec < (ParamKindOrd , GenericParamDef) > > () ; param_types_present . sort_by_key (| (ord , _) | * ord) ; let (mut param_types_present , ordered_params) : (Vec < ParamKindOrd > , Vec < GenericParamDef > ,) = param_types_present . into_iter () . unzip () ; param_types_present . dedup () ; generic_arg_mismatch_err (cx , arg , param , ! args_iter . clone () . is_sorted_by_key (| arg | arg . to_ord ()) , Some (format ! ("reorder the arguments: {}: `<{}>`" , param_types_present . into_iter () . map (| ord | format ! ("{ord}s")) . collect ::< Vec < String >> () . join (", then ") , ordered_params . into_iter () . filter_map (| param | { if param . name == kw :: SelfUpper { None } else { Some (param . name . to_string ()) } }) . collect ::< Vec < String >> () . join (", "))) ,) ; } while args_iter . next () . is_some () { } } } } (Some (& arg) , None) => { if arg_count . correct . is_ok () && arg_count . explicit_late_bound == ExplicitLateBound :: No { let kind = arg . descr () ; assert_eq ! (kind , "lifetime") ; let (provided_arg , param) = force_infer_lt . expect ("lifetimes ought to have been inferred") ; generic_arg_mismatch_err (cx , provided_arg , param , false , None) ; } break ; } (None , Some (& param)) => { args . push (ctx . inferred_kind (& args , param , infer_args)) ; params . next () ; } (None , None) => break , } } } tcx . mk_args (& args) }
}

macro_rules! check_generic_arg_count_for_call_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_generic_arg_count_for_call in module {}", module_path!());
    };
}

mkfn!{
    check_generic_arg_count_for_call_introspect!();
    # [doc = " Checks that the correct number of generic arguments have been provided."] # [doc = " Used specifically for function calls."] pub fn check_generic_arg_count_for_call (cx : & dyn HirTyLowerer < '_ > , def_id : DefId , generics : & ty :: Generics , seg : & hir :: PathSegment < '_ > , is_method_call : IsMethodCall ,) -> GenericArgCountResult { let gen_pos = match is_method_call { IsMethodCall :: Yes => GenericArgPosition :: MethodCall , IsMethodCall :: No => GenericArgPosition :: Value , } ; let has_self = generics . parent . is_none () && generics . has_self ; check_generic_arg_count (cx , def_id , seg , generics , gen_pos , has_self) }
}

macro_rules! check_generic_arg_count_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_generic_arg_count in module {}", module_path!());
    };
}

mkfn!{
    check_generic_arg_count_introspect!();
    # [doc = " Checks that the correct number of generic arguments have been provided."] # [doc = " This is used both for datatypes and function calls."] # [instrument (skip (cx , gen_pos) , level = "debug")] pub (crate) fn check_generic_arg_count (cx : & dyn HirTyLowerer < '_ > , def_id : DefId , seg : & hir :: PathSegment < '_ > , gen_params : & ty :: Generics , gen_pos : GenericArgPosition , has_self : bool ,) -> GenericArgCountResult { let gen_args = seg . args () ; let default_counts = gen_params . own_defaults () ; let param_counts = gen_params . own_counts () ; let synth_type_param_count = gen_params . own_params . iter () . filter (| param | matches ! (param . kind , ty :: GenericParamDefKind :: Type { synthetic : true , .. })) . count () ; let named_type_param_count = param_counts . types - has_self as usize - synth_type_param_count ; let named_const_param_count = param_counts . consts ; let infer_lifetimes = (gen_pos != GenericArgPosition :: Type || seg . infer_args) && ! gen_args . has_lifetime_params () ; if gen_pos != GenericArgPosition :: Type && let Some (c) = gen_args . constraints . first () { prohibit_assoc_item_constraint (cx , c , None) ; } let explicit_late_bound = prohibit_explicit_late_bound_lifetimes (cx , gen_params , gen_args , gen_pos) ; let mut invalid_args = vec ! [] ; let mut check_lifetime_args = | min_expected_args : usize , max_expected_args : usize , provided_args : usize , late_bounds_ignore : bool | { if (min_expected_args ..= max_expected_args) . contains (& provided_args) { return Ok (()) ; } if late_bounds_ignore { return Ok (()) ; } invalid_args . extend (min_expected_args .. provided_args) ; let gen_args_info = if provided_args > min_expected_args { let num_redundant_args = provided_args - min_expected_args ; GenericArgsInfo :: ExcessLifetimes { num_redundant_args } } else { let num_missing_args = min_expected_args - provided_args ; GenericArgsInfo :: MissingLifetimes { num_missing_args } } ; let reported = cx . dcx () . emit_err (WrongNumberOfGenericArgs :: new (cx . tcx () , gen_args_info , seg , gen_params , has_self as usize , gen_args , def_id ,)) ; Err (reported) } ; let min_expected_lifetime_args = if infer_lifetimes { 0 } else { param_counts . lifetimes } ; let max_expected_lifetime_args = param_counts . lifetimes ; let num_provided_lifetime_args = gen_args . num_lifetime_params () ; let lifetimes_correct = check_lifetime_args (min_expected_lifetime_args , max_expected_lifetime_args , num_provided_lifetime_args , explicit_late_bound == ExplicitLateBound :: Yes ,) ; let mut check_types_and_consts = | expected_min , expected_max , expected_max_with_synth , provided , params_offset , args_offset | { debug ! (? expected_min , ? expected_max , ? provided , ? params_offset , ? args_offset , "check_types_and_consts") ; if (expected_min ..= expected_max) . contains (& provided) { return Ok (()) ; } let num_default_params = expected_max - expected_min ; let mut all_params_are_binded = false ; let gen_args_info = if provided > expected_max { invalid_args . extend ((expected_max .. provided) . map (| i | i + args_offset)) ; let num_redundant_args = provided - expected_max ; let synth_provided = provided <= expected_max_with_synth ; GenericArgsInfo :: ExcessTypesOrConsts { num_redundant_args , num_default_params , args_offset , synth_provided , } } else { let parent_is_impl_block = cx . tcx () . hir_parent_owner_iter (seg . hir_id) . next () . is_some_and (| (_ , owner_node) | owner_node . is_impl_block ()) ; if parent_is_impl_block { let constraint_names : Vec < _ > = gen_args . constraints . iter () . map (| b | b . ident . name) . collect () ; let param_names : Vec < _ > = gen_params . own_params . iter () . filter (| param | ! has_self || param . index != 0) . map (| param | param . name) . collect () ; if constraint_names == param_names { all_params_are_binded = true ; } ; } let num_missing_args = expected_max - provided ; GenericArgsInfo :: MissingTypesOrConsts { num_missing_args , num_default_params , args_offset , } } ; debug ! (? gen_args_info) ; let reported = gen_args . has_err () . unwrap_or_else (| | { cx . dcx () . create_err (WrongNumberOfGenericArgs :: new (cx . tcx () , gen_args_info , seg , gen_params , params_offset , gen_args , def_id ,)) . emit_unless_delay (all_params_are_binded) }) ; Err (reported) } ; let args_correct = { let expected_min = if seg . infer_args { 0 } else { param_counts . consts + named_type_param_count - default_counts . types - default_counts . consts } ; debug ! (? expected_min) ; debug ! (arg_counts . lifetimes =? gen_args . num_lifetime_params ()) ; let provided = gen_args . num_generic_params () ; check_types_and_consts (expected_min , named_const_param_count + named_type_param_count , named_const_param_count + named_type_param_count + synth_type_param_count , provided , param_counts . lifetimes + has_self as usize , gen_args . num_lifetime_params () ,) } ; GenericArgCountResult { explicit_late_bound , correct : lifetimes_correct . and (args_correct) . map_err (| reported | GenericArgCountMismatch { reported , invalid_args }) , } }
}

macro_rules! prohibit_explicit_late_bound_lifetimes_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function prohibit_explicit_late_bound_lifetimes in module {}", module_path!());
    };
}

mkfn!{
    prohibit_explicit_late_bound_lifetimes_introspect!();
    # [doc = " Prohibits explicit lifetime arguments if late-bound lifetime parameters"] # [doc = " are present. This is used both for datatypes and function calls."] pub (crate) fn prohibit_explicit_late_bound_lifetimes (cx : & dyn HirTyLowerer < '_ > , def : & ty :: Generics , args : & hir :: GenericArgs < '_ > , position : GenericArgPosition ,) -> ExplicitLateBound { let param_counts = def . own_counts () ; let infer_lifetimes = position != GenericArgPosition :: Type && ! args . has_lifetime_params () ; if infer_lifetimes { return ExplicitLateBound :: No ; } if let Some (span_late) = def . has_late_bound_regions { let msg = "cannot specify lifetime arguments explicitly \
                       if late bound lifetime parameters are present" ; let note = "the late bound lifetime parameter is introduced here" ; let span = args . args [0] . span () ; if position == GenericArgPosition :: Value && args . num_lifetime_params () != param_counts . lifetimes { struct_span_code_err ! (cx . dcx () , span , E0794 , "{}" , msg) . with_span_note (span_late , note) . emit () ; } else { let mut multispan = MultiSpan :: from_span (span) ; multispan . push_span_label (span_late , note) ; cx . tcx () . node_span_lint (LATE_BOUND_LIFETIME_ARGUMENTS , args . args [0] . hir_id () , multispan , | lint | { lint . primary_message (msg) ; } ,) ; } ExplicitLateBound :: Yes } else { ExplicitLateBound :: No } }
}