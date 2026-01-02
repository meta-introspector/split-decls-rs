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
mkuse!{use lint :: BuiltinLintDiag ;}
mkuse!{use rustc_ast :: tokenstream :: TokenStream ;}
mkuse!{use rustc_ast :: { AsmMacro , token } ;}
mkuse!{use rustc_data_structures :: fx :: { FxHashMap , FxIndexMap } ;}
mkuse!{use rustc_errors :: PResult ;}
mkuse!{use rustc_expand :: base :: * ;}
mkuse!{use rustc_index :: bit_set :: GrowableBitSet ;}
mkuse!{use rustc_parse :: parser :: asm :: * ;}
mkuse!{use rustc_session :: lint ;}
mkuse!{use rustc_session :: parse :: feature_err ;}
mkuse!{use rustc_span :: { ErrorGuaranteed , InnerSpan , Span , Symbol , sym } ;}
mkuse!{use rustc_target :: asm :: InlineAsmArch ;}
mkuse!{use smallvec :: smallvec ;}
mkuse!{use { rustc_ast as ast , rustc_parse_format as parse } ;}
mkuse!{use crate :: util :: { ExprToSpannedString , expr_to_spanned_string } ;}
mkuse!{use crate :: { errors , fluent_generated as fluent } ;}
mkitem!{mkstruct!{# [doc = " Validated assembly arguments, ready for macro expansion."] struct ValidatedAsmArgs { pub templates : Vec < Box < ast :: Expr > > , pub operands : Vec < (ast :: InlineAsmOperand , Span) > , named_args : FxIndexMap < Symbol , usize > , reg_args : GrowableBitSet < usize > , pub clobber_abis : Vec < (Symbol , Span) > , options : ast :: InlineAsmOptions , pub options_spans : Vec < Span > , }}}

macro_rules! parse_args_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_args in module {}", module_path!());
    };
}

mkfn!{
    parse_args_introspect!();
    fn parse_args < 'a > (ecx : & ExtCtxt < 'a > , sp : Span , tts : TokenStream , asm_macro : AsmMacro ,) -> PResult < 'a , ValidatedAsmArgs > { let args = parse_asm_args (& mut ecx . new_parser_from_tts (tts) , sp , asm_macro) ? ; validate_asm_args (ecx , asm_macro , args) }
}

macro_rules! validate_asm_args_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function validate_asm_args in module {}", module_path!());
    };
}

