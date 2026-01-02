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
mkuse!{use rustc_arena :: { DroplessArena , TypedArena } ;}
mkuse!{use rustc_ast :: Mutability ;}
mkuse!{use rustc_data_structures :: fx :: FxIndexSet ;}
mkuse!{use rustc_data_structures :: stack :: ensure_sufficient_stack ;}
mkuse!{use rustc_errors :: codes :: * ;}
mkuse!{use rustc_errors :: { Applicability , ErrorGuaranteed , MultiSpan , struct_span_code_err } ;}
mkuse!{use rustc_hir :: def :: * ;}
mkuse!{use rustc_hir :: def_id :: LocalDefId ;}
mkuse!{use rustc_hir :: { self as hir , BindingMode , ByRef , HirId , MatchSource } ;}
mkuse!{use rustc_infer :: infer :: TyCtxtInferExt ;}
mkuse!{use rustc_lint :: Level ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: thir :: visit :: Visitor ;}
mkuse!{use rustc_middle :: thir :: * ;}
mkuse!{use rustc_middle :: ty :: print :: with_no_trimmed_paths ;}
mkuse!{use rustc_middle :: ty :: { self , AdtDef , Ty , TyCtxt } ;}
mkuse!{use rustc_pattern_analysis :: errors :: Uncovered ;}
mkuse!{use rustc_pattern_analysis :: rustc :: { Constructor , DeconstructedPat , MatchArm , RedundancyExplanation , RevealedTy , RustcPatCtxt as PatCtxt , Usefulness , UsefulnessReport , WitnessPat , } ;}
mkuse!{use rustc_session :: lint :: builtin :: { BINDINGS_WITH_VARIANT_NAME , IRREFUTABLE_LET_PATTERNS , UNREACHABLE_PATTERNS , } ;}
mkuse!{use rustc_span :: edit_distance :: find_best_match_for_name ;}
mkuse!{use rustc_span :: hygiene :: DesugaringKind ;}
mkuse!{use rustc_span :: { Ident , Span } ;}
mkuse!{use rustc_trait_selection :: infer :: InferCtxtExt ;}
mkuse!{use tracing :: instrument ;}
mkuse!{use crate :: errors :: * ;}
mkuse!{use crate :: fluent_generated as fluent ;}

macro_rules! check_match_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_match in module {}", module_path!());
    };
}

