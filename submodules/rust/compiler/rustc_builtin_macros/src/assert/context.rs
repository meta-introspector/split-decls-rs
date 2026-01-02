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
mkuse!{use rustc_ast :: token :: { self , Delimiter , IdentIsRaw } ;}
mkuse!{use rustc_ast :: tokenstream :: { DelimSpan , TokenStream , TokenTree } ;}
mkuse!{use rustc_ast :: { BinOpKind , BorrowKind , DUMMY_NODE_ID , DelimArgs , Expr , ExprKind , ItemKind , MacCall , MethodCall , Mutability , Path , PathSegment , Stmt , StructRest , UnOp , UseTree , UseTreeKind , } ;}
mkuse!{use rustc_ast_pretty :: pprust ;}
mkuse!{use rustc_data_structures :: fx :: FxHashSet ;}
mkuse!{use rustc_expand :: base :: ExtCtxt ;}
mkuse!{use rustc_span :: { Ident , Span , Symbol , sym } ;}
mkuse!{use thin_vec :: { ThinVec , thin_vec } ;}
mkitem!{mkstruct!{pub (super) struct Context < 'cx , 'a > { best_case_captures : Vec < Stmt > , capture_decls : Vec < Capture > , cx : & 'cx ExtCtxt < 'a > , fmt_string : String , is_consumed : bool , local_bind_decls : Vec < Stmt > , paths : FxHashSet < Ident > , span : Span , }}}
mkitem!{mkimpl!{impl < 'cx , 'a > Context < 'cx , 'a > { pub (super) fn new (cx : & 'cx ExtCtxt < 'a > , span : Span) -> Self { Self { best_case_captures : < _ > :: default () , capture_decls : < _ > :: default () , cx , fmt_string : < _ > :: default () , is_consumed : true , local_bind_decls : < _ > :: default () , paths : < _ > :: default () , span , } } # [doc = " Builds the whole `assert!` expression. For example, `let elem = 1; assert!(elem == 1);` expands to:"] # [doc = ""] # [doc = " ```rust"] # [doc = " let elem = 1;"] # [doc = " {"] # [doc = "   #[allow(unused_imports)]"] # [doc = "   use ::core::asserting::{TryCaptureGeneric, TryCapturePrintable};"] # [doc = "   let mut __capture0 = ::core::asserting::Capture::new();"] # [doc = "   let __local_bind0 = &elem;"] # [doc = "   if !("] # [doc = "     *{"] # [doc = "       (&::core::asserting::Wrapper(__local_bind0)).try_capture(&mut __capture0);"] # [doc = "       __local_bind0"] # [doc = "     } == 1"] # [doc = "   ) {"] # [doc = "     panic!(\"Assertion failed: elem == 1\\nWith captures:\\n  elem = {:?}\", __capture0)"] # [doc = "   }"] # [doc = " }"] # [doc = " ```"] pub (super) fn build (mut self , mut cond_expr : Box < Expr > , panic_path : Path) -> Box < Expr > { let expr_str = pprust :: expr_to_string (& cond_expr) ; self . manage_cond_expr (& mut cond_expr) ; let initial_imports = self . build_initial_imports () ; let panic = self . build_panic (& expr_str , panic_path) ; let cond_expr_with_unlikely = self . build_unlikely (cond_expr) ; let Self { best_case_captures , capture_decls , cx , local_bind_decls , span , .. } = self ; let mut assert_then_stmts = ThinVec :: with_capacity (2) ; assert_then_stmts . extend (best_case_captures) ; assert_then_stmts . push (self . cx . stmt_expr (panic)) ; let assert_then = self . cx . block (span , assert_then_stmts) ; let mut stmts = ThinVec :: with_capacity (4) ; stmts . push (initial_imports) ; stmts . extend (capture_decls . into_iter () . map (| c | c . decl)) ; stmts . extend (local_bind_decls) ; stmts . push (cx . stmt_expr (cx . expr (span , ExprKind :: If (cond_expr_with_unlikely , assert_then , None))) ,) ; cx . expr_block (cx . block (span , stmts)) } # [doc = " Initial **trait** imports"] # [doc = ""] # [doc = " use ::core::asserting::{ ... };"] fn build_initial_imports (& self) -> Stmt { let nested_tree = | this : & Self , sym | { (UseTree { prefix : this . cx . path (this . span , vec ! [Ident :: with_dummy_span (sym)]) , kind : UseTreeKind :: Simple (None) , span : this . span , } , DUMMY_NODE_ID ,) } ; self . cx . stmt_item (self . span , self . cx . item (self . span , thin_vec ! [self . cx . attr_nested_word (sym :: allow , sym :: unused_imports , self . span)] , ItemKind :: Use (UseTree { prefix : self . cx . path (self . span , self . cx . std_path (& [sym :: asserting])) , kind : UseTreeKind :: Nested { items : thin_vec ! [nested_tree (self , sym :: TryCaptureGeneric) , nested_tree (self , sym :: TryCapturePrintable) ,] , span : self . span , } , span : self . span , }) ,) ,) } # [doc = " Takes the conditional expression of `assert!` and then wraps it inside `unlikely`"] fn build_unlikely (& self , cond_expr : Box < Expr >) -> Box < Expr > { let unlikely_path = self . cx . std_path (& [sym :: intrinsics , sym :: unlikely]) ; self . cx . expr_call (self . span , self . cx . expr_path (self . cx . path (self . span , unlikely_path)) , thin_vec ! [self . cx . expr (self . span , ExprKind :: Unary (UnOp :: Not , cond_expr))] ,) } # [doc = " The necessary custom `panic!(...)` expression."] # [doc = ""] # [doc = " panic!("] # [doc = "     \"Assertion failed: ... \\n With expansion: ...\","] # [doc = "     __capture0,"] # [doc = "     ..."] # [doc = " );"] fn build_panic (& self , expr_str : & str , panic_path : Path) -> Box < Expr > { let escaped_expr_str = escape_to_fmt (expr_str) ; let initial = [TokenTree :: token_joint (token :: Literal (token :: Lit { kind : token :: LitKind :: Str , symbol : Symbol :: intern (& if self . fmt_string . is_empty () { format ! ("Assertion failed: {escaped_expr_str}") } else { format ! ("Assertion failed: {escaped_expr_str}\nWith captures:\n{}" , & self . fmt_string) }) , suffix : None , }) , self . span ,) , TokenTree :: token_alone (token :: Comma , self . span) ,] ; let captures = self . capture_decls . iter () . flat_map (| cap | { [TokenTree :: token_joint (token :: Ident (cap . ident . name , IdentIsRaw :: No) , cap . ident . span ,) , TokenTree :: token_alone (token :: Comma , self . span) ,] }) ; self . cx . expr (self . span , ExprKind :: MacCall (Box :: new (MacCall { path : panic_path , args : Box :: new (DelimArgs { dspan : DelimSpan :: from_single (self . span) , delim : Delimiter :: Parenthesis , tokens : initial . into_iter () . chain (captures) . collect :: < TokenStream > () , }) , })) ,) } # [doc = " Recursive function called until `cond_expr` and `fmt_str` are fully modified."] # [doc = ""] # [doc = " See [Self::manage_initial_capture] and [Self::manage_try_capture]"] fn manage_cond_expr (& mut self , expr : & mut Box < Expr >) { match & mut expr . kind { ExprKind :: AddrOf (_ , mutability , local_expr) => { self . with_is_consumed_management (matches ! (mutability , Mutability :: Mut) , | this | { this . manage_cond_expr (local_expr) }) ; } ExprKind :: Array (local_exprs) => { for local_expr in local_exprs { self . manage_cond_expr (local_expr) ; } } ExprKind :: Binary (op , lhs , rhs) => { self . with_is_consumed_management (matches ! (op . node , BinOpKind :: Add | BinOpKind :: And | BinOpKind :: BitAnd | BinOpKind :: BitOr | BinOpKind :: BitXor | BinOpKind :: Div | BinOpKind :: Mul | BinOpKind :: Or | BinOpKind :: Rem | BinOpKind :: Shl | BinOpKind :: Shr | BinOpKind :: Sub) , | this | { this . manage_cond_expr (lhs) ; this . manage_cond_expr (rhs) ; } ,) ; } ExprKind :: Call (_ , local_exprs) => { for local_expr in local_exprs { self . manage_cond_expr (local_expr) ; } } ExprKind :: Cast (local_expr , _) => { self . manage_cond_expr (local_expr) ; } ExprKind :: If (local_expr , _ , _) => { self . manage_cond_expr (local_expr) ; } ExprKind :: Index (prefix , suffix , _) => { self . manage_cond_expr (prefix) ; self . manage_cond_expr (suffix) ; } ExprKind :: Let (_ , local_expr , _ , _) => { self . manage_cond_expr (local_expr) ; } ExprKind :: Match (local_expr , ..) => { self . manage_cond_expr (local_expr) ; } ExprKind :: MethodCall (call) => { for arg in & mut call . args { self . manage_cond_expr (arg) ; } } ExprKind :: Path (_ , Path { segments , .. }) if let [path_segment] = & segments [..] => { let path_ident = path_segment . ident ; self . manage_initial_capture (expr , path_ident) ; } ExprKind :: Paren (local_expr) => { self . manage_cond_expr (local_expr) ; } ExprKind :: Range (prefix , suffix , _) => { if let Some (elem) = prefix { self . manage_cond_expr (elem) ; } if let Some (elem) = suffix { self . manage_cond_expr (elem) ; } } ExprKind :: Repeat (local_expr , elem) => { self . manage_cond_expr (local_expr) ; self . manage_cond_expr (& mut elem . value) ; } ExprKind :: Struct (elem) => { for field in & mut elem . fields { self . manage_cond_expr (& mut field . expr) ; } if let StructRest :: Base (local_expr) = & mut elem . rest { self . manage_cond_expr (local_expr) ; } } ExprKind :: Tup (local_exprs) => { for local_expr in local_exprs { self . manage_cond_expr (local_expr) ; } } ExprKind :: Unary (un_op , local_expr) => { self . with_is_consumed_management (matches ! (un_op , UnOp :: Neg | UnOp :: Not) , | this | { this . manage_cond_expr (local_expr) }) ; } ExprKind :: Assign (_ , _ , _) | ExprKind :: AssignOp (_ , _ , _) | ExprKind :: Gen (_ , _ , _ , _) | ExprKind :: Await (_ , _) | ExprKind :: Use (_ , _) | ExprKind :: Block (_ , _) | ExprKind :: Break (_ , _) | ExprKind :: Closure (_) | ExprKind :: ConstBlock (_) | ExprKind :: Continue (_) | ExprKind :: Dummy | ExprKind :: Err (_) | ExprKind :: Field (_ , _) | ExprKind :: ForLoop { .. } | ExprKind :: FormatArgs (_) | ExprKind :: IncludedBytes (..) | ExprKind :: InlineAsm (_) | ExprKind :: Lit (_) | ExprKind :: Loop (_ , _ , _) | ExprKind :: MacCall (_) | ExprKind :: OffsetOf (_ , _) | ExprKind :: Path (_ , _) | ExprKind :: Ret (_) | ExprKind :: Try (_) | ExprKind :: TryBlock (_) | ExprKind :: Type (_ , _) | ExprKind :: Underscore | ExprKind :: While (_ , _ , _) | ExprKind :: Yeet (_) | ExprKind :: Become (_) | ExprKind :: Yield (_) | ExprKind :: UnsafeBinderCast (..) => { } } } # [doc = " Pushes the top-level declarations and modifies `expr` to try capturing variables."] # [doc = ""] # [doc = " `fmt_str`, the formatting string used for debugging, is constructed to show possible"] # [doc = " captured variables."] fn manage_initial_capture (& mut self , expr : & mut Box < Expr > , path_ident : Ident) { if self . paths . contains (& path_ident) { return ; } else { self . fmt_string . push_str ("  ") ; self . fmt_string . push_str (path_ident . as_str ()) ; self . fmt_string . push_str (" = {:?}\n") ; let _ = self . paths . insert (path_ident) ; } let curr_capture_idx = self . capture_decls . len () ; let capture_string = format ! ("__capture{curr_capture_idx}") ; let ident = Ident :: new (Symbol :: intern (& capture_string) , self . span) ; let init_std_path = self . cx . std_path (& [sym :: asserting , sym :: Capture , sym :: new]) ; let init = self . cx . expr_call (self . span , self . cx . expr_path (self . cx . path (self . span , init_std_path)) , ThinVec :: new () ,) ; let capture = Capture { decl : self . cx . stmt_let (self . span , true , ident , init) , ident } ; self . capture_decls . push (capture) ; self . manage_try_capture (ident , curr_capture_idx , expr) ; } # [doc = " Tries to copy `__local_bindN` into `__captureN`."] # [doc = ""] # [doc = " *{"] # [doc = "    (&Wrapper(__local_bindN)).try_capture(&mut __captureN);"] # [doc = "    __local_bindN"] # [doc = " }"] fn manage_try_capture (& mut self , capture : Ident , curr_capture_idx : usize , expr : & mut Box < Expr > ,) { let local_bind_string = format ! ("__local_bind{curr_capture_idx}") ; let local_bind = Ident :: new (Symbol :: intern (& local_bind_string) , self . span) ; self . local_bind_decls . push (self . cx . stmt_let (self . span , false , local_bind , self . cx . expr_addr_of (self . span , expr . clone ()) ,)) ; let wrapper = self . cx . expr_call (self . span , self . cx . expr_path (self . cx . path (self . span , self . cx . std_path (& [sym :: asserting , sym :: Wrapper])) ,) , thin_vec ! [self . cx . expr_path (Path :: from_ident (local_bind))] ,) ; let try_capture_call = self . cx . stmt_expr (expr_method_call (self . cx , PathSegment { args : None , id : DUMMY_NODE_ID , ident : Ident :: new (sym :: try_capture , self . span) , } , expr_paren (self . cx , self . span , self . cx . expr_addr_of (self . span , wrapper)) , thin_vec ! [expr_addr_of_mut (self . cx , self . span , self . cx . expr_path (Path :: from_ident (capture)) ,)] , self . span ,)) . add_trailing_semicolon () ; let local_bind_path = self . cx . expr_path (Path :: from_ident (local_bind)) ; let rslt = if self . is_consumed { let ret = self . cx . stmt_expr (local_bind_path) ; self . cx . expr_block (self . cx . block (self . span , thin_vec ! [try_capture_call , ret])) } else { self . best_case_captures . push (try_capture_call) ; local_bind_path } ; * expr = self . cx . expr_deref (self . span , rslt) ; } fn with_is_consumed_management (& mut self , curr_is_consumed : bool , f : impl FnOnce (& mut Self)) { let prev_is_consumed = self . is_consumed ; self . is_consumed = curr_is_consumed ; f (self) ; self . is_consumed = prev_is_consumed ; } }}}
mkitem!{mkstruct!{# [doc = " Information about a captured element."] # [derive (Debug)] struct Capture { decl : Stmt , ident : Ident , }}}

macro_rules! escape_to_fmt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function escape_to_fmt in module {}", module_path!());
    };
}

mkfn!{
    escape_to_fmt_introspect!();
    # [doc = " Escapes to use as a formatting string."] fn escape_to_fmt (s : & str) -> String { let mut rslt = String :: with_capacity (s . len ()) ; for c in s . chars () { rslt . extend (c . escape_debug ()) ; match c { '{' | '}' => rslt . push (c) , _ => { } } } rslt }
}

macro_rules! expr_addr_of_mut_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function expr_addr_of_mut in module {}", module_path!());
    };
}

mkfn!{
    expr_addr_of_mut_introspect!();
    fn expr_addr_of_mut (cx : & ExtCtxt < '_ > , sp : Span , e : Box < Expr >) -> Box < Expr > { cx . expr (sp , ExprKind :: AddrOf (BorrowKind :: Ref , Mutability :: Mut , e)) }
}

macro_rules! expr_method_call_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function expr_method_call in module {}", module_path!());
    };
}

mkfn!{
    expr_method_call_introspect!();
    fn expr_method_call (cx : & ExtCtxt < '_ > , seg : PathSegment , receiver : Box < Expr > , args : ThinVec < Box < Expr > > , span : Span ,) -> Box < Expr > { cx . expr (span , ExprKind :: MethodCall (Box :: new (MethodCall { seg , receiver , args , span }))) }
}

macro_rules! expr_paren_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function expr_paren in module {}", module_path!());
    };
}

mkfn!{
    expr_paren_introspect!();
    fn expr_paren (cx : & ExtCtxt < '_ > , sp : Span , e : Box < Expr >) -> Box < Expr > { cx . expr (sp , ExprKind :: Paren (e)) }
}