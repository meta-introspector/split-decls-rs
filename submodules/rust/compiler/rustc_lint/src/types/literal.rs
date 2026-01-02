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
mkuse!{use hir :: { ExprKind , Node , is_range_literal } ;}
mkuse!{use rustc_abi :: { Integer , Size } ;}
mkuse!{use rustc_hir :: { HirId , attrs } ;}
mkuse!{use rustc_middle :: ty :: Ty ;}
mkuse!{use rustc_middle :: ty :: layout :: IntegerExt ;}
mkuse!{use rustc_middle :: { bug , ty } ;}
mkuse!{use rustc_span :: Span ;}
mkuse!{use { rustc_ast as ast , rustc_hir as hir } ;}
mkuse!{use crate :: LateContext ;}
mkuse!{use crate :: context :: LintContext ;}
mkuse!{use crate :: lints :: { OnlyCastu8ToChar , OverflowingBinHex , OverflowingBinHexSign , OverflowingBinHexSignBitSub , OverflowingBinHexSub , OverflowingInt , OverflowingIntHelp , OverflowingLiteral , OverflowingUInt , RangeEndpointOutOfRange , SurrogateCharCast , TooLargeCharCast , UseInclusiveRange , } ;}
mkuse!{use crate :: types :: { OVERFLOWING_LITERALS , TypeLimits } ;}

macro_rules! lint_overflowing_range_endpoint_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lint_overflowing_range_endpoint in module {}", module_path!());
    };
}

mkfn!{
    lint_overflowing_range_endpoint_introspect!();
    # [doc = " Attempts to special-case the overflowing literal lint when it occurs as a range endpoint (`expr..MAX+1`)."] # [doc = " Returns `true` iff the lint was emitted."] fn lint_overflowing_range_endpoint < 'tcx > (cx : & LateContext < 'tcx > , lit : & hir :: Lit , lit_val : u128 , max : u128 , hir_id : HirId , lit_span : Span , ty : & str ,) -> bool { let (hir_id , span) = if let Node :: Expr (par_expr) = cx . tcx . parent_hir_node (hir_id) && let ExprKind :: Cast (_ , _) = par_expr . kind { (par_expr . hir_id , par_expr . span) } else { (hir_id , lit_span) } ; let Node :: ExprField (field) = cx . tcx . parent_hir_node (hir_id) else { return false ; } ; let Node :: Expr (struct_expr) = cx . tcx . parent_hir_node (field . hir_id) else { return false ; } ; if ! is_range_literal (struct_expr) { return false ; } ; let ExprKind :: Struct (_ , [start , end] , _) = & struct_expr . kind else { return false ; } ; if ! (end . expr . hir_id == hir_id && lit_val - 1 == max) { return false ; } ; use rustc_ast :: { LitIntType , LitKind } ; let suffix = match lit . node { LitKind :: Int (_ , LitIntType :: Signed (s)) => s . name_str () , LitKind :: Int (_ , LitIntType :: Unsigned (s)) => s . name_str () , LitKind :: Int (_ , LitIntType :: Unsuffixed) => "" , _ => bug ! () , } ; let sub_sugg = if span . lo () == lit_span . lo () { let Ok (start) = cx . sess () . source_map () . span_to_snippet (start . span) else { return false ; } ; UseInclusiveRange :: WithoutParen { sugg : struct_expr . span . shrink_to_lo () . to (lit_span . shrink_to_hi ()) , start , literal : lit_val - 1 , suffix , } } else { UseInclusiveRange :: WithParen { eq_sugg : span . shrink_to_lo () , lit_sugg : lit_span , literal : lit_val - 1 , suffix , } } ; cx . emit_span_lint (OVERFLOWING_LITERALS , struct_expr . span , RangeEndpointOutOfRange { ty , sub : sub_sugg } ,) ; true }
}

macro_rules! int_ty_range_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function int_ty_range in module {}", module_path!());
    };
}

mkfn!{
    int_ty_range_introspect!();
    pub (crate) fn int_ty_range (int_ty : ty :: IntTy) -> (i128 , i128) { match int_ty { ty :: IntTy :: Isize => (i64 :: MIN . into () , i64 :: MAX . into ()) , ty :: IntTy :: I8 => (i8 :: MIN . into () , i8 :: MAX . into ()) , ty :: IntTy :: I16 => (i16 :: MIN . into () , i16 :: MAX . into ()) , ty :: IntTy :: I32 => (i32 :: MIN . into () , i32 :: MAX . into ()) , ty :: IntTy :: I64 => (i64 :: MIN . into () , i64 :: MAX . into ()) , ty :: IntTy :: I128 => (i128 :: MIN , i128 :: MAX) , } }
}

macro_rules! uint_ty_range_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function uint_ty_range in module {}", module_path!());
    };
}