mkfn!{
    check_match_introspect!();
    pub (crate) fn check_match (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> Result < () , ErrorGuaranteed > { let typeck_results = tcx . typeck (def_id) ; let (thir , expr) = tcx . thir_body (def_id) ? ; let thir = thir . borrow () ; let pattern_arena = TypedArena :: default () ; let dropless_arena = DroplessArena :: default () ; let mut visitor = MatchVisitor { tcx , thir : & * thir , typeck_results , typing_env : ty :: TypingEnv :: non_body_analysis (tcx , def_id) , lint_level : tcx . local_def_id_to_hir_id (def_id) , let_source : LetSource :: None , pattern_arena : & pattern_arena , dropless_arena : & dropless_arena , error : Ok (()) , } ; visitor . visit_expr (& thir [expr]) ; let origin = match tcx . def_kind (def_id) { DefKind :: AssocFn | DefKind :: Fn => "function argument" , DefKind :: Closure => "closure argument" , _ if thir . params . is_empty () => "" , kind => bug ! ("unexpected function parameters in THIR: {kind:?} {def_id:?}") , } ; for param in thir . params . iter () { if let Some (box ref pattern) = param . pat { visitor . check_binding_is_irrefutable (pattern , origin , None , None) ; } } visitor . error }
}
mkitem!{mkenum!{# [derive (Debug , Copy , Clone , PartialEq)] enum RefutableFlag { Irrefutable , Refutable , }}}
mkuse!{use RefutableFlag :: * ;}
mkitem!{mkenum!{# [derive (Clone , Copy , Debug , PartialEq , Eq)] enum LetSource { None , PlainLet , IfLet , IfLetGuard , LetElse , WhileLet , Else , ElseIfLet , }}}
mkitem!{mkstruct!{struct MatchVisitor < 'p , 'tcx > { tcx : TyCtxt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > , typeck_results : & 'tcx ty :: TypeckResults < 'tcx > , thir : & 'p Thir < 'tcx > , lint_level : HirId , let_source : LetSource , pattern_arena : & 'p TypedArena < DeconstructedPat < 'p , 'tcx > > , dropless_arena : & 'p DroplessArena , # [doc = " Tracks if we encountered an error while checking this body. That the first function to"] # [doc = " report it stores it here. Some functions return `Result` to allow callers to short-circuit"] # [doc = " on error, but callers don't need to store it here again."] error : Result < () , ErrorGuaranteed > , }}}
mkitem!{mkimpl!{impl < 'p , 'tcx > Visitor < 'p , 'tcx > for MatchVisitor < 'p , 'tcx > { fn thir (& self) -> & 'p Thir < 'tcx > { self . thir } # [instrument (level = "trace" , skip (self))] fn visit_arm (& mut self , arm : & 'p Arm < 'tcx >) { self . with_lint_level (arm . lint_level , | this | { if let Some (expr) = arm . guard { this . with_let_source (LetSource :: IfLetGuard , | this | { this . visit_expr (& this . thir [expr]) }) ; } this . visit_pat (& arm . pattern) ; this . visit_expr (& self . thir [arm . body]) ; }) ; } # [instrument (level = "trace" , skip (self))] fn visit_expr (& mut self , ex : & 'p Expr < 'tcx >) { match ex . kind { ExprKind :: Scope { value , lint_level , .. } => { self . with_lint_level (lint_level , | this | { this . visit_expr (& this . thir [value]) ; }) ; return ; } ExprKind :: If { cond , then , else_opt , if_then_scope : _ } => { let let_source = match ex . span . desugaring_kind () { Some (DesugaringKind :: WhileLoop) => LetSource :: WhileLet , _ => match self . let_source { LetSource :: Else => LetSource :: ElseIfLet , _ => LetSource :: IfLet , } , } ; self . with_let_source (let_source , | this | this . visit_expr (& self . thir [cond])) ; self . with_let_source (LetSource :: None , | this | { this . visit_expr (& this . thir [then]) ; }) ; if let Some (else_) = else_opt { self . with_let_source (LetSource :: Else , | this | { this . visit_expr (& this . thir [else_]) }) ; } return ; } ExprKind :: Match { scrutinee , box ref arms , match_source } => { self . check_match (scrutinee , arms , match_source , ex . span) ; } ExprKind :: LoopMatch { match_data : box LoopMatchMatchData { scrutinee , box ref arms , span } , .. } => { self . check_match (scrutinee , arms , MatchSource :: Normal , span) ; } ExprKind :: Let { box ref pat , expr } => { self . check_let (pat , Some (expr) , ex . span) ; } ExprKind :: LogicalOp { op : LogicalOp :: And , .. } if ! matches ! (self . let_source , LetSource :: None) => { let mut chain_refutabilities = Vec :: new () ; let Ok (()) = self . visit_land (ex , & mut chain_refutabilities) else { return } ; if chain_refutabilities . iter () . any (| x | x . is_some ()) { self . check_let_chain (chain_refutabilities , ex . span) ; } return ; } _ => { } } ; self . with_let_source (LetSource :: None , | this | visit :: walk_expr (this , ex)) ; } fn visit_stmt (& mut self , stmt : & 'p Stmt < 'tcx >) { match stmt . kind { StmtKind :: Let { box ref pattern , initializer , else_block , lint_level , span , .. } => { self . with_lint_level (lint_level , | this | { let let_source = if else_block . is_some () { LetSource :: LetElse } else { LetSource :: PlainLet } ; this . with_let_source (let_source , | this | { this . check_let (pattern , initializer , span) }) ; visit :: walk_stmt (this , stmt) ; }) ; } StmtKind :: Expr { .. } => { visit :: walk_stmt (self , stmt) ; } } } }}}
mkitem!{mkimpl!{impl < 'p , 'tcx > MatchVisitor < 'p , 'tcx > { # [instrument (level = "trace" , skip (self , f))] fn with_let_source (& mut self , let_source : LetSource , f : impl FnOnce (& mut Self)) { let old_let_source = self . let_source ; self . let_source = let_source ; ensure_sufficient_stack (| | f (self)) ; self . let_source = old_let_source ; } fn with_lint_level < T > (& mut self , new_lint_level : LintLevel , f : impl FnOnce (& mut Self) -> T ,) -> T { if let LintLevel :: Explicit (hir_id) = new_lint_level { let old_lint_level = self . lint_level ; self . lint_level = hir_id ; let ret = f (self) ; self . lint_level = old_lint_level ; ret } else { f (self) } } # [doc = " Visit a nested chain of `&&`. Used for if-let chains. This must call `visit_expr` on the"] # [doc = " subexpressions we are not handling ourselves."] fn visit_land (& mut self , ex : & 'p Expr < 'tcx > , accumulator : & mut Vec < Option < (Span , RefutableFlag) > > ,) -> Result < () , ErrorGuaranteed > { match ex . kind { ExprKind :: Scope { value , lint_level , .. } => self . with_lint_level (lint_level , | this | { this . visit_land (& this . thir [value] , accumulator) }) , ExprKind :: LogicalOp { op : LogicalOp :: And , lhs , rhs } => { let res_lhs = self . visit_land (& self . thir [lhs] , accumulator) ; let res_rhs = self . visit_land_rhs (& self . thir [rhs]) ? ; accumulator . push (res_rhs) ; res_lhs } _ => { let res = self . visit_land_rhs (ex) ? ; accumulator . push (res) ; Ok (()) } } } # [doc = " Visit the right-hand-side of a `&&`. Used for if-let chains. Returns `Some` if the"] # [doc = " expression was ultimately a `let ... = ...`, and `None` if it was a normal boolean"] # [doc = " expression. This must call `visit_expr` on the subexpressions we are not handling ourselves."] fn visit_land_rhs (& mut self , ex : & 'p Expr < 'tcx > ,) -> Result < Option < (Span , RefutableFlag) > , ErrorGuaranteed > { match ex . kind { ExprKind :: Scope { value , lint_level , .. } => { self . with_lint_level (lint_level , | this | this . visit_land_rhs (& this . thir [value])) } ExprKind :: Let { box ref pat , expr } => { let expr = & self . thir () [expr] ; self . with_let_source (LetSource :: None , | this | { this . visit_expr (expr) ; }) ; Ok (Some ((ex . span , self . is_let_irrefutable (pat , Some (expr)) ?))) } _ => { self . with_let_source (LetSource :: None , | this | { this . visit_expr (ex) ; }) ; Ok (None) } } } fn lower_pattern (& mut self , cx : & PatCtxt < 'p , 'tcx > , pat : & 'p Pat < 'tcx > ,) -> Result < & 'p DeconstructedPat < 'p , 'tcx > , ErrorGuaranteed > { if let Err (err) = pat . pat_error_reported () { self . error = Err (err) ; Err (err) } else { let refutable = if cx . refutable { Refutable } else { Irrefutable } ; let mut err = Ok (()) ; pat . walk_always (| pat | { check_borrow_conflicts_in_at_patterns (self , pat) ; check_for_bindings_named_same_as_variants (self , pat , refutable) ; err = err . and (check_never_pattern (cx , pat)) ; }) ; err ? ; Ok (self . pattern_arena . alloc (cx . lower_pat (pat))) } } # [doc = " Inspects the match scrutinee expression to determine whether the place it evaluates to may"] # [doc = " hold invalid data."] fn is_known_valid_scrutinee (& self , scrutinee : & Expr < 'tcx >) -> bool { use ExprKind :: * ; match & scrutinee . kind { Deref { .. } => false , Field { lhs , .. } => { let lhs = & self . thir () [* lhs] ; match lhs . ty . kind () { ty :: Adt (def , _) if def . is_union () => false , _ => self . is_known_valid_scrutinee (lhs) , } } Index { lhs , .. } => { let lhs = & self . thir () [* lhs] ; self . is_known_valid_scrutinee (lhs) } Scope { value , .. } => self . is_known_valid_scrutinee (& self . thir () [* value]) , NeverToAny { source } | Cast { source } | Use { source } | PointerCoercion { source , .. } | PlaceTypeAscription { source , .. } | ValueTypeAscription { source , .. } | PlaceUnwrapUnsafeBinder { source } | ValueUnwrapUnsafeBinder { source } | WrapUnsafeBinder { source } => self . is_known_valid_scrutinee (& self . thir () [* source]) , Become { .. } | Break { .. } | Continue { .. } | ConstContinue { .. } | Return { .. } => true , Assign { .. } | AssignOp { .. } | InlineAsm { .. } | Let { .. } => true , RawBorrow { .. } | Adt { .. } | Array { .. } | Binary { .. } | Block { .. } | Borrow { .. } | Box { .. } | Call { .. } | ByUse { .. } | Closure { .. } | ConstBlock { .. } | ConstParam { .. } | If { .. } | Literal { .. } | LogicalOp { .. } | Loop { .. } | LoopMatch { .. } | Match { .. } | NamedConst { .. } | NonHirLiteral { .. } | OffsetOf { .. } | Repeat { .. } | StaticRef { .. } | ThreadLocalRef { .. } | Tuple { .. } | Unary { .. } | UpvarRef { .. } | VarRef { .. } | ZstLiteral { .. } | Yield { .. } => true , } } fn new_cx (& self , refutability : RefutableFlag , whole_match_span : Option < Span > , scrutinee : Option < & Expr < 'tcx > > , scrut_span : Span ,) -> PatCtxt < 'p , 'tcx > { let refutable = match refutability { Irrefutable => false , Refutable => true , } ; let known_valid_scrutinee = scrutinee . map (| scrut | self . is_known_valid_scrutinee (scrut)) . unwrap_or (true) ; PatCtxt { tcx : self . tcx , typeck_results : self . typeck_results , typing_env : self . typing_env , module : self . tcx . parent_module (self . lint_level) . to_def_id () , dropless_arena : self . dropless_arena , match_lint_level : self . lint_level , whole_match_span , scrut_span , refutable , known_valid_scrutinee , internal_state : Default :: default () , } } fn analyze_patterns (& mut self , cx : & PatCtxt < 'p , 'tcx > , arms : & [MatchArm < 'p , 'tcx >] , scrut_ty : Ty < 'tcx > ,) -> Result < UsefulnessReport < 'p , 'tcx > , ErrorGuaranteed > { let report = rustc_pattern_analysis :: rustc :: analyze_match (& cx , & arms , scrut_ty) . map_err (| err | { self . error = Err (err) ; err }) ? ; for (arm , is_useful) in report . arm_usefulness . iter () { if let Usefulness :: Useful (redundant_subpats) = is_useful && ! redundant_subpats . is_empty () { let mut redundant_subpats = redundant_subpats . clone () ; redundant_subpats . sort_unstable_by_key (| (pat , _) | pat . data () . span) ; for (pat , explanation) in redundant_subpats { report_unreachable_pattern (cx , arm . arm_data , pat , & explanation , None) } } } Ok (report) } # [instrument (level = "trace" , skip (self))] fn check_let (& mut self , pat : & 'p Pat < 'tcx > , scrutinee : Option < ExprId > , span : Span) { assert ! (self . let_source != LetSource :: None) ; let scrut = scrutinee . map (| id | & self . thir [id]) ; if let LetSource :: PlainLet = self . let_source { self . check_binding_is_irrefutable (pat , "local binding" , scrut , Some (span)) } else { let Ok (refutability) = self . is_let_irrefutable (pat , scrut) else { return } ; if matches ! (refutability , Irrefutable) { report_irrefutable_let_patterns (self . tcx , self . lint_level , self . let_source , 1 , span ,) ; } } } fn check_match (& mut self , scrut : ExprId , arms : & [ArmId] , source : hir :: MatchSource , expr_span : Span ,) { let scrut = & self . thir [scrut] ; let cx = self . new_cx (Refutable , Some (expr_span) , Some (scrut) , scrut . span) ; let mut tarms = Vec :: with_capacity (arms . len ()) ; for & arm in arms { let arm = & self . thir . arms [arm] ; let got_error = self . with_lint_level (arm . lint_level , | this | { let Ok (pat) = this . lower_pattern (& cx , & arm . pattern) else { return true } ; let arm = MatchArm { pat , arm_data : this . lint_level , has_guard : arm . guard . is_some () } ; tarms . push (arm) ; false }) ; if got_error { return ; } } let Ok (report) = self . analyze_patterns (& cx , & tarms , scrut . ty) else { return } ; match source { hir :: MatchSource :: ForLoopDesugar if arms . len () == 1 => { } hir :: MatchSource :: ForLoopDesugar | hir :: MatchSource :: Postfix | hir :: MatchSource :: Normal | hir :: MatchSource :: FormatArgs => { let is_match_arm = matches ! (source , hir :: MatchSource :: Postfix | hir :: MatchSource :: Normal) ; report_arm_reachability (& cx , & report , is_match_arm) ; } hir :: MatchSource :: AwaitDesugar | hir :: MatchSource :: TryDesugar (_) => { } } let witnesses = report . non_exhaustiveness_witnesses ; if ! witnesses . is_empty () { if source == hir :: MatchSource :: ForLoopDesugar && let [_ , snd_arm] = * arms { let pat = & self . thir [snd_arm] . pattern ; debug_assert_eq ! (pat . span . desugaring_kind () , Some (DesugaringKind :: ForLoop)) ; let PatKind :: Variant { ref subpatterns , .. } = pat . kind else { bug ! () } ; let [pat_field] = & subpatterns [..] else { bug ! () } ; self . check_binding_is_irrefutable (& pat_field . pattern , "`for` loop binding" , None , None ,) ; } else { let braces_span = match source { hir :: MatchSource :: Normal => scrut . span . find_ancestor_in_same_ctxt (expr_span) . map (| scrut_span | scrut_span . shrink_to_hi () . with_hi (expr_span . hi ())) , hir :: MatchSource :: Postfix => { scrut . span . find_ancestor_in_same_ctxt (expr_span) . and_then (| scrut_span | { let sm = self . tcx . sess . source_map () ; let brace_span = sm . span_extend_to_next_char (scrut_span , '{' , true) ; if sm . span_to_snippet (sm . next_point (brace_span)) . as_deref () == Ok ("{") { let sp = brace_span . shrink_to_hi () . with_hi (expr_span . hi ()) ; sm . span_extend_prev_while (sp , | c | c . is_whitespace ()) . ok () } else { None } }) } hir :: MatchSource :: ForLoopDesugar | hir :: MatchSource :: TryDesugar (_) | hir :: MatchSource :: AwaitDesugar | hir :: MatchSource :: FormatArgs => None , } ; self . error = Err (report_non_exhaustive_match (& cx , self . thir , scrut . ty , scrut . span , witnesses , arms , braces_span ,)) ; } } } # [instrument (level = "trace" , skip (self))] fn check_let_chain (& mut self , chain_refutabilities : Vec < Option < (Span , RefutableFlag) > > , whole_chain_span : Span ,) { assert ! (self . let_source != LetSource :: None) ; if chain_refutabilities . iter () . all (| r | matches ! (* r , Some ((_ , Irrefutable)))) { report_irrefutable_let_patterns (self . tcx , self . lint_level , self . let_source , chain_refutabilities . len () , whole_chain_span ,) ; return ; } if let Some (until) = chain_refutabilities . iter () . position (| r | ! matches ! (* r , Some ((_ , Irrefutable)))) && until > 0 { if ! matches ! (self . let_source , LetSource :: WhileLet | LetSource :: IfLetGuard | LetSource :: ElseIfLet) { let prefix = & chain_refutabilities [.. until] ; let span_start = prefix [0] . unwrap () . 0 ; let span_end = prefix . last () . unwrap () . unwrap () . 0 ; let span = span_start . to (span_end) ; let count = prefix . len () ; self . tcx . emit_node_span_lint (IRREFUTABLE_LET_PATTERNS , self . lint_level , span , LeadingIrrefutableLetPatterns { count } ,) ; } } if let Some (from) = chain_refutabilities . iter () . rposition (| r | ! matches ! (* r , Some ((_ , Irrefutable)))) && from != (chain_refutabilities . len () - 1) { let suffix = & chain_refutabilities [from + 1 ..] ; let span_start = suffix [0] . unwrap () . 0 ; let span_end = suffix . last () . unwrap () . unwrap () . 0 ; let span = span_start . to (span_end) ; let count = suffix . len () ; self . tcx . emit_node_span_lint (IRREFUTABLE_LET_PATTERNS , self . lint_level , span , TrailingIrrefutableLetPatterns { count } ,) ; } } fn analyze_binding (& mut self , pat : & 'p Pat < 'tcx > , refutability : RefutableFlag , scrut : Option < & Expr < 'tcx > > ,) -> Result < (PatCtxt < 'p , 'tcx > , UsefulnessReport < 'p , 'tcx >) , ErrorGuaranteed > { let cx = self . new_cx (refutability , None , scrut , pat . span) ; let pat = self . lower_pattern (& cx , pat) ? ; let arms = [MatchArm { pat , arm_data : self . lint_level , has_guard : false }] ; let report = self . analyze_patterns (& cx , & arms , pat . ty () . inner ()) ? ; Ok ((cx , report)) } fn is_let_irrefutable (& mut self , pat : & 'p Pat < 'tcx > , scrut : Option < & Expr < 'tcx > > ,) -> Result < RefutableFlag , ErrorGuaranteed > { let (cx , report) = self . analyze_binding (pat , Refutable , scrut) ? ; report_arm_reachability (& cx , & report , false) ; Ok (if report . non_exhaustiveness_witnesses . is_empty () { Irrefutable } else { Refutable }) } # [instrument (level = "trace" , skip (self))] fn check_binding_is_irrefutable (& mut self , pat : & 'p Pat < 'tcx > , origin : & str , scrut : Option < & Expr < 'tcx > > , sp : Option < Span > ,) { let pattern_ty = pat . ty ; let Ok ((cx , report)) = self . analyze_binding (pat , Irrefutable , scrut) else { return } ; let witnesses = report . non_exhaustiveness_witnesses ; if witnesses . is_empty () { return ; } let inform = sp . is_some () . then_some (Inform) ; let mut let_suggestion = None ; let mut misc_suggestion = None ; let mut interpreted_as_const = None ; let mut interpreted_as_const_sugg = None ; let mut unpeeled_pat = pat ; while let PatKind :: AscribeUserType { ref subpattern , .. } = unpeeled_pat . kind { unpeeled_pat = subpattern ; } if let PatKind :: ExpandedConstant { def_id , .. } = unpeeled_pat . kind && let DefKind :: Const = self . tcx . def_kind (def_id) && let Ok (snippet) = self . tcx . sess . source_map () . span_to_snippet (pat . span) && snippet . chars () . all (| c | c . is_alphanumeric () || c == '_') { let span = self . tcx . def_span (def_id) ; let variable = self . tcx . item_name (def_id) . to_string () ; interpreted_as_const = Some (InterpretedAsConst { span , variable : variable . clone () }) ; interpreted_as_const_sugg = Some (InterpretedAsConstSugg { span : pat . span , variable }) ; } else if let PatKind :: Constant { .. } = unpeeled_pat . kind && let Ok (snippet) = self . tcx . sess . source_map () . span_to_snippet (pat . span) { if snippet . chars () . all (| c | c . is_digit (10)) { misc_suggestion = Some (MiscPatternSuggestion :: AttemptedIntegerLiteral { start_span : pat . span . shrink_to_lo () , }) ; } } if let Some (span) = sp && self . tcx . sess . source_map () . is_span_accessible (span) && interpreted_as_const . is_none () && scrut . is_some () { let mut bindings = vec ! [] ; pat . each_binding (| name , _ , _ , _ | bindings . push (name)) ; let semi_span = span . shrink_to_hi () ; let start_span = span . shrink_to_lo () ; let end_span = semi_span . shrink_to_lo () ; let count = witnesses . len () ; let_suggestion = Some (if bindings . is_empty () { SuggestLet :: If { start_span , semi_span , count } } else { SuggestLet :: Else { end_span , count } }) ; } ; let adt_defined_here = report_adt_defined_here (self . tcx , pattern_ty , & witnesses , false) ; let witness_1_is_privately_uninhabited = if let Some (witness_1) = witnesses . get (0) && let ty :: Adt (adt , args) = witness_1 . ty () . kind () && adt . is_enum () && let Constructor :: Variant (variant_index) = witness_1 . ctor () { let variant_inhabited = adt . variant (* variant_index) . inhabited_predicate (self . tcx , * adt) . instantiate (self . tcx , args) ; variant_inhabited . apply (self . tcx , cx . typing_env , cx . module) && ! variant_inhabited . apply_ignore_module (self . tcx , cx . typing_env) } else { false } ; let witness_1 = cx . print_witness_pat (witnesses . get (0) . unwrap ()) ; self . error = Err (self . tcx . dcx () . emit_err (PatternNotCovered { span : pat . span , origin , uncovered : Uncovered :: new (pat . span , & cx , witnesses) , inform , interpreted_as_const , interpreted_as_const_sugg , witness_1_is_privately_uninhabited , witness_1 , _p : () , pattern_ty , let_suggestion , misc_suggestion , adt_defined_here , })) ; } }}}

macro_rules! check_borrow_conflicts_in_at_patterns_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_borrow_conflicts_in_at_patterns in module {}", module_path!());
    };
}

mkfn!{
    check_borrow_conflicts_in_at_patterns_introspect!();
    # [doc = " Check if a by-value binding is by-value. That is, check if the binding's type is not `Copy`."] # [doc = " Check that there are no borrow or move conflicts in `binding @ subpat` patterns."] # [doc = ""] # [doc = " For example, this would reject:"] # [doc = " - `ref x @ Some(ref mut y)`,"] # [doc = " - `ref mut x @ Some(ref y)`,"] # [doc = " - `ref mut x @ Some(ref mut y)`,"] # [doc = " - `ref mut? x @ Some(y)`, and"] # [doc = " - `x @ Some(ref mut? y)`."] # [doc = ""] # [doc = " This analysis is *not* subsumed by NLL."] fn check_borrow_conflicts_in_at_patterns < 'tcx > (cx : & MatchVisitor < '_ , 'tcx > , pat : & Pat < 'tcx >) { let PatKind :: Binding { name , mode , ty , subpattern : Some (box ref sub) , .. } = pat . kind else { return ; } ; let is_binding_by_move = | ty : Ty < 'tcx > | ! cx . tcx . type_is_copy_modulo_regions (cx . typing_env , ty) ; let sess = cx . tcx . sess ; let mut_outer = match mode . 0 { ByRef :: No if is_binding_by_move (ty) => { let mut conflicts_ref = Vec :: new () ; sub . each_binding (| _ , mode , _ , span | { if matches ! (mode , ByRef :: Yes (_)) { conflicts_ref . push (span) } }) ; if ! conflicts_ref . is_empty () { sess . dcx () . emit_err (BorrowOfMovedValue { binding_span : pat . span , conflicts_ref , name : Ident :: new (name , pat . span) , ty , suggest_borrowing : Some (pat . span . shrink_to_lo ()) , }) ; } return ; } ByRef :: No => return , ByRef :: Yes (m) => m , } ; let mut conflicts_move = Vec :: new () ; let mut conflicts_mut_mut = Vec :: new () ; let mut conflicts_mut_ref = Vec :: new () ; sub . each_binding (| name , mode , ty , span | { match mode { ByRef :: Yes (mut_inner) => match (mut_outer , mut_inner) { (Mutability :: Not , Mutability :: Not) => { } (Mutability :: Mut , Mutability :: Mut) => { conflicts_mut_mut . push (Conflict :: Mut { span , name }) } (Mutability :: Not , Mutability :: Mut) => { conflicts_mut_ref . push (Conflict :: Mut { span , name }) } (Mutability :: Mut , Mutability :: Not) => { conflicts_mut_ref . push (Conflict :: Ref { span , name }) } } , ByRef :: No if is_binding_by_move (ty) => { conflicts_move . push (Conflict :: Moved { span , name }) } ByRef :: No => { } } }) ; let report_mut_mut = ! conflicts_mut_mut . is_empty () ; let report_mut_ref = ! conflicts_mut_ref . is_empty () ; let report_move_conflict = ! conflicts_move . is_empty () ; let mut occurrences = match mut_outer { Mutability :: Mut => vec ! [Conflict :: Mut { span : pat . span , name }] , Mutability :: Not => vec ! [Conflict :: Ref { span : pat . span , name }] , } ; occurrences . extend (conflicts_mut_mut) ; occurrences . extend (conflicts_mut_ref) ; occurrences . extend (conflicts_move) ; if report_mut_mut { sess . dcx () . emit_err (MultipleMutBorrows { span : pat . span , occurrences }) ; } else if report_mut_ref { match mut_outer { Mutability :: Mut => { sess . dcx () . emit_err (AlreadyMutBorrowed { span : pat . span , occurrences }) ; } Mutability :: Not => { sess . dcx () . emit_err (AlreadyBorrowed { span : pat . span , occurrences }) ; } } ; } else if report_move_conflict { sess . dcx () . emit_err (MovedWhileBorrowed { span : pat . span , occurrences }) ; } }
}

macro_rules! check_for_bindings_named_same_as_variants_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_for_bindings_named_same_as_variants in module {}", module_path!());
    };
}

mkfn!{
    check_for_bindings_named_same_as_variants_introspect!();
    fn check_for_bindings_named_same_as_variants (cx : & MatchVisitor < '_ , '_ > , pat : & Pat < '_ > , rf : RefutableFlag ,) { if let PatKind :: Binding { name , mode : BindingMode (ByRef :: No , Mutability :: Not) , subpattern : None , ty , .. } = pat . kind && let ty :: Adt (edef , _) = ty . peel_refs () . kind () && edef . is_enum () && edef . variants () . iter () . any (| variant | variant . name == name && variant . ctor_kind () == Some (CtorKind :: Const)) { let variant_count = edef . variants () . len () ; let ty_path = with_no_trimmed_paths ! (cx . tcx . def_path_str (edef . did ())) ; cx . tcx . emit_node_span_lint (BINDINGS_WITH_VARIANT_NAME , cx . lint_level , pat . span , BindingsWithVariantName { suggestion : if rf == Refutable || variant_count == 1 { Some (pat . span) } else { None } , ty_path , name : Ident :: new (name , pat . span) , } ,) } }
}

macro_rules! check_never_pattern_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_never_pattern in module {}", module_path!());
    };
}

mkfn!{
    check_never_pattern_introspect!();
    # [doc = " Check that never patterns are only used on inhabited types."] fn check_never_pattern < 'tcx > (cx : & PatCtxt < '_ , 'tcx > , pat : & Pat < 'tcx > ,) -> Result < () , ErrorGuaranteed > { if let PatKind :: Never = pat . kind { if ! cx . is_uninhabited (pat . ty) { return Err (cx . tcx . dcx () . emit_err (NonEmptyNeverPattern { span : pat . span , ty : pat . ty })) ; } } Ok (()) }
}

macro_rules! report_irrefutable_let_patterns_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function report_irrefutable_let_patterns in module {}", module_path!());
    };
}

mkfn!{
    report_irrefutable_let_patterns_introspect!();
    fn report_irrefutable_let_patterns (tcx : TyCtxt < '_ > , id : HirId , source : LetSource , count : usize , span : Span ,) { macro_rules ! emit_diag { ($ lint : tt) => { { tcx . emit_node_span_lint (IRREFUTABLE_LET_PATTERNS , id , span , $ lint { count }) ; } } ; } match source { LetSource :: None | LetSource :: PlainLet | LetSource :: Else => bug ! () , LetSource :: IfLet | LetSource :: ElseIfLet => emit_diag ! (IrrefutableLetPatternsIfLet) , LetSource :: IfLetGuard => emit_diag ! (IrrefutableLetPatternsIfLetGuard) , LetSource :: LetElse => emit_diag ! (IrrefutableLetPatternsLetElse) , LetSource :: WhileLet => emit_diag ! (IrrefutableLetPatternsWhileLet) , } }
}

macro_rules! report_unreachable_pattern_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function report_unreachable_pattern in module {}", module_path!());
    };
}

mkfn!{
    report_unreachable_pattern_introspect!();
    # [doc = " Report unreachable arms, if any."] fn report_unreachable_pattern < 'p , 'tcx > (cx : & PatCtxt < 'p , 'tcx > , hir_id : HirId , pat : & DeconstructedPat < 'p , 'tcx > , explanation : & RedundancyExplanation < 'p , 'tcx > , whole_arm_span : Option < Span > ,) { static CAP_COVERED_BY_MANY : usize = 4 ; let pat_span = pat . data () . span ; let mut lint = UnreachablePattern { span : Some (pat_span) , matches_no_values : None , matches_no_values_ty : * * pat . ty () , uninhabited_note : None , covered_by_catchall : None , covered_by_one : None , covered_by_many : None , covered_by_many_n_more_count : 0 , wanted_constant : None , accessible_constant : None , inaccessible_constant : None , pattern_let_binding : None , suggest_remove : None , } ; match explanation . covered_by . as_slice () { [] => { lint . span = None ; lint . uninhabited_note = Some (()) ; lint . matches_no_values = Some (pat_span) ; lint . suggest_remove = whole_arm_span ; pat . walk (& mut | subpat | { let ty = * * subpat . ty () ; if cx . is_uninhabited (ty) { lint . matches_no_values_ty = ty ; false } else if matches ! (subpat . ctor () , Constructor :: Ref | Constructor :: UnionField) { false } else { true } }) ; } [covering_pat] if pat_is_catchall (covering_pat) => { let pat = covering_pat . data () ; lint . covered_by_catchall = Some (pat . span) ; find_fallback_pattern_typo (cx , hir_id , pat , & mut lint) ; } [covering_pat] => { lint . covered_by_one = Some (covering_pat . data () . span) ; } covering_pats => { let mut iter = covering_pats . iter () ; let mut multispan = MultiSpan :: from_span (pat_span) ; for p in iter . by_ref () . take (CAP_COVERED_BY_MANY) { multispan . push_span_label (p . data () . span , fluent :: mir_build_unreachable_matches_same_values ,) ; } let remain = iter . count () ; if remain == 0 { multispan . push_span_label (pat_span , fluent :: mir_build_unreachable_making_this_unreachable ,) ; } else { lint . covered_by_many_n_more_count = remain ; multispan . push_span_label (pat_span , fluent :: mir_build_unreachable_making_this_unreachable_n_more ,) ; } lint . covered_by_many = Some (multispan) ; } } cx . tcx . emit_node_span_lint (UNREACHABLE_PATTERNS , hir_id , pat_span , lint) ; }
}

macro_rules! find_fallback_pattern_typo_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function find_fallback_pattern_typo in module {}", module_path!());
    };
}

mkfn!{
    find_fallback_pattern_typo_introspect!();
    # [doc = " Detect typos that were meant to be a `const` but were interpreted as a new pattern binding."] fn find_fallback_pattern_typo < 'tcx > (cx : & PatCtxt < '_ , 'tcx > , hir_id : HirId , pat : & Pat < 'tcx > , lint : & mut UnreachablePattern < '_ > ,) { if let Level :: Allow = cx . tcx . lint_level_at_node (UNREACHABLE_PATTERNS , hir_id) . level { return ; } if let PatKind :: Binding { name , subpattern : None , ty , .. } = pat . kind { let mut accessible = vec ! [] ; let mut accessible_path = vec ! [] ; let mut inaccessible = vec ! [] ; let mut imported = vec ! [] ; let mut imported_spans = vec ! [] ; let (infcx , param_env) = cx . tcx . infer_ctxt () . build_with_typing_env (cx . typing_env) ; let parent = cx . tcx . hir_get_parent_item (hir_id) ; for item in cx . tcx . hir_crate_items (()) . free_items () { if let DefKind :: Use = cx . tcx . def_kind (item . owner_id) { let item = cx . tcx . hir_expect_item (item . owner_id . def_id) ; let hir :: ItemKind :: Use (path , _) = item . kind else { continue ; } ; if let Some (value_ns) = path . res . value_ns && let Res :: Def (DefKind :: Const , id) = value_ns && infcx . can_eq (param_env , ty , cx . tcx . type_of (id) . instantiate_identity ()) { if cx . tcx . visibility (id) . is_accessible_from (parent , cx . tcx) { let item_name = cx . tcx . item_name (id) ; accessible . push (item_name) ; accessible_path . push (with_no_trimmed_paths ! (cx . tcx . def_path_str (id))) ; } else if cx . tcx . visibility (item . owner_id) . is_accessible_from (parent , cx . tcx) { let ident = item . kind . ident () . unwrap () ; imported . push (ident . name) ; imported_spans . push (ident . span) ; } } } if let DefKind :: Const = cx . tcx . def_kind (item . owner_id) && infcx . can_eq (param_env , ty , cx . tcx . type_of (item . owner_id) . instantiate_identity ()) { let item_name = cx . tcx . item_name (item . owner_id) ; let vis = cx . tcx . visibility (item . owner_id) ; if vis . is_accessible_from (parent , cx . tcx) { accessible . push (item_name) ; let path = with_no_trimmed_paths ! (cx . tcx . def_path_str (item . owner_id)) ; accessible_path . push (path) ; } else if name == item_name { inaccessible . push (cx . tcx . def_span (item . owner_id)) ; } } } if let Some ((i , & const_name)) = accessible . iter () . enumerate () . find (| & (_ , & const_name) | const_name == name) { lint . wanted_constant = Some (WantedConstant { span : pat . span , is_typo : false , const_name : const_name . to_string () , const_path : accessible_path [i] . clone () , }) ; } else if let Some (name) = find_best_match_for_name (& accessible , name , None) { lint . wanted_constant = Some (WantedConstant { span : pat . span , is_typo : true , const_name : name . to_string () , const_path : name . to_string () , }) ; } else if let Some (i) = imported . iter () . enumerate () . find (| & (_ , & const_name) | const_name == name) . map (| (i , _) | i) { lint . accessible_constant = Some (imported_spans [i]) ; } else if let Some (name) = find_best_match_for_name (& imported , name , None) { lint . wanted_constant = Some (WantedConstant { span : pat . span , is_typo : true , const_path : name . to_string () , const_name : name . to_string () , }) ; } else if ! inaccessible . is_empty () { for span in inaccessible { lint . inaccessible_constant = Some (span) ; } } else { for (_ , node) in cx . tcx . hir_parent_iter (hir_id) { match node { hir :: Node :: Stmt (hir :: Stmt { kind : hir :: StmtKind :: Let (let_stmt) , .. }) => { if let hir :: PatKind :: Binding (_ , _ , binding_name , _) = let_stmt . pat . kind { if name == binding_name . name { lint . pattern_let_binding = Some (binding_name . span) ; } } } hir :: Node :: Block (hir :: Block { stmts , .. }) => { for stmt in * stmts { if let hir :: StmtKind :: Let (let_stmt) = stmt . kind && let hir :: PatKind :: Binding (_ , _ , binding_name , _) = let_stmt . pat . kind && name == binding_name . name { lint . pattern_let_binding = Some (binding_name . span) ; } } } hir :: Node :: Item (_) => break , _ => { } } } } } }
}

macro_rules! report_arm_reachability_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function report_arm_reachability in module {}", module_path!());
    };
}

mkfn!{
    report_arm_reachability_introspect!();
    # [doc = " Report unreachable arms, if any."] fn report_arm_reachability < 'p , 'tcx > (cx : & PatCtxt < 'p , 'tcx > , report : & UsefulnessReport < 'p , 'tcx > , is_match_arm : bool ,) { let sm = cx . tcx . sess . source_map () ; for (arm , is_useful) in report . arm_usefulness . iter () { if let Usefulness :: Redundant (explanation) = is_useful { let hir_id = arm . arm_data ; let arm_span = cx . tcx . hir_span (hir_id) ; let whole_arm_span = if is_match_arm { let with_whitespace = sm . span_extend_while_whitespace (arm_span) ; if let Some (comma) = sm . span_look_ahead (with_whitespace , "," , Some (1)) { Some (arm_span . to (comma)) } else { Some (arm_span) } } else { None } ; report_unreachable_pattern (cx , hir_id , arm . pat , explanation , whole_arm_span) } } }
}

macro_rules! pat_is_catchall_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function pat_is_catchall in module {}", module_path!());
    };
}