mkfn!{
    validate_asm_args_introspect!();
    fn validate_asm_args < 'a > (ecx : & ExtCtxt < 'a > , asm_macro : AsmMacro , args : Vec < AsmArg > ,) -> PResult < 'a , ValidatedAsmArgs > { let dcx = ecx . dcx () ; let strip_unconfigured = rustc_expand :: config :: StripUnconfigured { sess : ecx . sess , features : Some (ecx . ecfg . features) , config_tokens : false , lint_node_id : ecx . current_expansion . lint_node_id , } ; let mut validated = ValidatedAsmArgs { templates : vec ! [] , operands : vec ! [] , named_args : Default :: default () , reg_args : Default :: default () , clobber_abis : Vec :: new () , options : ast :: InlineAsmOptions :: empty () , options_spans : vec ! [] , } ; let mut allow_templates = true ; for arg in args { for attr in arg . attributes . 0 . iter () { match attr . name () { Some (sym :: cfg | sym :: cfg_attr) => { if ! ecx . ecfg . features . asm_cfg () { let span = attr . span () ; feature_err (ecx . sess , sym :: asm_cfg , span , fluent :: builtin_macros_asm_cfg) . emit () ; } } _ => { ecx . dcx () . emit_err (errors :: AsmAttributeNotSupported { span : attr . span () }) ; } } } if ecx . ecfg . features . asm_cfg () && strip_unconfigured . configure (arg . attributes) . is_none () { continue ; } match arg . kind { AsmArgKind :: Template (template) => { if ! allow_templates { match template . kind { ast :: ExprKind :: Lit (token_lit) if matches ! (token_lit . kind , token :: LitKind :: Str | token :: LitKind :: StrRaw (_)) => { } ast :: ExprKind :: MacCall (..) => { } _ => { let err = dcx . create_err (errors :: AsmExpectedOther { span : template . span , is_inline_asm : matches ! (asm_macro , AsmMacro :: Asm) , }) ; return Err (err) ; } } } validated . templates . push (template) ; } AsmArgKind :: Operand (name , op) => { allow_templates = false ; let explicit_reg = matches ! (op . reg () , Some (ast :: InlineAsmRegOrRegClass :: Reg (_))) ; let span = arg . span ; let slot = validated . operands . len () ; validated . operands . push ((op , span)) ; if explicit_reg { if name . is_some () { dcx . emit_err (errors :: AsmExplicitRegisterName { span }) ; } validated . reg_args . insert (slot) ; } else if let Some (name) = name { if let Some (& prev) = validated . named_args . get (& name) { dcx . emit_err (errors :: AsmDuplicateArg { span , name , prev : validated . operands [prev] . 1 , }) ; continue ; } validated . named_args . insert (name , slot) ; } else if ! validated . named_args . is_empty () || ! validated . reg_args . is_empty () { let named = validated . named_args . values () . map (| p | validated . operands [* p] . 1) . collect () ; let explicit = validated . reg_args . iter () . map (| p | validated . operands [p] . 1) . collect () ; dcx . emit_err (errors :: AsmPositionalAfter { span , named , explicit }) ; } } AsmArgKind :: Options (new_options) => { allow_templates = false ; for asm_option in new_options { let AsmOption { span , symbol , span_with_comma , options } = asm_option ; if ! asm_macro . is_supported_option (options) { dcx . emit_err (errors :: AsmUnsupportedOption { span , symbol , span_with_comma , macro_name : asm_macro . macro_name () , }) ; } else if validated . options . contains (options) { dcx . emit_err (errors :: AsmOptAlreadyprovided { span , symbol , span_with_comma , }) ; } else { validated . options |= asm_option . options ; } } validated . options_spans . push (arg . span) ; } AsmArgKind :: ClobberAbi (new_abis) => { allow_templates = false ; match & new_abis [..] { [] => unreachable ! () , [(abi , _span)] => validated . clobber_abis . push ((* abi , arg . span)) , _ => validated . clobber_abis . extend (new_abis) , } } } } if validated . options . contains (ast :: InlineAsmOptions :: NOMEM) && validated . options . contains (ast :: InlineAsmOptions :: READONLY) { let spans = validated . options_spans . clone () ; dcx . emit_err (errors :: AsmMutuallyExclusive { spans , opt1 : "nomem" , opt2 : "readonly" }) ; } if validated . options . contains (ast :: InlineAsmOptions :: PURE) && validated . options . contains (ast :: InlineAsmOptions :: NORETURN) { let spans = validated . options_spans . clone () ; dcx . emit_err (errors :: AsmMutuallyExclusive { spans , opt1 : "pure" , opt2 : "noreturn" }) ; } if validated . options . contains (ast :: InlineAsmOptions :: PURE) && ! validated . options . intersects (ast :: InlineAsmOptions :: NOMEM | ast :: InlineAsmOptions :: READONLY) { let spans = validated . options_spans . clone () ; dcx . emit_err (errors :: AsmPureCombine { spans }) ; } let mut have_real_output = false ; let mut outputs_sp = vec ! [] ; let mut regclass_outputs = vec ! [] ; let mut labels_sp = vec ! [] ; for (op , op_sp) in & validated . operands { match op { ast :: InlineAsmOperand :: Out { reg , expr , .. } | ast :: InlineAsmOperand :: SplitInOut { reg , out_expr : expr , .. } => { outputs_sp . push (* op_sp) ; have_real_output |= expr . is_some () ; if let ast :: InlineAsmRegOrRegClass :: RegClass (_) = reg { regclass_outputs . push (* op_sp) ; } } ast :: InlineAsmOperand :: InOut { reg , .. } => { outputs_sp . push (* op_sp) ; have_real_output = true ; if let ast :: InlineAsmRegOrRegClass :: RegClass (_) = reg { regclass_outputs . push (* op_sp) ; } } ast :: InlineAsmOperand :: Label { .. } => { labels_sp . push (* op_sp) ; } _ => { } } } if validated . options . contains (ast :: InlineAsmOptions :: PURE) && ! have_real_output { dcx . emit_err (errors :: AsmPureNoOutput { spans : validated . options_spans . clone () }) ; } if validated . options . contains (ast :: InlineAsmOptions :: NORETURN) && ! outputs_sp . is_empty () && labels_sp . is_empty () { let err = dcx . create_err (errors :: AsmNoReturn { outputs_sp }) ; return Err (err) ; } if validated . options . contains (ast :: InlineAsmOptions :: MAY_UNWIND) && ! labels_sp . is_empty () { dcx . emit_err (errors :: AsmMayUnwind { labels_sp }) ; } if ! validated . clobber_abis . is_empty () { match asm_macro { AsmMacro :: GlobalAsm | AsmMacro :: NakedAsm => { let err = dcx . create_err (errors :: AsmUnsupportedClobberAbi { spans : validated . clobber_abis . iter () . map (| (_ , span) | * span) . collect () , macro_name : asm_macro . macro_name () , }) ; return Err (err) ; } AsmMacro :: Asm => { if ! regclass_outputs . is_empty () { dcx . emit_err (errors :: AsmClobberNoReg { spans : regclass_outputs , clobbers : validated . clobber_abis . iter () . map (| (_ , span) | * span) . collect () , }) ; } } } } Ok (validated) }
}