mkfn!{
    uint_ty_range_introspect!();
    pub (crate) fn uint_ty_range (uint_ty : ty :: UintTy) -> (u128 , u128) { let max = match uint_ty { ty :: UintTy :: Usize => u64 :: MAX . into () , ty :: UintTy :: U8 => u8 :: MAX . into () , ty :: UintTy :: U16 => u16 :: MAX . into () , ty :: UintTy :: U32 => u32 :: MAX . into () , ty :: UintTy :: U64 => u64 :: MAX . into () , ty :: UintTy :: U128 => u128 :: MAX , } ; (0 , max) }
}

macro_rules! get_bin_hex_repr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_bin_hex_repr in module {}", module_path!());
    };
}

mkfn!{
    get_bin_hex_repr_introspect!();
    fn get_bin_hex_repr (cx : & LateContext < '_ > , lit : & hir :: Lit) -> Option < String > { let src = cx . sess () . source_map () . span_to_snippet (lit . span) . ok () ? ; let firstch = src . chars () . next () ? ; if firstch == '0' { match src . chars () . nth (1) { Some ('x' | 'b') => return Some (src) , _ => return None , } } None }
}

macro_rules! report_bin_hex_error_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function report_bin_hex_error in module {}", module_path!());
    };
}

mkfn!{
    report_bin_hex_error_introspect!();
    fn report_bin_hex_error (cx : & LateContext < '_ > , hir_id : HirId , span : Span , ty : attrs :: IntType , size : Size , repr_str : String , val : u128 , negative : bool ,) { let (t , actually) = match ty { attrs :: IntType :: SignedInt (t) => { let actually = if negative { - (size . sign_extend (val)) } else { size . sign_extend (val) } ; (t . name_str () , actually . to_string ()) } attrs :: IntType :: UnsignedInt (t) => { let actually = size . truncate (val) ; (t . name_str () , actually . to_string ()) } } ; let sign = if negative { OverflowingBinHexSign :: Negative } else { OverflowingBinHexSign :: Positive } ; let sub = get_type_suggestion (cx . typeck_results () . node_type (hir_id) , val , negative) . map (| suggestion_ty | { if let Some (pos) = repr_str . chars () . position (| c | c == 'i' || c == 'u') { let (sans_suffix , _) = repr_str . split_at (pos) ; OverflowingBinHexSub :: Suggestion { span , suggestion_ty , sans_suffix } } else { OverflowingBinHexSub :: Help { suggestion_ty } } } ,) ; let sign_bit_sub = (! negative) . then (| | { let ty :: Int (int_ty) = cx . typeck_results () . node_type (hir_id) . kind () else { return None ; } ; let Some (bit_width) = int_ty . bit_width () else { return None ; } ; if (val & (1 << (bit_width - 1))) == 0 { return None ; } let lit_no_suffix = if let Some (pos) = repr_str . chars () . position (| c | c == 'i' || c == 'u') { repr_str . split_at (pos) . 0 } else { & repr_str } ; Some (OverflowingBinHexSignBitSub { span , lit_no_suffix , negative_val : actually . clone () , int_ty : int_ty . name_str () , uint_ty : Integer :: fit_unsigned (val) . uint_ty_str () , }) }) . flatten () ; cx . emit_span_lint (OVERFLOWING_LITERALS , span , OverflowingBinHex { ty : t , lit : repr_str . clone () , dec : val , actually , sign , sub , sign_bit_sub , } ,) }
}

macro_rules! get_type_suggestion_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_type_suggestion in module {}", module_path!());
    };
}

mkfn!{
    get_type_suggestion_introspect!();
    fn get_type_suggestion (t : Ty < '_ > , val : u128 , negative : bool) -> Option < & 'static str > { match t . kind () { ty :: Uint (ty :: UintTy :: Usize) | ty :: Int (ty :: IntTy :: Isize) => None , ty :: Uint (_) => Some (Integer :: fit_unsigned (val) . uint_ty_str ()) , ty :: Int (_) => { let signed = literal_to_i128 (val , negative) . map (Integer :: fit_signed) ; if negative { signed . map (Integer :: int_ty_str) } else { let unsigned = Integer :: fit_unsigned (val) ; Some (if let Some (signed) = signed { if unsigned . size () < signed . size () { unsigned . uint_ty_str () } else { signed . int_ty_str () } } else { unsigned . uint_ty_str () }) } } _ => None , } }
}

macro_rules! literal_to_i128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function literal_to_i128 in module {}", module_path!());
    };
}

mkfn!{
    literal_to_i128_introspect!();
    fn literal_to_i128 (val : u128 , negative : bool) -> Option < i128 > { if negative { (val <= i128 :: MAX as u128 + 1) . then (| | val . wrapping_neg () as i128) } else { val . try_into () . ok () } }
}

macro_rules! lint_int_literal_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lint_int_literal in module {}", module_path!());
    };
}