mkfn!{
    pat_is_catchall_introspect!();
    # [doc = " Checks for common cases of \"catchall\" patterns that may not be intended as such."] fn pat_is_catchall (pat : & DeconstructedPat < '_ , '_ >) -> bool { match pat . ctor () { Constructor :: Wildcard => true , Constructor :: Struct | Constructor :: Ref => { pat . iter_fields () . all (| ipat | pat_is_catchall (& ipat . pat)) } _ => false , } }
}

macro_rules! report_non_exhaustive_match_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function report_non_exhaustive_match in module {}", module_path!());
    };
}

mkfn!{
    report_non_exhaustive_match_introspect!();
    # [doc = " Report that a match is not exhaustive."] fn report_non_exhaustive_match < 'p , 'tcx > (cx : & PatCtxt < 'p , 'tcx > , thir : & Thir < 'tcx > , scrut_ty : Ty < 'tcx > , sp : Span , witnesses : Vec < WitnessPat < 'p , 'tcx > > , arms : & [ArmId] , braces_span : Option < Span > ,) -> ErrorGuaranteed { let is_empty_match = arms . is_empty () ; let non_empty_enum = match scrut_ty . kind () { ty :: Adt (def , _) => def . is_enum () && ! def . variants () . is_empty () , _ => false , } ; if is_empty_match && ! non_empty_enum { return cx . tcx . dcx () . emit_err (NonExhaustivePatternsTypeNotEmpty { cx , scrut_span : sp , braces_span , ty : scrut_ty , }) ; } let joined_patterns = joined_uncovered_patterns (cx , & witnesses) ; let mut err = struct_span_code_err ! (cx . tcx . dcx () , sp , E0004 , "non-exhaustive patterns: {joined_patterns} not covered") ; err . span_label (sp , format ! ("pattern{} {} not covered" , rustc_errors :: pluralize ! (witnesses . len ()) , joined_patterns) ,) ; if let Some (AdtDefinedHere { adt_def_span , ty , variants }) = report_adt_defined_here (cx . tcx , scrut_ty , & witnesses , true) { let mut multi_span = MultiSpan :: from_span (adt_def_span) ; multi_span . push_span_label (adt_def_span , "") ; for Variant { span } in variants { multi_span . push_span_label (span , "not covered") ; } err . span_note (multi_span , format ! ("`{ty}` defined here")) ; } err . note (format ! ("the matched value is of type `{}`" , scrut_ty)) ; if ! is_empty_match { let mut special_tys = FxIndexSet :: default () ; collect_special_tys (cx , & witnesses [0] , & mut special_tys) ; for ty in special_tys { if ty . is_ptr_sized_integral () { if ty . inner () == cx . tcx . types . usize { err . note (format ! ("`{ty}` does not have a fixed maximum value, so half-open ranges are \
                         necessary to match exhaustively" ,)) ; } else if ty . inner () == cx . tcx . types . isize { err . note (format ! ("`{ty}` does not have fixed minimum and maximum values, so half-open \
                         ranges are necessary to match exhaustively" ,)) ; } } else if ty . inner () == cx . tcx . types . str_ { err . note ("`&str` cannot be matched exhaustively, so a wildcard `_` is necessary") ; } else if cx . is_foreign_non_exhaustive_enum (ty) { err . note (format ! ("`{ty}` is marked as non-exhaustive, so a wildcard `_` is necessary to match exhaustively")) ; } else if cx . is_uninhabited (ty . inner ()) { err . note (format ! ("`{ty}` is uninhabited but is not being matched by value, so a wildcard `_` is required")) ; } } } if let ty :: Ref (_ , sub_ty , _) = scrut_ty . kind () { if ! sub_ty . is_inhabited_from (cx . tcx , cx . module , cx . typing_env) { err . note ("references are always considered inhabited") ; } } for & arm in arms { let arm = & thir . arms [arm] ; if let PatKind :: ExpandedConstant { def_id , .. } = arm . pattern . kind && ! matches ! (cx . tcx . def_kind (def_id) , DefKind :: InlineConst) && let Ok (snippet) = cx . tcx . sess . source_map () . span_to_snippet (arm . pattern . span) && snippet . chars () . all (| c | c . is_alphanumeric () || c == '_') { let const_name = cx . tcx . item_name (def_id) ; err . span_label (arm . pattern . span , format ! ("this pattern doesn't introduce a new catch-all binding, but rather pattern \
                     matches against the value of constant `{const_name}`" ,) ,) ; err . span_note (cx . tcx . def_span (def_id) , format ! ("constant `{const_name}` defined here")) ; err . span_suggestion_verbose (arm . pattern . span . shrink_to_hi () , "if you meant to introduce a binding, use a different name" , "_var" . to_string () , Applicability :: MaybeIncorrect ,) ; } } let suggest_the_witnesses = witnesses . len () < 4 ; let suggested_arm = if suggest_the_witnesses { let pattern = witnesses . iter () . map (| witness | cx . print_witness_pat (witness)) . collect :: < Vec < String > > () . join (" | ") ; if witnesses . iter () . all (| p | p . is_never_pattern ()) && cx . tcx . features () . never_patterns () { pattern } else { format ! ("{pattern} => todo!()") } } else { format ! ("_ => todo!()") } ; let mut suggestion = None ; let sm = cx . tcx . sess . source_map () ; match arms { [] if let Some (braces_span) = braces_span => { let (indentation , more) = if let Some (snippet) = sm . indentation_before (sp) { (format ! ("\n{snippet}") , "    ") } else { (" " . to_string () , "") } ; suggestion = Some ((braces_span , format ! (" {{{indentation}{more}{suggested_arm},{indentation}}}" ,) ,)) ; } [only] => { let only = & thir [* only] ; let (pre_indentation , is_multiline) = if let Some (snippet) = sm . indentation_before (only . span) && let Ok (with_trailing) = sm . span_extend_while (only . span , | c | c . is_whitespace () || c == ',') && sm . is_multiline (with_trailing) { (format ! ("\n{snippet}") , true) } else { (" " . to_string () , false) } ; let only_body = & thir [only . body] ; let comma = if matches ! (only_body . kind , ExprKind :: Block { .. }) && only . span . eq_ctxt (only_body . span) && is_multiline { "" } else { "," } ; suggestion = Some ((only . span . shrink_to_hi () , format ! ("{comma}{pre_indentation}{suggested_arm}") ,)) ; } [.. , prev , last] => { let prev = & thir [* prev] ; let last = & thir [* last] ; if prev . span . eq_ctxt (last . span) { let last_body = & thir [last . body] ; let comma = if matches ! (last_body . kind , ExprKind :: Block { .. }) && last . span . eq_ctxt (last_body . span) { "" } else { "," } ; let spacing = if sm . is_multiline (prev . span . between (last . span)) { sm . indentation_before (last . span) . map (| indent | format ! ("\n{indent}")) } else { Some (" " . to_string ()) } ; if let Some (spacing) = spacing { suggestion = Some ((last . span . shrink_to_hi () , format ! ("{comma}{spacing}{suggested_arm}") ,)) ; } } } _ => { } } let msg = format ! ("ensure that all possible cases are being handled by adding a match arm with a wildcard \
         pattern{}{}" , if witnesses . len () > 1 && suggest_the_witnesses && suggestion . is_some () { ", a match arm with multiple or-patterns" } else { "" } , match witnesses . len () { 0 if suggestion . is_some () => " as shown" , 0 => "" , 1 if suggestion . is_some () => " or an explicit pattern as shown" , 1 => " or an explicit pattern" , _ if suggestion . is_some () => " as shown, or multiple match arms" , _ => " or multiple match arms" , } ,) ; let all_arms_have_guards = arms . iter () . all (| arm_id | thir [* arm_id] . guard . is_some ()) ; if ! is_empty_match && all_arms_have_guards { err . subdiagnostic (NonExhaustiveMatchAllArmsGuarded) ; } if let Some ((span , sugg)) = suggestion { err . span_suggestion_verbose (span , msg , sugg , Applicability :: HasPlaceholders) ; } else { err . help (msg) ; } err . emit () }
}