macro_rules! expand_preparsed_asm_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function expand_preparsed_asm in module {}", module_path!());
    };
}

mkfn!{
    expand_preparsed_asm_introspect!();
    fn expand_preparsed_asm (ecx : & mut ExtCtxt < '_ > , asm_macro : AsmMacro , args : ValidatedAsmArgs ,) -> ExpandResult < Result < ast :: InlineAsm , ErrorGuaranteed > , () > { let mut template = vec ! [] ; let mut used = vec ! [false ; args . operands . len ()] ; for pos in args . reg_args . iter () { used [pos] = true ; } let named_pos : FxHashMap < usize , Symbol > = args . named_args . iter () . map (| (& sym , & idx) | (idx , sym)) . collect () ; let mut line_spans = Vec :: with_capacity (args . templates . len ()) ; let mut curarg = 0 ; let mut template_strs = Vec :: with_capacity (args . templates . len ()) ; for (i , template_expr) in args . templates . into_iter () . enumerate () { if i != 0 { template . push (ast :: InlineAsmTemplatePiece :: String ("\n" . into ())) ; } let msg = "asm template must be a string literal" ; let template_sp = template_expr . span ; let template_is_mac_call = matches ! (template_expr . kind , ast :: ExprKind :: MacCall (_)) ; let ExprToSpannedString { symbol : template_str , style : template_style , span : template_span , .. } = { let ExpandResult :: Ready (mac) = expr_to_spanned_string (ecx , template_expr , msg) else { return ExpandResult :: Retry (()) ; } ; match mac { Ok (template_part) => template_part , Err (err) => { return ExpandResult :: Ready (Err (match err { Ok ((err , _)) => err . emit () , Err (guar) => guar , })) ; } } } ; let str_style = match template_style { ast :: StrStyle :: Cooked => None , ast :: StrStyle :: Raw (raw) => Some (raw as usize) , } ; let template_snippet = ecx . source_map () . span_to_snippet (template_sp) . ok () ; template_strs . push ((template_str , template_snippet . as_deref () . map (Symbol :: intern) , template_sp ,)) ; let template_str = template_str . as_str () ; if let Some (InlineAsmArch :: X86 | InlineAsmArch :: X86_64) = ecx . sess . asm_arch { let find_span = | needle : & str | -> Span { if let Some (snippet) = & template_snippet { if let Some (pos) = snippet . find (needle) { let end = pos + snippet [pos ..] . find (| c | matches ! (c , '\n' | ';' | '\\' | '"')) . unwrap_or (snippet [pos ..] . len () - 1) ; let inner = InnerSpan :: new (pos , end) ; return template_sp . from_inner (inner) ; } } template_sp } ; if template_str . contains (".intel_syntax") { ecx . psess () . buffer_lint (lint :: builtin :: BAD_ASM_STYLE , find_span (".intel_syntax") , ecx . current_expansion . lint_node_id , BuiltinLintDiag :: AvoidUsingIntelSyntax ,) ; } if template_str . contains (".att_syntax") { ecx . psess () . buffer_lint (lint :: builtin :: BAD_ASM_STYLE , find_span (".att_syntax") , ecx . current_expansion . lint_node_id , BuiltinLintDiag :: AvoidUsingAttSyntax ,) ; } } if args . options . contains (ast :: InlineAsmOptions :: RAW) { template . push (ast :: InlineAsmTemplatePiece :: String (template_str . to_string () . into ())) ; let template_num_lines = 1 + template_str . matches ('\n') . count () ; line_spans . extend (std :: iter :: repeat (template_sp) . take (template_num_lines)) ; continue ; } let mut parser = parse :: Parser :: new (template_str , str_style , template_snippet , false , parse :: ParseMode :: InlineAsm ,) ; parser . curarg = curarg ; let mut unverified_pieces = Vec :: new () ; while let Some (piece) = parser . next () { if ! parser . errors . is_empty () { break ; } else { unverified_pieces . push (piece) ; } } if ! parser . errors . is_empty () { let err = parser . errors . remove (0) ; let err_sp = if template_is_mac_call { template_span } else { template_span . from_inner (InnerSpan :: new (err . span . start , err . span . end)) } ; let msg = format ! ("invalid asm template string: {}" , err . description) ; let mut e = ecx . dcx () . struct_span_err (err_sp , msg) ; e . span_label (err_sp , err . label + " in asm template string") ; if let Some (note) = err . note { e . note (note) ; } if let Some ((label , span)) = err . secondary_label { let err_sp = template_span . from_inner (InnerSpan :: new (span . start , span . end)) ; e . span_label (err_sp , label) ; } let guar = e . emit () ; return ExpandResult :: Ready (Err (guar)) ; } curarg = parser . curarg ; let mut arg_spans = parser . arg_places . iter () . map (| span | template_span . from_inner (InnerSpan :: new (span . start , span . end))) ; for piece in unverified_pieces { match piece { parse :: Piece :: Lit (s) => { template . push (ast :: InlineAsmTemplatePiece :: String (s . to_string () . into ())) } parse :: Piece :: NextArgument (arg) => { let span = arg_spans . next () . unwrap_or (template_sp) ; let operand_idx = match arg . position { parse :: ArgumentIs (idx) | parse :: ArgumentImplicitlyIs (idx) => { if idx >= args . operands . len () || named_pos . contains_key (& idx) || args . reg_args . contains (idx) { let msg = format ! ("invalid reference to argument at index {idx}") ; let mut err = ecx . dcx () . struct_span_err (span , msg) ; err . span_label (span , "from here") ; let positional_args = args . operands . len () - args . named_args . len () - args . reg_args . len () ; let positional = if positional_args != args . operands . len () { "positional " } else { "" } ; let msg = match positional_args { 0 => format ! ("no {positional}arguments were given") , 1 => format ! ("there is 1 {positional}argument") , x => format ! ("there are {x} {positional}arguments") , } ; err . note (msg) ; if named_pos . contains_key (& idx) { err . span_label (args . operands [idx] . 1 , "named argument") ; err . span_note (args . operands [idx] . 1 , "named arguments cannot be referenced by position" ,) ; } else if args . reg_args . contains (idx) { err . span_label (args . operands [idx] . 1 , "explicit register argument" ,) ; err . span_note (args . operands [idx] . 1 , "explicit register arguments cannot be used in the asm template" ,) ; err . span_help (args . operands [idx] . 1 , "use the register name directly in the assembly code" ,) ; } err . emit () ; None } else { Some (idx) } } parse :: ArgumentNamed (name) => { match args . named_args . get (& Symbol :: intern (name)) { Some (& idx) => Some (idx) , None => { let span = arg . position_span ; ecx . dcx () . create_err (errors :: AsmNoMatchedArgumentName { name : name . to_owned () , span : template_span . from_inner (InnerSpan :: new (span . start , span . end)) , }) . emit () ; None } } } } ; let mut chars = arg . format . ty . chars () ; let mut modifier = chars . next () ; if chars . next () . is_some () { let span = arg . format . ty_span . map (| sp | template_sp . from_inner (InnerSpan :: new (sp . start , sp . end))) . unwrap_or (template_sp) ; ecx . dcx () . emit_err (errors :: AsmModifierInvalid { span }) ; modifier = None ; } if let Some (operand_idx) = operand_idx { used [operand_idx] = true ; template . push (ast :: InlineAsmTemplatePiece :: Placeholder { operand_idx , modifier , span , }) ; } } } } if parser . line_spans . is_empty () { let template_num_lines = 1 + template_str . matches ('\n') . count () ; line_spans . extend (std :: iter :: repeat (template_sp) . take (template_num_lines)) ; } else { line_spans . extend (parser . line_spans . iter () . map (| span | template_span . from_inner (InnerSpan :: new (span . start , span . end))) ,) ; } ; } let mut unused_operands = vec ! [] ; let mut help_str = String :: new () ; for (idx , used) in used . into_iter () . enumerate () { if ! used { let msg = if let Some (sym) = named_pos . get (& idx) { help_str . push_str (& format ! (" {{{}}}" , sym)) ; "named argument never used" } else { help_str . push_str (& format ! (" {{{}}}" , idx)) ; "argument never used" } ; unused_operands . push ((args . operands [idx] . 1 , msg)) ; } } match unused_operands [..] { [] => { } [(sp , msg)] => { ecx . dcx () . struct_span_err (sp , msg) . with_span_label (sp , msg) . with_help (format ! ("if this argument is intentionally unused, \
                     consider using it in an asm comment: `\"/*{help_str} */\"`")) . emit () ; } _ => { let mut err = ecx . dcx () . struct_span_err (unused_operands . iter () . map (| & (sp , _) | sp) . collect :: < Vec < Span > > () , "multiple unused asm arguments" ,) ; for (sp , msg) in unused_operands { err . span_label (sp , msg) ; } err . help (format ! ("if these arguments are intentionally unused, \
                 consider using them in an asm comment: `\"/*{help_str} */\"`")) ; err . emit () ; } } ExpandResult :: Ready (Ok (ast :: InlineAsm { asm_macro , template , template_strs : template_strs . into_boxed_slice () , operands : args . operands , clobber_abis : args . clobber_abis , options : args . options , line_spans , })) }
}

macro_rules! expand_asm_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function expand_asm in module {}", module_path!());
    };
}

