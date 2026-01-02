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
mkuse!{use std :: fmt :: Write ;}
mkuse!{use ast :: { ForLoopKind , MatchKind } ;}
mkuse!{use itertools :: { Itertools , Position } ;}
mkuse!{use rustc_ast :: util :: classify ;}
mkuse!{use rustc_ast :: util :: literal :: escape_byte_str_symbol ;}
mkuse!{use rustc_ast :: util :: parser :: { self , ExprPrecedence , Fixity } ;}
mkuse!{use rustc_ast :: { self as ast , BinOpKind , BlockCheckMode , FormatAlignment , FormatArgPosition , FormatArgsPiece , FormatCount , FormatDebugHex , FormatSign , FormatTrait , YieldKind , token , } ;}
mkuse!{use crate :: pp :: Breaks :: Inconsistent ;}
mkuse!{use crate :: pprust :: state :: fixup :: FixupContext ;}
mkuse!{use crate :: pprust :: state :: { AnnNode , INDENT_UNIT , PrintState , State } ;}
mkitem!{mkimpl!{impl < 'a > State < 'a > { fn print_else (& mut self , els : Option < & ast :: Expr >) { if let Some (_else) = els { match & _else . kind { ast :: ExprKind :: If (i , then , e) => { let cb = self . cbox (0) ; let ib = self . ibox (0) ; self . word (" else if ") ; self . print_expr_as_cond (i) ; self . space () ; self . print_block (then , cb , ib) ; self . print_else (e . as_deref ()) } ast :: ExprKind :: Block (b , None) => { let cb = self . cbox (0) ; let ib = self . ibox (0) ; self . word (" else ") ; self . print_block (b , cb , ib) } _ => { panic ! ("print_if saw if with weird alternative") ; } } } } fn print_if (& mut self , test : & ast :: Expr , blk : & ast :: Block , elseopt : Option < & ast :: Expr >) { let cb = self . cbox (0) ; let ib = self . ibox (0) ; self . word_nbsp ("if") ; self . print_expr_as_cond (test) ; self . space () ; self . print_block (blk , cb , ib) ; self . print_else (elseopt) } fn print_call_post (& mut self , args : & [Box < ast :: Expr >]) { self . popen () ; self . commasep_exprs (Inconsistent , args) ; self . pclose () } #[doc = " Prints an expr using syntax that's acceptable in a condition position, such as the `cond` in"] #[doc = " `if cond { ... }`."] fn print_expr_as_cond (& mut self , expr : & ast :: Expr) { self . print_expr_cond_paren (expr , Self :: cond_needs_par (expr) , FixupContext :: new_cond ()) } #[doc = " Does `expr` need parentheses when printed in a condition position?"] #[doc = ""] #[doc = " These cases need parens due to the parse error observed in #26461: `if return {}`"] #[doc = " parses as the erroneous construct `if (return {})`, not `if (return) {}`."] fn cond_needs_par (expr : & ast :: Expr) -> bool { match expr . kind { ast :: ExprKind :: Break (..) | ast :: ExprKind :: Closure (..) | ast :: ExprKind :: Ret (..) | ast :: ExprKind :: Yeet (..) => true , _ => parser :: contains_exterior_struct_lit (expr) , } } #[doc = " Prints `expr` or `(expr)` when `needs_par` holds."] pub (super) fn print_expr_cond_paren (& mut self , expr : & ast :: Expr , needs_par : bool , mut fixup : FixupContext ,) { if needs_par { self . popen () ; fixup = FixupContext :: default () ; } self . print_expr (expr , fixup) ; if needs_par { self . pclose () ; } } fn print_expr_vec (& mut self , exprs : & [Box < ast :: Expr >]) { let ib = self . ibox (INDENT_UNIT) ; self . word ("[") ; self . commasep_exprs (Inconsistent , exprs) ; self . word ("]") ; self . end (ib) ; } pub (super) fn print_expr_anon_const (& mut self , expr : & ast :: AnonConst , attrs : & [ast :: Attribute] ,) { let ib = self . ibox (INDENT_UNIT) ; self . word ("const") ; self . nbsp () ; if let ast :: ExprKind :: Block (block , None) = & expr . value . kind { let cb = self . cbox (0) ; let ib = self . ibox (0) ; self . print_block_with_attrs (block , attrs , cb , ib) ; } else { self . print_expr (& expr . value , FixupContext :: default ()) ; } self . end (ib) ; } fn print_expr_repeat (& mut self , element : & ast :: Expr , count : & ast :: AnonConst) { let ib = self . ibox (INDENT_UNIT) ; self . word ("[") ; self . print_expr (element , FixupContext :: default ()) ; self . word_space (";") ; self . print_expr (& count . value , FixupContext :: default ()) ; self . word ("]") ; self . end (ib) ; } fn print_expr_struct (& mut self , qself : & Option < Box < ast :: QSelf > > , path : & ast :: Path , fields : & [ast :: ExprField] , rest : & ast :: StructRest ,) { if let Some (qself) = qself { self . print_qpath (path , qself , true) ; } else { self . print_path (path , true , 0) ; } self . nbsp () ; self . word ("{") ; let has_rest = match rest { ast :: StructRest :: Base (_) | ast :: StructRest :: Rest (_) => true , ast :: StructRest :: None => false , } ; if fields . is_empty () && ! has_rest { self . word ("}") ; return ; } let cb = self . cbox (0) ; for (pos , field) in fields . iter () . with_position () { let is_first = matches ! (pos , Position :: First | Position :: Only) ; let is_last = matches ! (pos , Position :: Last | Position :: Only) ; self . maybe_print_comment (field . span . hi ()) ; self . print_outer_attributes (& field . attrs) ; if is_first { self . space_if_not_bol () ; } if ! field . is_shorthand { self . print_ident (field . ident) ; self . word_nbsp (":") ; } self . print_expr (& field . expr , FixupContext :: default ()) ; if ! is_last || has_rest { self . word_space (",") ; } else { self . trailing_comma_or_space () ; } } if has_rest { if fields . is_empty () { self . space () ; } self . word ("..") ; if let ast :: StructRest :: Base (expr) = rest { self . print_expr (expr , FixupContext :: default ()) ; } self . space () ; } self . offset (- INDENT_UNIT) ; self . end (cb) ; self . word ("}") ; } fn print_expr_tup (& mut self , exprs : & [Box < ast :: Expr >]) { self . popen () ; self . commasep_exprs (Inconsistent , exprs) ; if exprs . len () == 1 { self . word (",") ; } self . pclose () } fn print_expr_call (& mut self , func : & ast :: Expr , args : & [Box < ast :: Expr >] , fixup : FixupContext) { let func_fixup = fixup . leftmost_subexpression_with_operator (true) ; let needs_paren = match func . kind { ast :: ExprKind :: Field (_ , name) => ! name . is_numeric () , _ => func_fixup . precedence (func) < ExprPrecedence :: Unambiguous , } ; self . print_expr_cond_paren (func , needs_paren , func_fixup) ; self . print_call_post (args) } fn print_expr_method_call (& mut self , segment : & ast :: PathSegment , receiver : & ast :: Expr , base_args : & [Box < ast :: Expr >] , fixup : FixupContext ,) { self . print_expr_cond_paren (receiver , receiver . precedence () < ExprPrecedence :: Unambiguous , fixup . leftmost_subexpression_with_dot () ,) ; self . word (".") ; self . print_ident (segment . ident) ; if let Some (args) = & segment . args { self . print_generic_args (args , true) ; } self . print_call_post (base_args) } fn print_expr_binary (& mut self , op : ast :: BinOpKind , lhs : & ast :: Expr , rhs : & ast :: Expr , fixup : FixupContext ,) { let operator_can_begin_expr = match op { | BinOpKind :: Sub | BinOpKind :: Mul | BinOpKind :: And | BinOpKind :: Or | BinOpKind :: BitAnd | BinOpKind :: BitOr | BinOpKind :: Shl | BinOpKind :: Lt => true , _ => false , } ; let left_fixup = fixup . leftmost_subexpression_with_operator (operator_can_begin_expr) ; let binop_prec = op . precedence () ; let left_prec = left_fixup . precedence (lhs) ; let right_prec = fixup . precedence (rhs) ; let (mut left_needs_paren , right_needs_paren) = match op . fixity () { Fixity :: Left => (left_prec < binop_prec , right_prec <= binop_prec) , Fixity :: Right => (left_prec <= binop_prec , right_prec < binop_prec) , Fixity :: None => (left_prec <= binop_prec , right_prec <= binop_prec) , } ; match (& lhs . kind , op) { (& ast :: ExprKind :: Cast { .. } , ast :: BinOpKind :: Lt | ast :: BinOpKind :: Shl) => { left_needs_paren = true ; } (& ast :: ExprKind :: Let { .. } , _) if ! parser :: needs_par_as_let_scrutinee (binop_prec) => { left_needs_paren = true ; } _ => { } } self . print_expr_cond_paren (lhs , left_needs_paren , left_fixup) ; self . space () ; self . word_space (op . as_str ()) ; self . print_expr_cond_paren (rhs , right_needs_paren , fixup . rightmost_subexpression ()) ; } fn print_expr_unary (& mut self , op : ast :: UnOp , expr : & ast :: Expr , fixup : FixupContext) { self . word (op . as_str ()) ; self . print_expr_cond_paren (expr , fixup . precedence (expr) < ExprPrecedence :: Prefix , fixup . rightmost_subexpression () ,) ; } fn print_expr_addr_of (& mut self , kind : ast :: BorrowKind , mutability : ast :: Mutability , expr : & ast :: Expr , fixup : FixupContext ,) { self . word ("&") ; match kind { ast :: BorrowKind :: Ref => self . print_mutability (mutability , false) , ast :: BorrowKind :: Raw => { self . word_nbsp ("raw") ; self . print_mutability (mutability , true) ; } ast :: BorrowKind :: Pin => { self . word_nbsp ("pin") ; self . print_mutability (mutability , true) ; } } self . print_expr_cond_paren (expr , fixup . precedence (expr) < ExprPrecedence :: Prefix , fixup . rightmost_subexpression () ,) ; } pub (super) fn print_expr (& mut self , expr : & ast :: Expr , fixup : FixupContext) { self . print_expr_outer_attr_style (expr , true , fixup) } pub (super) fn print_expr_outer_attr_style (& mut self , expr : & ast :: Expr , is_inline : bool , mut fixup : FixupContext ,) { self . maybe_print_comment (expr . span . lo ()) ; let attrs = & expr . attrs ; if is_inline { self . print_outer_attributes_inline (attrs) ; } else { self . print_outer_attributes (attrs) ; } let ib = self . ibox (INDENT_UNIT) ; let needs_par = { fixup . would_cause_statement_boundary (expr) } || { ! attrs . is_empty () && matches ! (expr . kind , ast :: ExprKind :: Binary (..) | ast :: ExprKind :: Cast (..) | ast :: ExprKind :: Assign (..) | ast :: ExprKind :: AssignOp (..) | ast :: ExprKind :: Range (..)) } ; if needs_par { self . popen () ; fixup = FixupContext :: default () ; } self . ann . pre (self , AnnNode :: Expr (expr)) ; match & expr . kind { ast :: ExprKind :: Array (exprs) => { self . print_expr_vec (exprs) ; } ast :: ExprKind :: ConstBlock (anon_const) => { self . print_expr_anon_const (anon_const , attrs) ; } ast :: ExprKind :: Repeat (element , count) => { self . print_expr_repeat (element , count) ; } ast :: ExprKind :: Struct (se) => { self . print_expr_struct (& se . qself , & se . path , & se . fields , & se . rest) ; } ast :: ExprKind :: Tup (exprs) => { self . print_expr_tup (exprs) ; } ast :: ExprKind :: Call (func , args) => { self . print_expr_call (func , args , fixup) ; } ast :: ExprKind :: MethodCall (box ast :: MethodCall { seg , receiver , args , .. }) => { self . print_expr_method_call (seg , receiver , args , fixup) ; } ast :: ExprKind :: Binary (op , lhs , rhs) => { self . print_expr_binary (op . node , lhs , rhs , fixup) ; } ast :: ExprKind :: Unary (op , expr) => { self . print_expr_unary (* op , expr , fixup) ; } ast :: ExprKind :: AddrOf (k , m , expr) => { self . print_expr_addr_of (* k , * m , expr , fixup) ; } ast :: ExprKind :: Lit (token_lit) => { self . print_token_literal (* token_lit , expr . span) ; } ast :: ExprKind :: IncludedBytes (byte_sym) => { let lit = token :: Lit :: new (token :: ByteStr , escape_byte_str_symbol (byte_sym . as_byte_str ()) , None ,) ; self . print_token_literal (lit , expr . span) } ast :: ExprKind :: Cast (expr , ty) => { self . print_expr_cond_paren (expr , expr . precedence () < ExprPrecedence :: Cast , fixup . leftmost_subexpression () ,) ; self . space () ; self . word_space ("as") ; self . print_type (ty) ; } ast :: ExprKind :: Type (expr , ty) => { self . word ("builtin # type_ascribe") ; self . popen () ; let ib = self . ibox (0) ; self . print_expr (expr , FixupContext :: default ()) ; self . word (",") ; self . space_if_not_bol () ; self . print_type (ty) ; self . end (ib) ; self . pclose () ; } ast :: ExprKind :: Let (pat , scrutinee , _ , _) => { self . print_let (pat , scrutinee , fixup) ; } ast :: ExprKind :: If (test , blk , elseopt) => self . print_if (test , blk , elseopt . as_deref ()) , ast :: ExprKind :: While (test , blk , opt_label) => { if let Some (label) = opt_label { self . print_ident (label . ident) ; self . word_space (":") ; } let cb = self . cbox (0) ; let ib = self . ibox (0) ; self . word_nbsp ("while") ; self . print_expr_as_cond (test) ; self . space () ; self . print_block_with_attrs (blk , attrs , cb , ib) ; } ast :: ExprKind :: ForLoop { pat , iter , body , label , kind } => { if let Some (label) = label { self . print_ident (label . ident) ; self . word_space (":") ; } let cb = self . cbox (0) ; let ib = self . ibox (0) ; self . word_nbsp ("for") ; if kind == & ForLoopKind :: ForAwait { self . word_nbsp ("await") ; } self . print_pat (pat) ; self . space () ; self . word_space ("in") ; self . print_expr_as_cond (iter) ; self . space () ; self . print_block_with_attrs (body , attrs , cb , ib) ; } ast :: ExprKind :: Loop (blk , opt_label , _) => { let cb = self . cbox (0) ; let ib = self . ibox (0) ; if let Some (label) = opt_label { self . print_ident (label . ident) ; self . word_space (":") ; } self . word_nbsp ("loop") ; self . print_block_with_attrs (blk , attrs , cb , ib) ; } ast :: ExprKind :: Match (expr , arms , match_kind) => { let cb = self . cbox (0) ; let ib = self . ibox (0) ; match match_kind { MatchKind :: Prefix => { self . word_nbsp ("match") ; self . print_expr_as_cond (expr) ; self . space () ; } MatchKind :: Postfix => { self . print_expr_cond_paren (expr , expr . precedence () < ExprPrecedence :: Unambiguous , fixup . leftmost_subexpression_with_dot () ,) ; self . word_nbsp (".match") ; } } self . bopen (ib) ; self . print_inner_attributes_no_trailing_hardbreak (attrs) ; for arm in arms { self . print_arm (arm) ; } let empty = attrs . is_empty () && arms . is_empty () ; self . bclose (expr . span , empty , cb) ; } ast :: ExprKind :: Closure (box ast :: Closure { binder , capture_clause , constness , coroutine_kind , movability , fn_decl , body , fn_decl_span : _ , fn_arg_span : _ , }) => { self . print_closure_binder (binder) ; self . print_constness (* constness) ; self . print_movability (* movability) ; coroutine_kind . map (| coroutine_kind | self . print_coroutine_kind (coroutine_kind)) ; self . print_capture_clause (* capture_clause) ; self . print_fn_params_and_ret (fn_decl , true) ; self . space () ; self . print_expr (body , FixupContext :: default ()) ; } ast :: ExprKind :: Block (blk , opt_label) => { if let Some (label) = opt_label { self . print_ident (label . ident) ; self . word_space (":") ; } let cb = self . cbox (0) ; let ib = self . ibox (0) ; self . print_block_with_attrs (blk , attrs , cb , ib) ; } ast :: ExprKind :: Gen (capture_clause , blk , kind , _decl_span) => { self . word_nbsp (kind . modifier ()) ; self . print_capture_clause (* capture_clause) ; let cb = self . cbox (0) ; let ib = self . ibox (0) ; self . print_block_with_attrs (blk , attrs , cb , ib) ; } ast :: ExprKind :: Await (expr , _) => { self . print_expr_cond_paren (expr , expr . precedence () < ExprPrecedence :: Unambiguous , fixup . leftmost_subexpression_with_dot () ,) ; self . word (".await") ; } ast :: ExprKind :: Use (expr , _) => { self . print_expr_cond_paren (expr , expr . precedence () < ExprPrecedence :: Unambiguous , fixup ,) ; self . word (".use") ; } ast :: ExprKind :: Assign (lhs , rhs , _) => { self . print_expr_cond_paren (lhs , lhs . precedence () <= ExprPrecedence :: Range , fixup . leftmost_subexpression () ,) ; self . space () ; self . word_space ("=") ; self . print_expr_cond_paren (rhs , fixup . precedence (rhs) < ExprPrecedence :: Assign , fixup . rightmost_subexpression () ,) ; } ast :: ExprKind :: AssignOp (op , lhs , rhs) => { self . print_expr_cond_paren (lhs , lhs . precedence () <= ExprPrecedence :: Range , fixup . leftmost_subexpression () ,) ; self . space () ; self . word_space (op . node . as_str ()) ; self . print_expr_cond_paren (rhs , fixup . precedence (rhs) < ExprPrecedence :: Assign , fixup . rightmost_subexpression () ,) ; } ast :: ExprKind :: Field (expr , ident) => { self . print_expr_cond_paren (expr , expr . precedence () < ExprPrecedence :: Unambiguous , fixup . leftmost_subexpression_with_dot () ,) ; self . word (".") ; self . print_ident (* ident) ; } ast :: ExprKind :: Index (expr , index , _) => { let expr_fixup = fixup . leftmost_subexpression_with_operator (true) ; self . print_expr_cond_paren (expr , expr_fixup . precedence (expr) < ExprPrecedence :: Unambiguous , expr_fixup ,) ; self . word ("[") ; self . print_expr (index , FixupContext :: default ()) ; self . word ("]") ; } ast :: ExprKind :: Range (start , end , limits) => { let fake_prec = ExprPrecedence :: LOr ; if let Some (e) = start { let start_fixup = fixup . leftmost_subexpression_with_operator (true) ; self . print_expr_cond_paren (e , start_fixup . precedence (e) < fake_prec , start_fixup ,) ; } match limits { ast :: RangeLimits :: HalfOpen => self . word ("..") , ast :: RangeLimits :: Closed => self . word ("..=") , } if let Some (e) = end { self . print_expr_cond_paren (e , fixup . precedence (e) < fake_prec , fixup . rightmost_subexpression () ,) ; } } ast :: ExprKind :: Underscore => self . word ("_") , ast :: ExprKind :: Path (None , path) => self . print_path (path , true , 0) , ast :: ExprKind :: Path (Some (qself) , path) => self . print_qpath (path , qself , true) , ast :: ExprKind :: Break (opt_label , opt_expr) => { self . word ("break") ; if let Some (label) = opt_label { self . space () ; self . print_ident (label . ident) ; } if let Some (expr) = opt_expr { self . space () ; self . print_expr_cond_paren (expr , opt_label . is_none () && classify :: leading_labeled_expr (expr) , fixup . rightmost_subexpression () ,) ; } } ast :: ExprKind :: Continue (opt_label) => { self . word ("continue") ; if let Some (label) = opt_label { self . space () ; self . print_ident (label . ident) ; } } ast :: ExprKind :: Ret (result) => { self . word ("return") ; if let Some (expr) = result { self . word (" ") ; self . print_expr (expr , fixup . rightmost_subexpression ()) ; } } ast :: ExprKind :: Yeet (result) => { self . word ("do") ; self . word (" ") ; self . word ("yeet") ; if let Some (expr) = result { self . word (" ") ; self . print_expr (expr , fixup . rightmost_subexpression ()) ; } } ast :: ExprKind :: Become (result) => { self . word ("become") ; self . word (" ") ; self . print_expr (result , fixup . rightmost_subexpression ()) ; } ast :: ExprKind :: InlineAsm (a) => { self . word ("asm!") ; self . print_inline_asm (a) ; } ast :: ExprKind :: FormatArgs (fmt) => { self . word ("format_args!") ; self . popen () ; let ib = self . ibox (0) ; self . word (reconstruct_format_args_template_string (& fmt . template)) ; for arg in fmt . arguments . all_args () { self . word_space (",") ; self . print_expr (& arg . expr , FixupContext :: default ()) ; } self . end (ib) ; self . pclose () ; } ast :: ExprKind :: OffsetOf (container , fields) => { self . word ("builtin # offset_of") ; self . popen () ; let ib = self . ibox (0) ; self . print_type (container) ; self . word (",") ; self . space () ; if let Some ((& first , rest)) = fields . split_first () { self . print_ident (first) ; for & field in rest { self . word (".") ; self . print_ident (field) ; } } self . end (ib) ; self . pclose () ; } ast :: ExprKind :: MacCall (m) => self . print_mac (m) , ast :: ExprKind :: Paren (e) => { self . popen () ; self . print_expr (e , FixupContext :: default ()) ; self . pclose () ; } ast :: ExprKind :: Yield (YieldKind :: Prefix (e)) => { self . word ("yield") ; if let Some (expr) = e { self . space () ; self . print_expr (expr , fixup . rightmost_subexpression ()) ; } } ast :: ExprKind :: Yield (YieldKind :: Postfix (e)) => { self . print_expr_cond_paren (e , e . precedence () < ExprPrecedence :: Unambiguous , fixup . leftmost_subexpression_with_dot () ,) ; self . word (".yield") ; } ast :: ExprKind :: Try (e) => { self . print_expr_cond_paren (e , e . precedence () < ExprPrecedence :: Unambiguous , fixup . leftmost_subexpression_with_dot () ,) ; self . word ("?") } ast :: ExprKind :: TryBlock (blk) => { let cb = self . cbox (0) ; let ib = self . ibox (0) ; self . word_nbsp ("try") ; self . print_block_with_attrs (blk , attrs , cb , ib) } ast :: ExprKind :: UnsafeBinderCast (kind , expr , ty) => { self . word ("builtin # ") ; match kind { ast :: UnsafeBinderCastKind :: Wrap => self . word ("wrap_binder") , ast :: UnsafeBinderCastKind :: Unwrap => self . word ("unwrap_binder") , } self . popen () ; let ib = self . ibox (0) ; self . print_expr (expr , FixupContext :: default ()) ; if let Some (ty) = ty { self . word (",") ; self . space () ; self . print_type (ty) ; } self . end (ib) ; self . pclose () ; } ast :: ExprKind :: Err (_) => { self . popen () ; self . word ("/*ERROR*/") ; self . pclose () } ast :: ExprKind :: Dummy => { self . popen () ; self . word ("/*DUMMY*/") ; self . pclose () ; } } self . ann . post (self , AnnNode :: Expr (expr)) ; if needs_par { self . pclose () ; } self . end (ib) ; } fn print_arm (& mut self , arm : & ast :: Arm) { if arm . attrs . is_empty () { self . space () ; } let cb = self . cbox (INDENT_UNIT) ; let ib = self . ibox (0) ; self . maybe_print_comment (arm . pat . span . lo ()) ; self . print_outer_attributes (& arm . attrs) ; self . print_pat (& arm . pat) ; self . space () ; if let Some (e) = & arm . guard { self . word_space ("if") ; self . print_expr (e , FixupContext :: default ()) ; self . space () ; } if let Some (body) = & arm . body { self . word_space ("=>") ; match & body . kind { ast :: ExprKind :: Block (blk , opt_label) => { if let Some (label) = opt_label { self . print_ident (label . ident) ; self . word_space (":") ; } self . print_block_unclosed_indent (blk , ib) ; if let BlockCheckMode :: Unsafe (ast :: UserProvided) = blk . rules { self . word (",") ; } } _ => { self . end (ib) ; self . print_expr (body , FixupContext :: new_match_arm ()) ; self . word (",") ; } } } else { self . end (ib) ; self . word (",") ; } self . end (cb) ; } fn print_closure_binder (& mut self , binder : & ast :: ClosureBinder) { match binder { ast :: ClosureBinder :: NotPresent => { } ast :: ClosureBinder :: For { generic_params , .. } => { self . print_formal_generic_params (generic_params) } } } fn print_movability (& mut self , movability : ast :: Movability) { match movability { ast :: Movability :: Static => self . word_space ("static") , ast :: Movability :: Movable => { } } } fn print_capture_clause (& mut self , capture_clause : ast :: CaptureBy) { match capture_clause { ast :: CaptureBy :: Value { .. } => self . word_space ("move") , ast :: CaptureBy :: Use { .. } => self . word_space ("use") , ast :: CaptureBy :: Ref => { } } } }}}

macro_rules! reconstruct_format_args_template_string_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function reconstruct_format_args_template_string in module {}", module_path!());
    };
}

mkfn!{
    reconstruct_format_args_template_string_introspect!();
    fn reconstruct_format_args_template_string (pieces : & [FormatArgsPiece]) -> String { let mut template = "\"" . to_string () ; for piece in pieces { match piece { FormatArgsPiece :: Literal (s) => { for c in s . as_str () . chars () { template . extend (c . escape_debug ()) ; if let '{' | '}' = c { template . push (c) ; } } } FormatArgsPiece :: Placeholder (p) => { template . push ('{') ; let (Ok (n) | Err (n)) = p . argument . index ; write ! (template , "{n}") . unwrap () ; if p . format_options != Default :: default () || p . format_trait != FormatTrait :: Display { template . push (':') ; } if let Some (fill) = p . format_options . fill { template . push (fill) ; } match p . format_options . alignment { Some (FormatAlignment :: Left) => template . push ('<') , Some (FormatAlignment :: Right) => template . push ('>') , Some (FormatAlignment :: Center) => template . push ('^') , None => { } } match p . format_options . sign { Some (FormatSign :: Plus) => template . push ('+') , Some (FormatSign :: Minus) => template . push ('-') , None => { } } if p . format_options . alternate { template . push ('#') ; } if p . format_options . zero_pad { template . push ('0') ; } if let Some (width) = & p . format_options . width { match width { FormatCount :: Literal (n) => write ! (template , "{n}") . unwrap () , FormatCount :: Argument (FormatArgPosition { index : Ok (n) | Err (n) , .. }) => { write ! (template , "{n}$") . unwrap () ; } } } if let Some (precision) = & p . format_options . precision { template . push ('.') ; match precision { FormatCount :: Literal (n) => write ! (template , "{n}") . unwrap () , FormatCount :: Argument (FormatArgPosition { index : Ok (n) | Err (n) , .. }) => { write ! (template , "{n}$") . unwrap () ; } } } match p . format_options . debug_hex { Some (FormatDebugHex :: Lower) => template . push ('x') , Some (FormatDebugHex :: Upper) => template . push ('X') , None => { } } template . push_str (match p . format_trait { FormatTrait :: Display => "" , FormatTrait :: Debug => "?" , FormatTrait :: LowerExp => "e" , FormatTrait :: UpperExp => "E" , FormatTrait :: Octal => "o" , FormatTrait :: Pointer => "p" , FormatTrait :: Binary => "b" , FormatTrait :: LowerHex => "x" , FormatTrait :: UpperHex => "X" , }) ; template . push ('}') ; } } } template . push ('"') ; template }
}