macro_rules! joined_uncovered_patterns_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function joined_uncovered_patterns in module {}", module_path!());
    };
}

mkfn!{
    joined_uncovered_patterns_introspect!();
    fn joined_uncovered_patterns < 'p , 'tcx > (cx : & PatCtxt < 'p , 'tcx > , witnesses : & [WitnessPat < 'p , 'tcx >] ,) -> String { const LIMIT : usize = 3 ; let pat_to_str = | pat : & WitnessPat < 'p , 'tcx > | cx . print_witness_pat (pat) ; match witnesses { [] => bug ! () , [witness] => format ! ("`{}`" , cx . print_witness_pat (witness)) , [head @ .. , tail] if head . len () < LIMIT => { let head : Vec < _ > = head . iter () . map (pat_to_str) . collect () ; format ! ("`{}` and `{}`" , head . join ("`, `") , cx . print_witness_pat (tail)) } _ => { let (head , tail) = witnesses . split_at (LIMIT) ; let head : Vec < _ > = head . iter () . map (pat_to_str) . collect () ; format ! ("`{}` and {} more" , head . join ("`, `") , tail . len ()) } } }
}

macro_rules! collect_special_tys_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function collect_special_tys in module {}", module_path!());
    };
}

mkfn!{
    collect_special_tys_introspect!();
    # [doc = " Collect types that require specific explanations when they show up in witnesses."] fn collect_special_tys < 'tcx > (cx : & PatCtxt < '_ , 'tcx > , pat : & WitnessPat < '_ , 'tcx > , special_tys : & mut FxIndexSet < RevealedTy < 'tcx > > ,) { if matches ! (pat . ctor () , Constructor :: NonExhaustive | Constructor :: Never) { special_tys . insert (* pat . ty ()) ; } if let Constructor :: IntRange (range) = pat . ctor () { if cx . is_range_beyond_boundaries (range , * pat . ty ()) { special_tys . insert (* pat . ty ()) ; } } pat . iter_fields () . for_each (| field_pat | collect_special_tys (cx , field_pat , special_tys)) }
}