mkfn!{
    expand_asm_introspect!();
    pub (super) fn expand_asm < 'cx > (ecx : & 'cx mut ExtCtxt < '_ > , sp : Span , tts : TokenStream ,) -> MacroExpanderResult < 'cx > { ExpandResult :: Ready (match parse_args (ecx , sp , tts , AsmMacro :: Asm) { Ok (args) => { let ExpandResult :: Ready (mac) = expand_preparsed_asm (ecx , AsmMacro :: Asm , args) else { return ExpandResult :: Retry (()) ; } ; let expr = match mac { Ok (inline_asm) => Box :: new (ast :: Expr { id : ast :: DUMMY_NODE_ID , kind : ast :: ExprKind :: InlineAsm (Box :: new (inline_asm)) , span : sp , attrs : ast :: AttrVec :: new () , tokens : None , }) , Err (guar) => DummyResult :: raw_expr (sp , Some (guar)) , } ; MacEager :: expr (expr) } Err (err) => { let guar = err . emit () ; DummyResult :: any (sp , guar) } }) }
}

macro_rules! expand_naked_asm_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function expand_naked_asm in module {}", module_path!());
    };
}

mkfn!{
    expand_naked_asm_introspect!();
    pub (super) fn expand_naked_asm < 'cx > (ecx : & 'cx mut ExtCtxt < '_ > , sp : Span , tts : TokenStream ,) -> MacroExpanderResult < 'cx > { ExpandResult :: Ready (match parse_args (ecx , sp , tts , AsmMacro :: NakedAsm) { Ok (args) => { let ExpandResult :: Ready (mac) = expand_preparsed_asm (ecx , AsmMacro :: NakedAsm , args) else { return ExpandResult :: Retry (()) ; } ; let expr = match mac { Ok (inline_asm) => Box :: new (ast :: Expr { id : ast :: DUMMY_NODE_ID , kind : ast :: ExprKind :: InlineAsm (Box :: new (inline_asm)) , span : sp , attrs : ast :: AttrVec :: new () , tokens : None , }) , Err (guar) => DummyResult :: raw_expr (sp , Some (guar)) , } ; MacEager :: expr (expr) } Err (err) => { let guar = err . emit () ; DummyResult :: any (sp , guar) } }) }
}

