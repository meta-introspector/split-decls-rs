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
mkuse!{use rustc_ast :: { self as ast , AsmMacro } ;}
mkuse!{use rustc_span :: { Span , Symbol , kw } ;}
mkuse!{use super :: { ExpKeywordPair , ForceCollect , IdentIsRaw , Trailing , UsePreAttrPos } ;}
mkuse!{use crate :: { PResult , Parser , errors , exp , token } ;}
mkitem!{mkstruct!{# [doc = " An argument to one of the `asm!` macros. The argument is syntactically valid, but is otherwise"] # [doc = " not validated at all."] pub struct AsmArg { pub kind : AsmArgKind , pub attributes : AsmAttrVec , pub span : Span , }}}
mkitem!{mkenum!{pub enum AsmArgKind { Template (Box < ast :: Expr >) , Operand (Option < Symbol > , ast :: InlineAsmOperand) , Options (Vec < AsmOption >) , ClobberAbi (Vec < (Symbol , Span) >) , }}}
mkitem!{mkstruct!{pub struct AsmOption { pub symbol : Symbol , pub span : Span , pub options : ast :: InlineAsmOptions , pub span_with_comma : Span , }}}
mkitem!{mkstruct!{# [doc = " A parsed list of attributes that is not attached to any item."] # [doc = " Used to check whether `asm!` arguments are configured out."] pub struct AsmAttrVec (pub ast :: AttrVec) ;}}
mkitem!{mkimpl!{impl AsmAttrVec { fn parse < 'a > (p : & mut Parser < 'a >) -> PResult < 'a , Self > { let attrs = p . parse_outer_attributes () ? ; p . collect_tokens (None , attrs , ForceCollect :: No , | _ , attrs | { Ok ((Self (attrs) , Trailing :: No , UsePreAttrPos :: No)) }) } }}}
mkitem!{mkimpl!{impl ast :: HasAttrs for AsmAttrVec { const SUPPORTS_CUSTOM_INNER_ATTRS : bool = false ; fn attrs (& self) -> & [rustc_ast :: Attribute] { & self . 0 } fn visit_attrs (& mut self , f : impl FnOnce (& mut rustc_ast :: AttrVec)) { f (& mut self . 0) } }}}
mkitem!{mkimpl!{impl ast :: HasTokens for AsmAttrVec { fn tokens (& self) -> Option < & rustc_ast :: tokenstream :: LazyAttrTokenStream > { None } fn tokens_mut (& mut self) -> Option < & mut Option < rustc_ast :: tokenstream :: LazyAttrTokenStream > > { None } }}}

macro_rules! eat_operand_keyword_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function eat_operand_keyword in module {}", module_path!());
    };
}

mkfn!{
    eat_operand_keyword_introspect!();
    # [doc = " Used for better error messages when operand types are used that are not"] # [doc = " supported by the current macro (e.g. `in` or `out` for `global_asm!`)"] # [doc = ""] # [doc = " returns"] # [doc = ""] # [doc = " - `Ok(true)` if the current token matches the keyword, and was expected"] # [doc = " - `Ok(false)` if the current token does not match the keyword"] # [doc = " - `Err(_)` if the current token matches the keyword, but was not expected"] fn eat_operand_keyword < 'a > (p : & mut Parser < 'a > , exp : ExpKeywordPair , asm_macro : AsmMacro ,) -> PResult < 'a , bool > { if matches ! (asm_macro , AsmMacro :: Asm) { Ok (p . eat_keyword (exp)) } else { let span = p . token . span ; if p . eat_keyword_noexpect (exp . kw) { let symbol = if exp . kw == kw :: In { "in" } else { exp . kw . as_str () } ; Err (p . dcx () . create_err (errors :: AsmUnsupportedOperand { span , symbol , macro_name : asm_macro . macro_name () , })) } else { Ok (false) } } }
}

macro_rules! parse_asm_operand_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_asm_operand in module {}", module_path!());
    };
}

mkfn!{
    parse_asm_operand_introspect!();
    fn parse_asm_operand < 'a > (p : & mut Parser < 'a > , asm_macro : AsmMacro ,) -> PResult < 'a , Option < ast :: InlineAsmOperand > > { let dcx = p . dcx () ; Ok (Some (if eat_operand_keyword (p , exp ! (In) , asm_macro) ? { let reg = parse_reg (p) ? ; if p . eat_keyword (exp ! (Underscore)) { let err = dcx . create_err (errors :: AsmUnderscoreInput { span : p . token . span }) ; return Err (err) ; } let expr = p . parse_expr () ? ; ast :: InlineAsmOperand :: In { reg , expr } } else if eat_operand_keyword (p , exp ! (Out) , asm_macro) ? { let reg = parse_reg (p) ? ; let expr = if p . eat_keyword (exp ! (Underscore)) { None } else { Some (p . parse_expr () ?) } ; ast :: InlineAsmOperand :: Out { reg , expr , late : false } } else if eat_operand_keyword (p , exp ! (Lateout) , asm_macro) ? { let reg = parse_reg (p) ? ; let expr = if p . eat_keyword (exp ! (Underscore)) { None } else { Some (p . parse_expr () ?) } ; ast :: InlineAsmOperand :: Out { reg , expr , late : true } } else if eat_operand_keyword (p , exp ! (Inout) , asm_macro) ? { let reg = parse_reg (p) ? ; if p . eat_keyword (exp ! (Underscore)) { let err = dcx . create_err (errors :: AsmUnderscoreInput { span : p . token . span }) ; return Err (err) ; } let expr = p . parse_expr () ? ; if p . eat (exp ! (FatArrow)) { let out_expr = if p . eat_keyword (exp ! (Underscore)) { None } else { Some (p . parse_expr () ?) } ; ast :: InlineAsmOperand :: SplitInOut { reg , in_expr : expr , out_expr , late : false } } else { ast :: InlineAsmOperand :: InOut { reg , expr , late : false } } } else if eat_operand_keyword (p , exp ! (Inlateout) , asm_macro) ? { let reg = parse_reg (p) ? ; if p . eat_keyword (exp ! (Underscore)) { let err = dcx . create_err (errors :: AsmUnderscoreInput { span : p . token . span }) ; return Err (err) ; } let expr = p . parse_expr () ? ; if p . eat (exp ! (FatArrow)) { let out_expr = if p . eat_keyword (exp ! (Underscore)) { None } else { Some (p . parse_expr () ?) } ; ast :: InlineAsmOperand :: SplitInOut { reg , in_expr : expr , out_expr , late : true } } else { ast :: InlineAsmOperand :: InOut { reg , expr , late : true } } } else if eat_operand_keyword (p , exp ! (Label) , asm_macro) ? { let block = p . parse_block () ? ; ast :: InlineAsmOperand :: Label { block } } else if p . eat_keyword (exp ! (Const)) { let anon_const = p . parse_expr_anon_const () ? ; ast :: InlineAsmOperand :: Const { anon_const } } else if p . eat_keyword (exp ! (Sym)) { let expr = p . parse_expr () ? ; let ast :: ExprKind :: Path (qself , path) = & expr . kind else { let err = dcx . create_err (errors :: AsmSymNoPath { span : expr . span }) ; return Err (err) ; } ; let sym = ast :: InlineAsmSym { id : ast :: DUMMY_NODE_ID , qself : qself . clone () , path : path . clone () } ; ast :: InlineAsmOperand :: Sym { sym } } else { return Ok (None) ; })) }
}

macro_rules! parse_asm_args_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_asm_args in module {}", module_path!());
    };
}