macro_rules! report_adt_defined_here_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function report_adt_defined_here in module {}", module_path!());
    };
}

mkfn!{
    report_adt_defined_here_introspect!();
    fn report_adt_defined_here < 'tcx > (tcx : TyCtxt < 'tcx > , ty : Ty < 'tcx > , witnesses : & [WitnessPat < '_ , 'tcx >] , point_at_non_local_ty : bool ,) -> Option < AdtDefinedHere < 'tcx > > { let ty = ty . peel_refs () ; let ty :: Adt (def , _) = ty . kind () else { return None ; } ; let adt_def_span = tcx . hir_get_if_local (def . did ()) . and_then (| node | node . ident ()) . map (| ident | ident . span) ; let adt_def_span = if point_at_non_local_ty { adt_def_span . unwrap_or_else (| | tcx . def_span (def . did ())) } else { adt_def_span ? } ; let mut variants = vec ! [] ; for span in maybe_point_at_variant (tcx , * def , witnesses . iter () . take (5)) { variants . push (Variant { span }) ; } Some (AdtDefinedHere { adt_def_span , ty , variants }) }
}

macro_rules! maybe_point_at_variant_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function maybe_point_at_variant in module {}", module_path!());
    };
}

mkfn!{
    maybe_point_at_variant_introspect!();
    fn maybe_point_at_variant < 'a , 'p : 'a , 'tcx : 'p > (tcx : TyCtxt < 'tcx > , def : AdtDef < 'tcx > , patterns : impl Iterator < Item = & 'a WitnessPat < 'p , 'tcx > > ,) -> Vec < Span > { let mut covered = vec ! [] ; for pattern in patterns { if let Constructor :: Variant (variant_index) = pattern . ctor () { if let ty :: Adt (this_def , _) = pattern . ty () . kind () && this_def . did () != def . did () { continue ; } let sp = def . variant (* variant_index) . ident (tcx) . span ; if covered . contains (& sp) { continue ; } covered . push (sp) ; } covered . extend (maybe_point_at_variant (tcx , def , pattern . iter_fields ())) ; } covered }
}