macro_rules! expand_global_asm_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function expand_global_asm in module {}", module_path!());
    };
}

mkfn!{
    expand_global_asm_introspect!();
    pub (super) fn expand_global_asm < 'cx > (ecx : & 'cx mut ExtCtxt < '_ > , sp : Span , tts : TokenStream ,) -> MacroExpanderResult < 'cx > { ExpandResult :: Ready (match parse_args (ecx , sp , tts , AsmMacro :: GlobalAsm) { Ok (args) => { let ExpandResult :: Ready (mac) = expand_preparsed_asm (ecx , AsmMacro :: GlobalAsm , args) else { return ExpandResult :: Retry (()) ; } ; match mac { Ok (inline_asm) => MacEager :: items (smallvec ! [Box :: new (ast :: Item { attrs : ast :: AttrVec :: new () , id : ast :: DUMMY_NODE_ID , kind : ast :: ItemKind :: GlobalAsm (Box :: new (inline_asm)) , vis : ast :: Visibility { span : sp . shrink_to_lo () , kind : ast :: VisibilityKind :: Inherited , tokens : None , } , span : sp , tokens : None , })]) , Err (guar) => DummyResult :: any (sp , guar) , } } Err (err) => { let guar = err . emit () ; DummyResult :: any (sp , guar) } }) }
}