mkfn!{
    lint_int_literal_introspect!();
    fn lint_int_literal < 'tcx > (cx : & LateContext < 'tcx > , type_limits : & TypeLimits , hir_id : HirId , span : Span , lit : & hir :: Lit , t : ty :: IntTy , v : u128 ,) { let int_type = t . normalize (cx . sess () . target . pointer_width) ; let (min , max) = int_ty_range (int_type) ; let max = max as u128 ; let negative = type_limits . negated_expr_id == Some (hir_id) ; if (negative && v > max + 1) || (! negative && v > max) { if let Some (repr_str) = get_bin_hex_repr (cx , lit) { report_bin_hex_error (cx , hir_id , span , attrs :: IntType :: SignedInt (t) , Integer :: from_int_ty (cx , t) . size () , repr_str , v , negative ,) ; return ; } if lint_overflowing_range_endpoint (cx , lit , v , max , hir_id , span , t . name_str ()) { return ; } let span = if negative { type_limits . negated_expr_span . unwrap () } else { span } ; let lit = cx . sess () . source_map () . span_to_snippet (span) . unwrap_or_else (| _ | if negative { format ! ("-{v}") } else { v . to_string () }) ; let help = get_type_suggestion (cx . typeck_results () . node_type (hir_id) , v , negative) . map (| suggestion_ty | OverflowingIntHelp { suggestion_ty }) ; cx . emit_span_lint (OVERFLOWING_LITERALS , span , OverflowingInt { ty : t . name_str () , lit , min , max , help } ,) ; } }
}

macro_rules! lint_uint_literal_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lint_uint_literal in module {}", module_path!());
    };
}

mkfn!{
    lint_uint_literal_introspect!();
    fn lint_uint_literal < 'tcx > (cx : & LateContext < 'tcx > , hir_id : HirId , span : Span , lit : & hir :: Lit , t : ty :: UintTy ,) { let uint_type = t . normalize (cx . sess () . target . pointer_width) ; let (min , max) = uint_ty_range (uint_type) ; let lit_val : u128 = match lit . node { ast :: LitKind :: Byte (_v) => return , ast :: LitKind :: Int (v , _) => v . get () , _ => bug ! () , } ; if lit_val < min || lit_val > max { if let Node :: Expr (par_e) = cx . tcx . parent_hir_node (hir_id) { match par_e . kind { hir :: ExprKind :: Cast (..) => { if let ty :: Char = cx . typeck_results () . expr_ty (par_e) . kind () { if lit_val > 0x10FFFF { cx . emit_span_lint (OVERFLOWING_LITERALS , par_e . span , TooLargeCharCast { literal : lit_val } ,) ; } else if (0xD800 ..= 0xDFFF) . contains (& lit_val) { cx . emit_span_lint (OVERFLOWING_LITERALS , par_e . span , SurrogateCharCast { literal : lit_val } ,) ; } else { cx . emit_span_lint (OVERFLOWING_LITERALS , par_e . span , OnlyCastu8ToChar { span : par_e . span , literal : lit_val } ,) ; } return ; } } _ => { } } } if lint_overflowing_range_endpoint (cx , lit , lit_val , max , hir_id , span , t . name_str ()) { return ; } if let Some (repr_str) = get_bin_hex_repr (cx , lit) { report_bin_hex_error (cx , hir_id , span , attrs :: IntType :: UnsignedInt (t) , Integer :: from_uint_ty (cx , t) . size () , repr_str , lit_val , false ,) ; return ; } cx . emit_span_lint (OVERFLOWING_LITERALS , span , OverflowingUInt { ty : t . name_str () , lit : cx . sess () . source_map () . span_to_snippet (lit . span) . unwrap_or_else (| _ | lit_val . to_string ()) , min , max , } ,) ; } }
}

macro_rules! lint_literal_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lint_literal in module {}", module_path!());
    };
}

mkfn!{
    lint_literal_introspect!();
    pub (crate) fn lint_literal < 'tcx > (cx : & LateContext < 'tcx > , type_limits : & TypeLimits , hir_id : HirId , span : Span , lit : & hir :: Lit , negated : bool ,) { match * cx . typeck_results () . node_type (hir_id) . kind () { ty :: Int (t) => { match lit . node { ast :: LitKind :: Int (v , ast :: LitIntType :: Signed (_) | ast :: LitIntType :: Unsuffixed) => { lint_int_literal (cx , type_limits , hir_id , span , lit , t , v . get ()) } _ => bug ! () , } ; } ty :: Uint (t) => { assert ! (! negated) ; lint_uint_literal (cx , hir_id , span , lit , t) } ty :: Float (t) => { let (is_infinite , sym) = match lit . node { ast :: LitKind :: Float (v , _) => match t { ty :: FloatTy :: F16 => (Ok (false) , v) , ty :: FloatTy :: F32 => (v . as_str () . parse () . map (f32 :: is_infinite) , v) , ty :: FloatTy :: F64 => (v . as_str () . parse () . map (f64 :: is_infinite) , v) , ty :: FloatTy :: F128 => (Ok (false) , v) , } , _ => bug ! () , } ; if is_infinite == Ok (true) { cx . emit_span_lint (OVERFLOWING_LITERALS , span , OverflowingLiteral { ty : t . name_str () , lit : cx . sess () . source_map () . span_to_snippet (lit . span) . unwrap_or_else (| _ | sym . to_string ()) , } ,) ; } } _ => { } } }
}