mkfn!{
    parse_asm_args_introspect!();
    pub fn parse_asm_args < 'a > (p : & mut Parser < 'a > , sp : Span , asm_macro : AsmMacro ,) -> PResult < 'a , Vec < AsmArg > > { let dcx = p . dcx () ; if p . token == token :: Eof { return Err (dcx . create_err (errors :: AsmRequiresTemplate { span : sp })) ; } let mut args = Vec :: new () ; let attributes = AsmAttrVec :: parse (p) ? ; let first_template = p . parse_expr () ? ; args . push (AsmArg { span : first_template . span , kind : AsmArgKind :: Template (first_template) , attributes , }) ; let mut allow_templates = true ; while p . token != token :: Eof { if ! p . eat (exp ! (Comma)) { if allow_templates { return Err (dcx . create_err (errors :: AsmExpectedComma { span : p . token . span })) ; } else { return Err (p . expect (exp ! (Comma)) . err () . unwrap ()) ; } } if p . token == token :: Eof { break ; } let attributes = AsmAttrVec :: parse (p) ? ; let span_start = p . token . span ; if p . eat_keyword (exp ! (ClobberAbi)) { allow_templates = false ; args . push (AsmArg { kind : AsmArgKind :: ClobberAbi (parse_clobber_abi (p) ?) , span : span_start . to (p . prev_token . span) , attributes , }) ; continue ; } if p . eat_keyword (exp ! (Options)) { allow_templates = false ; args . push (AsmArg { kind : AsmArgKind :: Options (parse_options (p , asm_macro) ?) , span : span_start . to (p . prev_token . span) , attributes , }) ; continue ; } let name = if p . token . is_ident () && p . look_ahead (1 , | t | * t == token :: Eq) { let (ident , _) = p . token . ident () . unwrap () ; p . bump () ; p . expect (exp ! (Eq)) ? ; allow_templates = false ; Some (ident . name) } else { None } ; if let Some (op) = parse_asm_operand (p , asm_macro) ? { allow_templates = false ; args . push (AsmArg { span : span_start . to (p . prev_token . span) , kind : AsmArgKind :: Operand (name , op) , attributes , }) ; } else if allow_templates { let template = p . parse_expr () ? ; match template . kind { ast :: ExprKind :: Lit (token_lit) if matches ! (token_lit . kind , token :: LitKind :: Str | token :: LitKind :: StrRaw (_)) => { } ast :: ExprKind :: MacCall (..) => { } _ => { let err = dcx . create_err (errors :: AsmExpectedOther { span : template . span , is_inline_asm : matches ! (asm_macro , AsmMacro :: Asm) , }) ; return Err (err) ; } } args . push (AsmArg { span : template . span , kind : AsmArgKind :: Template (template) , attributes , }) ; } else { p . unexpected_any () ? } } Ok (args) }
}

