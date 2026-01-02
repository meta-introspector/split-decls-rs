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
mkuse!{use rustc_abi :: { FieldIdx , VariantIdx } ;}
mkuse!{use rustc_apfloat :: Float ;}
mkuse!{use rustc_data_structures :: fx :: FxHashSet ;}
mkuse!{use rustc_errors :: Diag ;}
mkuse!{use rustc_hir as hir ;}
mkuse!{use rustc_hir :: attrs :: AttributeKind ;}
mkuse!{use rustc_hir :: find_attr ;}
mkuse!{use rustc_index :: Idx ;}
mkuse!{use rustc_infer :: infer :: TyCtxtInferExt ;}
mkuse!{use rustc_infer :: traits :: Obligation ;}
mkuse!{use rustc_middle :: mir :: interpret :: ErrorHandled ;}
mkuse!{use rustc_middle :: span_bug ;}
mkuse!{use rustc_middle :: thir :: { FieldPat , Pat , PatKind } ;}
mkuse!{use rustc_middle :: ty :: { self , Ty , TyCtxt , TypeSuperVisitable , TypeVisitableExt , TypeVisitor , ValTree , } ;}
mkuse!{use rustc_span :: def_id :: DefId ;}
mkuse!{use rustc_span :: { DUMMY_SP , Span } ;}
mkuse!{use rustc_trait_selection :: traits :: ObligationCause ;}
mkuse!{use rustc_trait_selection :: traits :: query :: evaluate_obligation :: InferCtxtExt ;}
mkuse!{use tracing :: { debug , instrument , trace } ;}
mkuse!{use super :: PatCtxt ;}
mkuse!{use crate :: errors :: { ConstPatternDependsOnGenericParameter , CouldNotEvalConstPattern , InvalidPattern , NaNPattern , PointerPattern , TypeNotPartialEq , TypeNotStructural , UnionPattern , UnsizedPattern , } ;}
mkitem!{mkimpl!{impl < 'a , 'tcx > PatCtxt < 'a , 'tcx > { # [doc = " Converts a constant to a pattern (if possible)."] # [doc = " This means aggregate values (like structs and enums) are converted"] # [doc = " to a pattern that matches the value (as if you'd compared via structural equality)."] # [doc = ""] # [doc = " Only type system constants are supported, as we are using valtrees"] # [doc = " as an intermediate step. Unfortunately those don't carry a type"] # [doc = " so we have to carry one ourselves."] # [instrument (level = "debug" , skip (self) , ret)] pub (super) fn const_to_pat (& self , c : ty :: Const < 'tcx > , ty : Ty < 'tcx > , id : hir :: HirId , span : Span ,) -> Box < Pat < 'tcx > > { let mut convert = ConstToPat :: new (self , id , span , c) ; match c . kind () { ty :: ConstKind :: Unevaluated (uv) => convert . unevaluated_to_pat (uv , ty) , ty :: ConstKind :: Value (cv) => convert . valtree_to_pat (cv . valtree , cv . ty) , _ => span_bug ! (span , "Invalid `ConstKind` for `const_to_pat`: {:?}" , c) , } } }}}
mkitem!{mkstruct!{struct ConstToPat < 'tcx > { tcx : TyCtxt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > , span : Span , id : hir :: HirId , c : ty :: Const < 'tcx > , }}}
mkitem!{mkimpl!{impl < 'tcx > ConstToPat < 'tcx > { fn new (pat_ctxt : & PatCtxt < '_ , 'tcx > , id : hir :: HirId , span : Span , c : ty :: Const < 'tcx >) -> Self { trace ! (? pat_ctxt . typeck_results . hir_owner) ; ConstToPat { tcx : pat_ctxt . tcx , typing_env : pat_ctxt . typing_env , span , id , c } } fn type_marked_structural (& self , ty : Ty < 'tcx >) -> bool { ty . is_structural_eq_shallow (self . tcx) } # [doc = " We errored. Signal that in the pattern, so that follow up errors can be silenced."] fn mk_err (& self , mut err : Diag < '_ > , ty : Ty < 'tcx >) -> Box < Pat < 'tcx > > { if let ty :: ConstKind :: Unevaluated (uv) = self . c . kind () { let def_kind = self . tcx . def_kind (uv . def) ; if let hir :: def :: DefKind :: AssocConst = def_kind && let Some (def_id) = uv . def . as_local () { err . span_label (self . tcx . def_span (self . tcx . local_parent (def_id)) , "") ; } if let hir :: def :: DefKind :: Const | hir :: def :: DefKind :: AssocConst = def_kind { err . span_label (self . tcx . def_span (uv . def) , crate :: fluent_generated :: mir_build_const_defined_here ,) ; } } Box :: new (Pat { span : self . span , ty , kind : PatKind :: Error (err . emit ()) }) } fn unevaluated_to_pat (& mut self , uv : ty :: UnevaluatedConst < 'tcx > , ty : Ty < 'tcx > ,) -> Box < Pat < 'tcx > > { let typing_env = self . tcx . erase_and_anonymize_regions (self . typing_env) . with_post_analysis_normalized (self . tcx) ; let uv = self . tcx . erase_and_anonymize_regions (uv) ; let valtree = match self . tcx . const_eval_resolve_for_typeck (typing_env , uv , self . span) { Ok (Ok (c)) => c , Err (ErrorHandled :: Reported (_ , _)) => { let mut err = self . tcx . dcx () . create_err (CouldNotEvalConstPattern { span : self . span }) ; if let ty :: ConstKind :: Unevaluated (uv) = self . c . kind () && let hir :: def :: DefKind :: Const | hir :: def :: DefKind :: AssocConst = self . tcx . def_kind (uv . def) { err . downgrade_to_delayed_bug () ; } return self . mk_err (err , ty) ; } Err (ErrorHandled :: TooGeneric (_)) => { let mut e = self . tcx . dcx () . create_err (ConstPatternDependsOnGenericParameter { span : self . span }) ; for arg in uv . args { if let ty :: GenericArgKind :: Type (ty) = arg . kind () && let ty :: Param (param_ty) = ty . kind () { let def_id = self . tcx . hir_enclosing_body_owner (self . id) ; let generics = self . tcx . generics_of (def_id) ; let param = generics . type_param (* param_ty , self . tcx) ; let span = self . tcx . def_span (param . def_id) ; e . span_label (span , "constant depends on this generic parameter") ; if let Some (ident) = self . tcx . def_ident_span (def_id) && self . tcx . sess . source_map () . is_multiline (ident . between (span)) { e . span_label (ident , "") ; } } } return self . mk_err (e , ty) ; } Ok (Err (bad_ty)) => { let e = match bad_ty . kind () { ty :: Adt (def , ..) => { assert ! (def . is_union ()) ; self . tcx . dcx () . create_err (UnionPattern { span : self . span }) } ty :: FnPtr (..) | ty :: RawPtr (..) => { self . tcx . dcx () . create_err (PointerPattern { span : self . span }) } _ => self . tcx . dcx () . create_err (InvalidPattern { span : self . span , non_sm_ty : bad_ty , prefix : bad_ty . prefix_string (self . tcx) . to_string () , }) , } ; return self . mk_err (e , ty) ; } } ; let inlined_const_as_pat = self . valtree_to_pat (valtree , ty) ; if ! inlined_const_as_pat . references_error () { if ! type_has_partial_eq_impl (self . tcx , typing_env , ty) . has_impl { let mut err = self . tcx . dcx () . create_err (TypeNotPartialEq { span : self . span , ty }) ; extend_type_not_partial_eq (self . tcx , typing_env , ty , & mut err) ; return self . mk_err (err , ty) ; } } let kind = PatKind :: ExpandedConstant { subpattern : inlined_const_as_pat , def_id : uv . def } ; Box :: new (Pat { kind , ty , span : self . span }) } fn field_pats (& self , vals : impl Iterator < Item = (ValTree < 'tcx > , Ty < 'tcx >) > ,) -> Vec < FieldPat < 'tcx > > { vals . enumerate () . map (| (idx , (val , ty)) | { let field = FieldIdx :: new (idx) ; let ty = self . tcx . normalize_erasing_regions (self . typing_env , ty) ; FieldPat { field , pattern : * self . valtree_to_pat (val , ty) } }) . collect () } # [instrument (skip (self) , level = "debug")] fn valtree_to_pat (& self , cv : ValTree < 'tcx > , ty : Ty < 'tcx >) -> Box < Pat < 'tcx > > { let span = self . span ; let tcx = self . tcx ; let kind = match ty . kind () { ty :: Adt (adt_def , _) if ! self . type_marked_structural (ty) => { debug ! ("adt_def {:?} has !type_marked_structural for cv.ty: {:?}" , adt_def , ty) ; let PartialEqImplStatus { is_derived , structural_partial_eq , non_blanket_impl , .. } = type_has_partial_eq_impl (self . tcx , self . typing_env , ty) ; let (manual_partialeq_impl_span , manual_partialeq_impl_note) = match (structural_partial_eq , non_blanket_impl) { (true , _) => (None , false) , (_ , Some (def_id)) if def_id . is_local () && ! is_derived => { (Some (tcx . def_span (def_id)) , false) } _ => (None , true) , } ; let ty_def_span = tcx . def_span (adt_def . did ()) ; let err = TypeNotStructural { span , ty , ty_def_span , manual_partialeq_impl_span , manual_partialeq_impl_note , } ; return self . mk_err (tcx . dcx () . create_err (err) , ty) ; } ty :: Adt (adt_def , args) if adt_def . is_enum () => { let (& variant_index , fields) = cv . unwrap_branch () . split_first () . unwrap () ; let variant_index = VariantIdx :: from_u32 (variant_index . unwrap_leaf () . to_u32 ()) ; PatKind :: Variant { adt_def : * adt_def , args , variant_index , subpatterns : self . field_pats (fields . iter () . copied () . zip (adt_def . variants () [variant_index] . fields . iter () . map (| field | field . ty (tcx , args)) ,) ,) , } } ty :: Adt (def , args) => { assert ! (! def . is_union ()) ; PatKind :: Leaf { subpatterns : self . field_pats (cv . unwrap_branch () . iter () . copied () . zip (def . non_enum_variant () . fields . iter () . map (| field | field . ty (tcx , args)) ,)) , } } ty :: Tuple (fields) => PatKind :: Leaf { subpatterns : self . field_pats (cv . unwrap_branch () . iter () . copied () . zip (fields . iter ())) , } , ty :: Slice (elem_ty) => PatKind :: Slice { prefix : cv . unwrap_branch () . iter () . map (| val | * self . valtree_to_pat (* val , * elem_ty)) . collect () , slice : None , suffix : Box :: new ([]) , } , ty :: Array (elem_ty , _) => PatKind :: Array { prefix : cv . unwrap_branch () . iter () . map (| val | * self . valtree_to_pat (* val , * elem_ty)) . collect () , slice : None , suffix : Box :: new ([]) , } , ty :: Str => { let ref_str_ty = Ty :: new_imm_ref (tcx , tcx . lifetimes . re_erased , ty) ; PatKind :: Constant { value : ty :: Value { ty : ref_str_ty , valtree : cv } } } ty :: Ref (_ , pointee_ty , ..) => match * pointee_ty . kind () { ty :: Str => PatKind :: Constant { value : ty :: Value { ty , valtree : cv } } , _ => { if ! pointee_ty . is_sized (tcx , self . typing_env) && ! pointee_ty . is_slice () { return self . mk_err (tcx . dcx () . create_err (UnsizedPattern { span , non_sm_ty : * pointee_ty }) , ty ,) ; } else { PatKind :: Deref { subpattern : self . valtree_to_pat (cv , * pointee_ty) } } } } , ty :: Float (flt) => { let v = cv . unwrap_leaf () ; let is_nan = match flt { ty :: FloatTy :: F16 => v . to_f16 () . is_nan () , ty :: FloatTy :: F32 => v . to_f32 () . is_nan () , ty :: FloatTy :: F64 => v . to_f64 () . is_nan () , ty :: FloatTy :: F128 => v . to_f128 () . is_nan () , } ; if is_nan { return self . mk_err (tcx . dcx () . create_err (NaNPattern { span }) , ty) ; } else { PatKind :: Constant { value : ty :: Value { ty , valtree : cv } } } } ty :: Pat (..) | ty :: Bool | ty :: Char | ty :: Int (_) | ty :: Uint (_) | ty :: RawPtr (..) => { PatKind :: Constant { value : ty :: Value { ty , valtree : cv } } } ty :: FnPtr (..) => { unreachable ! ("Valtree construction would never succeed for FnPtr, so this is unreachable.") } _ => { let err = InvalidPattern { span , non_sm_ty : ty , prefix : ty . prefix_string (tcx) . to_string () , } ; return self . mk_err (tcx . dcx () . create_err (err) , ty) ; } } ; Box :: new (Pat { span , ty , kind }) } }}}

macro_rules! extend_type_not_partial_eq_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function extend_type_not_partial_eq in module {}", module_path!());
    };
}

mkfn!{
    extend_type_not_partial_eq_introspect!();
    # [doc = " Given a type with type parameters, visit every ADT looking for types that need to"] # [doc = " `#[derive(PartialEq)]` for it to be a structural type."] fn extend_type_not_partial_eq < 'tcx > (tcx : TyCtxt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > , ty : Ty < 'tcx > , err : & mut Diag < '_ > ,) { # [doc = " Collect all types that need to be `StructuralPartialEq`."] struct UsedParamsNeedInstantiationVisitor < 'tcx > { tcx : TyCtxt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > , # [doc = " The user has written `impl PartialEq for Ty` which means it's non-structural."] adts_with_manual_partialeq : FxHashSet < Span > , # [doc = " The type has no `PartialEq` implementation, neither manual or derived."] adts_without_partialeq : FxHashSet < Span > , # [doc = " The user has written `impl PartialEq for Ty` which means it's non-structural,"] # [doc = " but we don't have a span to point at, so we'll just add them as a `note`."] manual : FxHashSet < Ty < 'tcx > > , # [doc = " The type has no `PartialEq` implementation, neither manual or derived, but"] # [doc = " we don't have a span to point at, so we'll just add them as a `note`."] without : FxHashSet < Ty < 'tcx > > , } impl < 'tcx > TypeVisitor < TyCtxt < 'tcx > > for UsedParamsNeedInstantiationVisitor < 'tcx > { type Result = ControlFlow < () > ; fn visit_ty (& mut self , ty : Ty < 'tcx >) -> Self :: Result { match ty . kind () { ty :: Dynamic (..) => return ControlFlow :: Break (()) , ty :: UnsafeBinder (..) => return ControlFlow :: Break (()) , ty :: FnPtr (..) => return ControlFlow :: Continue (()) , ty :: Adt (def , _args) => { let ty_def_id = def . did () ; let ty_def_span = self . tcx . def_span (ty_def_id) ; let PartialEqImplStatus { has_impl , is_derived , structural_partial_eq , non_blanket_impl , } = type_has_partial_eq_impl (self . tcx , self . typing_env , ty) ; match (has_impl , is_derived , structural_partial_eq , non_blanket_impl) { (_ , _ , true , _) => { } (true , false , _ , Some (def_id)) if def_id . is_local () => { self . adts_with_manual_partialeq . insert (self . tcx . def_span (def_id)) ; } (true , false , _ , _) if ty_def_id . is_local () => { self . adts_with_manual_partialeq . insert (ty_def_span) ; } (false , _ , _ , _) if ty_def_id . is_local () => { self . adts_without_partialeq . insert (ty_def_span) ; } (true , false , _ , _) => { self . manual . insert (ty) ; } (false , _ , _ , _) => { self . without . insert (ty) ; } _ => { } } ; ty . super_visit_with (self) } _ => ty . super_visit_with (self) , } } } let mut v = UsedParamsNeedInstantiationVisitor { tcx , typing_env , adts_with_manual_partialeq : FxHashSet :: default () , adts_without_partialeq : FxHashSet :: default () , manual : FxHashSet :: default () , without : FxHashSet :: default () , } ; if v . visit_ty (ty) . is_break () { return ; } # [allow (rustc :: potential_query_instability)] for span in v . adts_with_manual_partialeq { err . span_note (span , "the `PartialEq` trait must be derived, manual `impl`s are not sufficient; see https://doc.rust-lang.org/stable/std/marker/trait.StructuralPartialEq.html for details") ; } # [allow (rustc :: potential_query_instability)] for span in v . adts_without_partialeq { err . span_label (span , "must be annotated with `#[derive(PartialEq)]` to be usable in patterns" ,) ; } # [allow (rustc :: potential_query_instability)] let mut manual : Vec < _ > = v . manual . into_iter () . map (| t | t . to_string ()) . collect () ; manual . sort () ; for ty in manual { err . note (format ! ("`{ty}` must be annotated with `#[derive(PartialEq)]` to be usable in patterns, manual `impl`s are not sufficient; see https://doc.rust-lang.org/stable/std/marker/trait.StructuralPartialEq.html for details")) ; } # [allow (rustc :: potential_query_instability)] let mut without : Vec < _ > = v . without . into_iter () . map (| t | t . to_string ()) . collect () ; without . sort () ; for ty in without { err . note (format ! ("`{ty}` must be annotated with `#[derive(PartialEq)]` to be usable in patterns")) ; } }
}
mkitem!{mkstruct!{# [derive (Debug)] struct PartialEqImplStatus { has_impl : bool , is_derived : bool , structural_partial_eq : bool , non_blanket_impl : Option < DefId > , }}}

macro_rules! type_has_partial_eq_impl_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function type_has_partial_eq_impl in module {}", module_path!());
    };
}

mkfn!{
    type_has_partial_eq_impl_introspect!();
    # [instrument (level = "trace" , skip (tcx) , ret)] fn type_has_partial_eq_impl < 'tcx > (tcx : TyCtxt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > , ty : Ty < 'tcx > ,) -> PartialEqImplStatus { let (infcx , param_env) = tcx . infer_ctxt () . build_with_typing_env (typing_env) ; let partial_eq_trait_id = tcx . require_lang_item (hir :: LangItem :: PartialEq , DUMMY_SP) ; let structural_partial_eq_trait_id = tcx . require_lang_item (hir :: LangItem :: StructuralPeq , DUMMY_SP) ; let partial_eq_obligation = Obligation :: new (tcx , ObligationCause :: dummy () , param_env , ty :: TraitRef :: new (tcx , partial_eq_trait_id , [ty , ty]) ,) ; let mut automatically_derived = false ; let mut structural_peq = false ; let mut impl_def_id = None ; for def_id in tcx . non_blanket_impls_for_ty (partial_eq_trait_id , ty) { automatically_derived = find_attr ! (tcx . get_all_attrs (def_id) , AttributeKind :: AutomaticallyDerived (..)) ; impl_def_id = Some (def_id) ; } for _ in tcx . non_blanket_impls_for_ty (structural_partial_eq_trait_id , ty) { structural_peq = true ; } PartialEqImplStatus { has_impl : infcx . predicate_must_hold_modulo_regions (& partial_eq_obligation) , is_derived : automatically_derived , structural_partial_eq : structural_peq , non_blanket_impl : impl_def_id , } }
}