macro_rules! parse_options_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_options in module {}", module_path!());
    };
}

mkfn!{
    parse_options_introspect!();
    fn parse_options < 'a > (p : & mut Parser < 'a > , asm_macro : AsmMacro) -> PResult < 'a , Vec < AsmOption > > { p . expect (exp ! (OpenParen)) ? ; let mut asm_options = Vec :: new () ; while ! p . eat (exp ! (CloseParen)) { const OPTIONS : [(ExpKeywordPair , ast :: InlineAsmOptions) ; ast :: InlineAsmOptions :: COUNT] = [(exp ! (Pure) , ast :: InlineAsmOptions :: PURE) , (exp ! (Nomem) , ast :: InlineAsmOptions :: NOMEM) , (exp ! (Readonly) , ast :: InlineAsmOptions :: READONLY) , (exp ! (PreservesFlags) , ast :: InlineAsmOptions :: PRESERVES_FLAGS) , (exp ! (Noreturn) , ast :: InlineAsmOptions :: NORETURN) , (exp ! (Nostack) , ast :: InlineAsmOptions :: NOSTACK) , (exp ! (MayUnwind) , ast :: InlineAsmOptions :: MAY_UNWIND) , (exp ! (AttSyntax) , ast :: InlineAsmOptions :: ATT_SYNTAX) , (exp ! (Raw) , ast :: InlineAsmOptions :: RAW) ,] ; 'blk : { for (exp , options) in OPTIONS { let kw_matched = if asm_macro . is_supported_option (options) { p . eat_keyword (exp) } else { p . eat_keyword_noexpect (exp . kw) } ; if kw_matched { let span = p . prev_token . span ; let span_with_comma = if p . token == token :: Comma { span . to (p . token . span) } else { span } ; asm_options . push (AsmOption { symbol : exp . kw , span , options , span_with_comma }) ; break 'blk ; } } return p . unexpected_any () ; } if p . eat (exp ! (CloseParen)) { break ; } p . expect (exp ! (Comma)) ? ; } Ok (asm_options) }
}

macro_rules! parse_clobber_abi_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_clobber_abi in module {}", module_path!());
    };
}

mkfn!{
    parse_clobber_abi_introspect!();
    fn parse_clobber_abi < 'a > (p : & mut Parser < 'a >) -> PResult < 'a , Vec < (Symbol , Span) > > { p . expect (exp ! (OpenParen)) ? ; if p . eat (exp ! (CloseParen)) { return Err (p . dcx () . create_err (errors :: NonABI { span : p . token . span })) ; } let mut new_abis = Vec :: new () ; while ! p . eat (exp ! (CloseParen)) { match p . parse_str_lit () { Ok (str_lit) => { new_abis . push ((str_lit . symbol_unescaped , str_lit . span)) ; } Err (opt_lit) => { let span = opt_lit . map_or (p . token . span , | lit | lit . span) ; return Err (p . dcx () . create_err (errors :: AsmExpectedStringLiteral { span })) ; } } ; if p . eat (exp ! (CloseParen)) { break ; } p . expect (exp ! (Comma)) ? ; } Ok (new_abis) }
}

macro_rules! parse_reg_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_reg in module {}", module_path!());
    };
}

mkfn!{
    parse_reg_introspect!();
    fn parse_reg < 'a > (p : & mut Parser < 'a >) -> PResult < 'a , ast :: InlineAsmRegOrRegClass > { p . expect (exp ! (OpenParen)) ? ; let result = match p . token . uninterpolate () . kind { token :: Ident (name , IdentIsRaw :: No) => ast :: InlineAsmRegOrRegClass :: RegClass (name) , token :: Literal (token :: Lit { kind : token :: LitKind :: Str , symbol , suffix : _ }) => { ast :: InlineAsmRegOrRegClass :: Reg (symbol) } _ => { return Err (p . dcx () . create_err (errors :: ExpectedRegisterClassOrExplicitRegister { span : p . token . span , })) ; } } ; p . bump () ; p . expect (exp ! (CloseParen)) ? ; Ok (result) }
}