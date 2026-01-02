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
mkuse!{use std :: ops :: Range ;}
mkuse!{use parse :: Position :: ArgumentNamed ;}
mkuse!{use rustc_ast :: tokenstream :: TokenStream ;}
mkuse!{use rustc_ast :: { Expr , ExprKind , FormatAlignment , FormatArgPosition , FormatArgPositionKind , FormatArgs , FormatArgsPiece , FormatArgument , FormatArgumentKind , FormatArguments , FormatCount , FormatDebugHex , FormatOptions , FormatPlaceholder , FormatSign , FormatTrait , Recovered , StmtKind , token , } ;}
mkuse!{use rustc_data_structures :: fx :: FxHashSet ;}
mkuse!{use rustc_errors :: { Applicability , BufferedEarlyLint , Diag , MultiSpan , PResult , SingleLabelManySpans , listify , pluralize , } ;}
mkuse!{use rustc_expand :: base :: * ;}
mkuse!{use rustc_lint_defs :: builtin :: NAMED_ARGUMENTS_USED_POSITIONALLY ;}
mkuse!{use rustc_lint_defs :: { BuiltinLintDiag , LintId } ;}
mkuse!{use rustc_parse :: exp ;}
mkuse!{use rustc_parse_format as parse ;}
mkuse!{use rustc_span :: { BytePos , ErrorGuaranteed , Ident , InnerSpan , Span , Symbol } ;}
mkuse!{use crate :: errors ;}
mkuse!{use crate :: util :: { ExprToSpannedString , expr_to_spanned_string } ;}
mkitem!{mkenum!{#[derive (Clone , Copy , Debug , PartialEq , Eq)] enum PositionUsedAs { Placeholder (Option < Span >) , Precision , Width , }}}
mkuse!{use PositionUsedAs :: * ;}
mkitem!{mkstruct!{#[derive (Debug)] struct MacroInput { fmtstr : Box < Expr > , args : FormatArguments , #[doc = " Whether the first argument was a string literal or a result from eager macro expansion."] #[doc = " If it's not a string literal, we disallow implicit argument capturing."] #[doc = ""] #[doc = " This does not correspond to whether we can treat spans to the literal normally, as the whole"] #[doc = " invocation might be the result of another macro expansion, in which case this flag may still be true."] #[doc = ""] #[doc = " See [RFC 2795] for more information."] #[doc = ""] #[doc = " [RFC 2795]: https://rust-lang.github.io/rfcs/2795-format-args-implicit-identifiers.html#macro-hygiene"] is_direct_literal : bool , }}}

macro_rules! parse_args_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_args in module {}", module_path!());
    };
}

mkfn!{
    parse_args_introspect!();
    #[doc = " Parses the arguments from the given list of tokens, returning the diagnostic"] #[doc = " if there's a parse error so we can continue parsing other format!"] #[doc = " expressions."] #[doc = ""] #[doc = " If parsing succeeds, the return value is:"] #[doc = ""] #[doc = " ```text"] #[doc = " Ok((fmtstr, parsed arguments))"] #[doc = " ```"] fn parse_args < 'a > (ecx : & ExtCtxt < 'a > , sp : Span , tts : TokenStream) -> PResult < 'a , MacroInput > { let mut args = FormatArguments :: new () ; let mut p = ecx . new_parser_from_tts (tts) ; if p . token == token :: Eof { return Err (ecx . dcx () . create_err (errors :: FormatRequiresString { span : sp })) ; } let first_token = & p . token ; let fmtstr = if let token :: Literal (lit) = first_token . kind && matches ! (lit . kind , token :: Str | token :: StrRaw (_)) { p . parse_literal_maybe_minus () ? } else { p . parse_expr () ? } ; let is_direct_literal = matches ! (fmtstr . kind , ExprKind :: Lit (_)) ; let mut first = true ; while p . token != token :: Eof { if ! p . eat (exp ! (Comma)) { if first { p . clear_expected_token_types () ; } match p . expect (exp ! (Comma)) { Err (err) => { if token :: TokenKind :: Comma . similar_tokens () . contains (& p . token . kind) { err . emit () ; p . bump () ; } else { return Err (err) ; } } Ok (Recovered :: Yes (_)) => () , Ok (Recovered :: No) => unreachable ! () , } } first = false ; if p . token == token :: Eof { break ; } match p . token . ident () { Some ((ident , _)) if p . look_ahead (1 , | t | * t == token :: Eq) => { p . bump () ; p . expect (exp ! (Eq)) ? ; let expr = p . parse_expr () ? ; if let Some ((_ , prev)) = args . by_name (ident . name) { ecx . dcx () . emit_err (errors :: FormatDuplicateArg { span : ident . span , prev : prev . kind . ident () . unwrap () . span , duplicate : ident . span , ident , }) ; continue ; } args . add (FormatArgument { kind : FormatArgumentKind :: Named (ident) , expr }) ; } _ => { let expr = p . parse_expr () ? ; if ! args . named_args () . is_empty () { return Err (ecx . dcx () . create_err (errors :: PositionalAfterNamed { span : expr . span , args : args . named_args () . iter () . filter_map (| a | a . kind . ident () . map (| ident | (a , ident))) . map (| (arg , n) | n . span . to (arg . expr . span)) . collect () , })) ; } args . add (FormatArgument { kind : FormatArgumentKind :: Normal , expr }) ; } } } Ok (MacroInput { fmtstr , args , is_direct_literal }) }
}

macro_rules! make_format_args_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function make_format_args in module {}", module_path!());
    };
}

mkfn!{
    make_format_args_introspect!();
    fn make_format_args (ecx : & mut ExtCtxt < '_ > , input : MacroInput , append_newline : bool ,) -> ExpandResult < Result < FormatArgs , ErrorGuaranteed > , () > { let msg = "format argument must be a string literal" ; let unexpanded_fmt_span = input . fmtstr . span ; let MacroInput { fmtstr : efmt , mut args , is_direct_literal } = input ; let ExprToSpannedString { symbol : fmt_str , span : fmt_span , style : fmt_style , uncooked_symbol : uncooked_fmt_str , } = { let ExpandResult :: Ready (mac) = expr_to_spanned_string (ecx , efmt . clone () , msg) else { return ExpandResult :: Retry (()) ; } ; match mac { Ok (mut fmt) if append_newline => { fmt . symbol = Symbol :: intern (& format ! ("{}\n" , fmt . symbol)) ; fmt } Ok (fmt) => fmt , Err (err) => { let guar = match err { Ok ((mut err , suggested)) => { if ! suggested { if let ExprKind :: Block (block , None) = & efmt . kind && let [stmt] = block . stmts . as_slice () && let StmtKind :: Expr (expr) = & stmt . kind && let ExprKind :: Path (None , path) = & expr . kind && path . segments . len () == 1 && path . segments [0] . args . is_none () { err . multipart_suggestion ("quote your inlined format argument to use as string literal" , vec ! [(unexpanded_fmt_span . shrink_to_hi () , "\"" . to_string ()) , (unexpanded_fmt_span . shrink_to_lo () , "\"" . to_string ()) ,] , Applicability :: MaybeIncorrect ,) ; } else { let should_suggest = | kind : & ExprKind | -> bool { match kind { ExprKind :: Block (b , None) if b . stmts . is_empty () => true , ExprKind :: Tup (v) if v . is_empty () => true , _ => false , } } ; let mut sugg_fmt = String :: new () ; for kind in std :: iter :: once (& efmt . kind) . chain (args . explicit_args () . into_iter () . map (| a | & a . expr . kind)) { sugg_fmt . push_str (if should_suggest (kind) { "{:?} " } else { "{} " }) ; } sugg_fmt = sugg_fmt . trim_end () . to_string () ; err . span_suggestion (unexpanded_fmt_span . shrink_to_lo () , "you might be missing a string literal to format with" , format ! ("\"{sugg_fmt}\", ") , Applicability :: MaybeIncorrect ,) ; } } err . emit () } Err (guar) => guar , } ; return ExpandResult :: Ready (Err (guar)) ; } } } ; let str_style = match fmt_style { rustc_ast :: StrStyle :: Cooked => None , rustc_ast :: StrStyle :: Raw (raw) => Some (raw as usize) , } ; let fmt_str = fmt_str . as_str () ; let fmt_snippet = ecx . source_map () . span_to_snippet (unexpanded_fmt_span) . ok () ; let mut parser = parse :: Parser :: new (fmt_str , str_style , fmt_snippet , append_newline , parse :: ParseMode :: Format ,) ; let mut pieces = Vec :: new () ; while let Some (piece) = parser . next () { if ! parser . errors . is_empty () { break ; } else { pieces . push (piece) ; } } let is_source_literal = parser . is_source_literal ; if ! parser . errors . is_empty () { let err = parser . errors . remove (0) ; let sp = if is_source_literal { fmt_span . from_inner (InnerSpan :: new (err . span . start , err . span . end)) } else { fmt_span } ; let mut e = errors :: InvalidFormatString { span : sp , note_ : None , label_ : None , sugg_ : None , desc : err . description , label1 : err . label , } ; if let Some (note) = err . note { e . note_ = Some (errors :: InvalidFormatStringNote { note }) ; } if let Some ((label , span)) = err . secondary_label && is_source_literal { e . label_ = Some (errors :: InvalidFormatStringLabel { span : fmt_span . from_inner (InnerSpan :: new (span . start , span . end)) , label , }) ; } match err . suggestion { parse :: Suggestion :: None => { } parse :: Suggestion :: UsePositional => { let captured_arg_span = fmt_span . from_inner (InnerSpan :: new (err . span . start , err . span . end)) ; if let Ok (arg) = ecx . source_map () . span_to_snippet (captured_arg_span) { let span = match args . unnamed_args () . last () { Some (arg) => arg . expr . span , None => fmt_span , } ; e . sugg_ = Some (errors :: InvalidFormatStringSuggestion :: UsePositional { captured : captured_arg_span , len : args . unnamed_args () . len () . to_string () , span : span . shrink_to_hi () , arg , }) ; } } parse :: Suggestion :: RemoveRawIdent (span) => { if is_source_literal { let span = fmt_span . from_inner (InnerSpan :: new (span . start , span . end)) ; e . sugg_ = Some (errors :: InvalidFormatStringSuggestion :: RemoveRawIdent { span }) } } parse :: Suggestion :: ReorderFormatParameter (span , replacement) => { let span = fmt_span . from_inner (InnerSpan :: new (span . start , span . end)) ; e . sugg_ = Some (errors :: InvalidFormatStringSuggestion :: ReorderFormatParameter { span , replacement , }) ; } } let guar = ecx . dcx () . emit_err (e) ; return ExpandResult :: Ready (Err (guar)) ; } let to_span = | inner_span : Range < usize > | { is_source_literal . then (| | { fmt_span . from_inner (InnerSpan { start : inner_span . start , end : inner_span . end }) }) } ; let mut used = vec ! [false ; args . explicit_args () . len ()] ; let mut invalid_refs = Vec :: new () ; let mut numeric_references_to_named_arg = Vec :: new () ; enum ArgRef < 'a > { Index (usize) , Name (& 'a str , Option < Span >) , } use ArgRef :: * ; let mut unnamed_arg_after_named_arg = false ; let mut lookup_arg = | arg : ArgRef < '_ > , span : Option < Span > , used_as : PositionUsedAs , kind : FormatArgPositionKind | -> FormatArgPosition { let index = match arg { Index (index) => { if let Some (arg) = args . by_index (index) { used [index] = true ; if arg . kind . ident () . is_some () { numeric_references_to_named_arg . push ((index , span , used_as)) ; } Ok (index) } else { invalid_refs . push ((index , span , used_as , kind)) ; Err (index) } } Name (name , span) => { let name = Symbol :: intern (name) ; if let Some ((index , _)) = args . by_name (name) { if index < args . explicit_args () . len () { used [index] = true ; } Ok (index) } else { let span = span . unwrap_or (fmt_span) ; let ident = Ident :: new (name , span) ; let expr = if is_direct_literal { ecx . expr_ident (span , ident) } else { let guar = ecx . dcx () . emit_err (errors :: FormatNoArgNamed { span , name }) ; unnamed_arg_after_named_arg = true ; DummyResult :: raw_expr (span , Some (guar)) } ; Ok (args . add (FormatArgument { kind : FormatArgumentKind :: Captured (ident) , expr })) } } } ; FormatArgPosition { index , kind , span } } ; let mut template = Vec :: new () ; let mut unfinished_literal = String :: new () ; let mut placeholder_index = 0 ; for piece in & pieces { match piece . clone () { parse :: Piece :: Lit (s) => { unfinished_literal . push_str (s) ; } parse :: Piece :: NextArgument (box parse :: Argument { position , position_span , format }) => { if ! unfinished_literal . is_empty () { template . push (FormatArgsPiece :: Literal (Symbol :: intern (& unfinished_literal))) ; unfinished_literal . clear () ; } let span = parser . arg_places . get (placeholder_index) . and_then (| s | to_span (s . clone ())) ; placeholder_index += 1 ; let position_span = to_span (position_span) ; let argument = match position { parse :: ArgumentImplicitlyIs (i) => lookup_arg (Index (i) , position_span , Placeholder (span) , FormatArgPositionKind :: Implicit ,) , parse :: ArgumentIs (i) => lookup_arg (Index (i) , position_span , Placeholder (span) , FormatArgPositionKind :: Number ,) , parse :: ArgumentNamed (name) => lookup_arg (Name (name , position_span) , position_span , Placeholder (span) , FormatArgPositionKind :: Named ,) , } ; let alignment = match format . align { parse :: AlignUnknown => None , parse :: AlignLeft => Some (FormatAlignment :: Left) , parse :: AlignRight => Some (FormatAlignment :: Right) , parse :: AlignCenter => Some (FormatAlignment :: Center) , } ; let format_trait = match format . ty { "" => FormatTrait :: Display , "?" => FormatTrait :: Debug , "e" => FormatTrait :: LowerExp , "E" => FormatTrait :: UpperExp , "o" => FormatTrait :: Octal , "p" => FormatTrait :: Pointer , "b" => FormatTrait :: Binary , "x" => FormatTrait :: LowerHex , "X" => FormatTrait :: UpperHex , _ => { invalid_placeholder_type_error (ecx , format . ty , format . ty_span , fmt_span) ; FormatTrait :: Display } } ; let precision_span = format . precision_span . and_then (to_span) ; let precision = match format . precision { parse :: CountIs (n) => Some (FormatCount :: Literal (n)) , parse :: CountIsName (name , name_span) => Some (FormatCount :: Argument (lookup_arg (Name (name , to_span (name_span)) , precision_span , Precision , FormatArgPositionKind :: Named ,))) , parse :: CountIsParam (i) => Some (FormatCount :: Argument (lookup_arg (Index (i) , precision_span , Precision , FormatArgPositionKind :: Number ,))) , parse :: CountIsStar (i) => Some (FormatCount :: Argument (lookup_arg (Index (i) , precision_span , Precision , FormatArgPositionKind :: Implicit ,))) , parse :: CountImplied => None , } ; let width_span = format . width_span . and_then (to_span) ; let width = match format . width { parse :: CountIs (n) => Some (FormatCount :: Literal (n)) , parse :: CountIsName (name , name_span) => Some (FormatCount :: Argument (lookup_arg (Name (name , to_span (name_span)) , width_span , Width , FormatArgPositionKind :: Named ,))) , parse :: CountIsParam (i) => Some (FormatCount :: Argument (lookup_arg (Index (i) , width_span , Width , FormatArgPositionKind :: Number ,))) , parse :: CountIsStar (_) => unreachable ! () , parse :: CountImplied => None , } ; template . push (FormatArgsPiece :: Placeholder (FormatPlaceholder { argument , span , format_trait , format_options : FormatOptions { fill : format . fill , alignment , sign : format . sign . map (| s | match s { parse :: Sign :: Plus => FormatSign :: Plus , parse :: Sign :: Minus => FormatSign :: Minus , }) , alternate : format . alternate , zero_pad : format . zero_pad , debug_hex : format . debug_hex . map (| s | match s { parse :: DebugHex :: Lower => FormatDebugHex :: Lower , parse :: DebugHex :: Upper => FormatDebugHex :: Upper , }) , precision , width , } , })) ; } } } if ! unfinished_literal . is_empty () { template . push (FormatArgsPiece :: Literal (Symbol :: intern (& unfinished_literal))) ; } if ! invalid_refs . is_empty () { report_invalid_references (ecx , & invalid_refs , & template , fmt_span , & args , parser) ; } let unused = used . iter () . enumerate () . filter (| & (_ , used) | ! used) . map (| (i , _) | { let named = matches ! (args . explicit_args () [i] . kind , FormatArgumentKind :: Named (_)) ; (args . explicit_args () [i] . expr . span , named) }) . collect :: < Vec < _ > > () ; let has_unused = ! unused . is_empty () ; if has_unused { let detect_foreign_fmt = unused . len () > args . explicit_args () . len () / 2 ; report_missing_placeholders (ecx , unused , & used , & args , & pieces , & invalid_refs , detect_foreign_fmt , str_style , fmt_str , uncooked_fmt_str . 1 . as_str () , fmt_span ,) ; } if invalid_refs . is_empty () && ! has_unused && ! unnamed_arg_after_named_arg { for & (index , span , used_as) in & numeric_references_to_named_arg { let (position_sp_to_replace , position_sp_for_msg) = match used_as { Placeholder (pspan) => (span , pspan) , Precision => { let span = span . map (| span | span . with_lo (span . lo () + BytePos (1))) ; (span , span) } Width => (span , span) , } ; let arg_name = args . explicit_args () [index] . kind . ident () . unwrap () ; ecx . buffered_early_lint . push (BufferedEarlyLint { span : Some (arg_name . span . into ()) , node_id : rustc_ast :: CRATE_NODE_ID , lint_id : LintId :: of (NAMED_ARGUMENTS_USED_POSITIONALLY) , diagnostic : BuiltinLintDiag :: NamedArgumentUsedPositionally { position_sp_to_replace , position_sp_for_msg , named_arg_sp : arg_name . span , named_arg_name : arg_name . name . to_string () , is_formatting_arg : matches ! (used_as , Width | Precision) , } . into () , }) ; } } ExpandResult :: Ready (Ok (FormatArgs { span : fmt_span , template , arguments : args , uncooked_fmt_str , is_source_literal , })) }
}

macro_rules! invalid_placeholder_type_error_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function invalid_placeholder_type_error in module {}", module_path!());
    };
}

mkfn!{
    invalid_placeholder_type_error_introspect!();
    fn invalid_placeholder_type_error (ecx : & ExtCtxt < '_ > , ty : & str , ty_span : Option < Range < usize > > , fmt_span : Span ,) { let sp = ty_span . map (| sp | fmt_span . from_inner (InnerSpan :: new (sp . start , sp . end))) ; let suggs = if let Some (sp) = sp { [("" , "Display") , ("?" , "Debug") , ("e" , "LowerExp") , ("E" , "UpperExp") , ("o" , "Octal") , ("p" , "Pointer") , ("b" , "Binary") , ("x" , "LowerHex") , ("X" , "UpperHex") ,] . into_iter () . map (| (fmt , trait_name) | errors :: FormatUnknownTraitSugg { span : sp , fmt , trait_name }) . collect () } else { vec ! [] } ; ecx . dcx () . emit_err (errors :: FormatUnknownTrait { span : sp . unwrap_or (fmt_span) , ty , suggs }) ; }
}

macro_rules! report_missing_placeholders_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function report_missing_placeholders in module {}", module_path!());
    };
}

mkfn!{
    report_missing_placeholders_introspect!();
    fn report_missing_placeholders (ecx : & ExtCtxt < '_ > , unused : Vec < (Span , bool) > , used : & [bool] , args : & FormatArguments , pieces : & [parse :: Piece < '_ >] , invalid_refs : & [(usize , Option < Span > , PositionUsedAs , FormatArgPositionKind)] , detect_foreign_fmt : bool , str_style : Option < usize > , fmt_str : & str , uncooked_fmt_str : & str , fmt_span : Span ,) { let mut diag = if let & [(span , named)] = & unused [..] { ecx . dcx () . create_err (errors :: FormatUnusedArg { span , named }) } else { let unused_labels = unused . iter () . map (| & (span , named) | errors :: FormatUnusedArg { span , named }) . collect () ; let unused_spans = unused . iter () . map (| & (span , _) | span) . collect () ; ecx . dcx () . create_err (errors :: FormatUnusedArgs { fmt : fmt_span , unused : unused_spans , unused_labels , }) } ; let placeholders = pieces . iter () . filter_map (| piece | { if let parse :: Piece :: NextArgument (argument) = piece && let ArgumentNamed (binding) = argument . position { let span = fmt_span . from_inner (InnerSpan :: new (argument . position_span . start , argument . position_span . end ,)) ; Some ((span , binding)) } else { None } }) . collect :: < Vec < _ > > () ; if ! placeholders . is_empty () { if let Some (new_diag) = report_redundant_format_arguments (ecx , args , used , placeholders) { diag . cancel () ; new_diag . emit () ; return ; } } let mut found_foreign = false ; if detect_foreign_fmt { use super :: format_foreign as foreign ; let mut explained = FxHashSet :: default () ; macro_rules ! check_foreign { ($ kind : ident) => { { let mut show_doc_note = false ; let mut suggestions = vec ! [] ; let padding = str_style . map (| i | i + 2) . unwrap_or (1) ; for sub in foreign ::$ kind :: iter_subs (fmt_str , padding) { let (trn , success) = match sub . translate () { Ok (trn) => (trn , true) , Err (Some (msg)) => (msg , false) , _ => continue , } ; let pos = sub . position () ; if ! explained . insert (sub . to_string ()) { continue ; } if ! found_foreign { found_foreign = true ; show_doc_note = true ; } let sp = fmt_span . from_inner (pos) ; if success { suggestions . push ((sp , trn)) ; } else { diag . span_note (sp , format ! ("format specifiers use curly braces, and {}" , trn) ,) ; } } if show_doc_note { diag . note (concat ! (stringify ! ($ kind) , " formatting is not supported; see the documentation for `std::fmt`" ,)) ; } if suggestions . len () > 0 { diag . multipart_suggestion ("format specifiers use curly braces" , suggestions , Applicability :: MachineApplicable ,) ; } } } ; } check_foreign ! (printf) ; if ! found_foreign { check_foreign ! (shell) ; } } if ! found_foreign && unused . len () == 1 { diag . span_label (fmt_span , "formatting specifier missing") ; } if ! found_foreign && invalid_refs . is_empty () { let show_example = used . iter () . all (| used | ! used) ; if ! show_example { if unused . len () > 1 { diag . note (format ! ("consider adding {} format specifiers" , unused . len ())) ; } } else { let msg = if unused . len () == 1 { "a format specifier" . to_string () } else { format ! ("{} format specifiers" , unused . len ()) } ; let sugg = match str_style { None => format ! ("\"{}{}\"" , uncooked_fmt_str , "{}" . repeat (unused . len ())) , Some (n_hashes) => format ! ("r{hashes}\"{uncooked_fmt_str}{fmt_specifiers}\"{hashes}" , hashes = "#" . repeat (n_hashes) , fmt_specifiers = "{}" . repeat (unused . len ())) , } ; let msg = format ! ("format specifiers use curly braces, consider adding {msg}") ; diag . span_suggestion_verbose (fmt_span , msg , sugg , Applicability :: MaybeIncorrect) ; } } diag . emit () ; }
}

macro_rules! report_redundant_format_arguments_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function report_redundant_format_arguments in module {}", module_path!());
    };
}

mkfn!{
    report_redundant_format_arguments_introspect!();
    #[doc = " This function detects and reports unused format!() arguments that are"] #[doc = " redundant due to implicit captures (e.g. `format!(\"{x}\", x)`)."] fn report_redundant_format_arguments < 'a > (ecx : & ExtCtxt < 'a > , args : & FormatArguments , used : & [bool] , placeholders : Vec < (Span , & str) > ,) -> Option < Diag < 'a > > { let mut fmt_arg_indices = vec ! [] ; let mut args_spans = vec ! [] ; let mut fmt_spans = vec ! [] ; for (i , unnamed_arg) in args . unnamed_args () . iter () . enumerate () . rev () { let Some (ty) = unnamed_arg . expr . to_ty () else { continue } ; let Some (argument_binding) = ty . kind . is_simple_path () else { continue } ; let argument_binding = argument_binding . as_str () ; if used [i] { continue ; } let matching_placeholders = placeholders . iter () . filter (| (_ , inline_binding) | argument_binding == * inline_binding) . map (| (span , _) | span) . collect :: < Vec < _ > > () ; if ! matching_placeholders . is_empty () { fmt_arg_indices . push (i) ; args_spans . push (unnamed_arg . expr . span) ; for span in & matching_placeholders { if fmt_spans . contains (* span) { continue ; } fmt_spans . push (* * span) ; } } } if ! args_spans . is_empty () { let multispan = MultiSpan :: from (fmt_spans) ; let mut suggestion_spans = vec ! [] ; for (arg_span , fmt_arg_idx) in args_spans . iter () . zip (fmt_arg_indices . iter ()) { let span = if fmt_arg_idx + 1 == args . explicit_args () . len () { * arg_span } else { arg_span . until (args . explicit_args () [* fmt_arg_idx + 1] . expr . span) } ; suggestion_spans . push (span) ; } let sugg = if args . named_args () . len () == 0 { Some (errors :: FormatRedundantArgsSugg { spans : suggestion_spans }) } else { None } ; return Some (ecx . dcx () . create_err (errors :: FormatRedundantArgs { n : args_spans . len () , span : MultiSpan :: from (args_spans) , note : multispan , sugg , })) ; } None }
}

macro_rules! report_invalid_references_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function report_invalid_references in module {}", module_path!());
    };
}

mkfn!{
    report_invalid_references_introspect!();
    #[doc = " Handle invalid references to positional arguments. Output different"] #[doc = " errors for the case where all arguments are positional and for when"] #[doc = " there are named arguments or numbered positional arguments in the"] #[doc = " format string."] fn report_invalid_references (ecx : & ExtCtxt < '_ > , invalid_refs : & [(usize , Option < Span > , PositionUsedAs , FormatArgPositionKind)] , template : & [FormatArgsPiece] , fmt_span : Span , args : & FormatArguments , parser : parse :: Parser < '_ > ,) { let num_args_desc = match args . explicit_args () . len () { 0 => "no arguments were given" . to_string () , 1 => "there is 1 argument" . to_string () , n => format ! ("there are {n} arguments") , } ; let mut e ; if template . iter () . all (| piece | match piece { FormatArgsPiece :: Placeholder (FormatPlaceholder { argument : FormatArgPosition { kind : FormatArgPositionKind :: Number , .. } , .. }) => false , FormatArgsPiece :: Placeholder (FormatPlaceholder { format_options : FormatOptions { precision : Some (FormatCount :: Argument (FormatArgPosition { kind : FormatArgPositionKind :: Number , .. })) , .. } | FormatOptions { width : Some (FormatCount :: Argument (FormatArgPosition { kind : FormatArgPositionKind :: Number , .. })) , .. } , .. }) => false , _ => true , }) { let mut spans = Vec :: new () ; let mut num_placeholders = 0 ; for piece in template { let mut placeholder = None ; if let FormatArgsPiece :: Placeholder (FormatPlaceholder { format_options : FormatOptions { precision : Some (FormatCount :: Argument (FormatArgPosition { span , kind : FormatArgPositionKind :: Implicit , .. })) , .. } , .. }) = piece { placeholder = * span ; num_placeholders += 1 ; } if let FormatArgsPiece :: Placeholder (FormatPlaceholder { argument : FormatArgPosition { kind : FormatArgPositionKind :: Implicit , .. } , span , .. }) = piece { placeholder = * span ; num_placeholders += 1 ; } spans . extend (placeholder) ; } let span = if spans . is_empty () { MultiSpan :: from_span (fmt_span) } else { MultiSpan :: from_spans (spans) } ; e = ecx . dcx () . create_err (errors :: FormatPositionalMismatch { span , n : num_placeholders , desc : num_args_desc , highlight : SingleLabelManySpans { spans : args . explicit_args () . iter () . map (| arg | arg . expr . span) . collect () , label : "" , } , }) ; let mut has_precision_star = false ; for piece in template { if let FormatArgsPiece :: Placeholder (FormatPlaceholder { format_options : FormatOptions { precision : Some (FormatCount :: Argument (FormatArgPosition { index , span : Some (span) , kind : FormatArgPositionKind :: Implicit , .. })) , .. } , .. }) = piece { let (Ok (index) | Err (index)) = index ; has_precision_star = true ; e . span_label (* span , format ! ("this precision flag adds an extra required argument at position {}, which is why there {} expected" , index , if num_placeholders == 1 { "is 1 argument" . to_string () } else { format ! ("are {num_placeholders} arguments") } ,) ,) ; } } if has_precision_star { e . note ("positional arguments are zero-based") ; } } else { let mut indexes : Vec < _ > = invalid_refs . iter () . map (| & (index , _ , _ , _) | index) . collect () ; indexes . sort () ; indexes . dedup () ; let span : MultiSpan = if ! parser . is_source_literal || parser . arg_places . is_empty () { MultiSpan :: from_span (fmt_span) } else { MultiSpan :: from_spans (invalid_refs . iter () . filter_map (| & (_ , span , _ , _) | span) . collect ()) } ; let arg_list = format ! ("argument{} {}" , pluralize ! (indexes . len ()) , listify (& indexes , | i : & usize | i . to_string ()) . unwrap_or_default ()) ; e = ecx . dcx () . struct_span_err (span , format ! ("invalid reference to positional {arg_list} ({num_args_desc})") ,) ; e . note ("positional arguments are zero-based") ; } if template . iter () . any (| piece | match piece { FormatArgsPiece :: Placeholder (FormatPlaceholder { format_options : f , .. }) => { * f != FormatOptions :: default () } _ => false , }) { e . note ("for information about formatting flags, visit https://doc.rust-lang.org/std/fmt/index.html") ; } e . emit () ; }
}

macro_rules! expand_format_args_impl_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function expand_format_args_impl in module {}", module_path!());
    };
}

mkfn!{
    expand_format_args_impl_introspect!();
    fn expand_format_args_impl < 'cx > (ecx : & 'cx mut ExtCtxt < '_ > , mut sp : Span , tts : TokenStream , nl : bool ,) -> MacroExpanderResult < 'cx > { sp = ecx . with_def_site_ctxt (sp) ; ExpandResult :: Ready (match parse_args (ecx , sp , tts) { Ok (input) => { let ExpandResult :: Ready (mac) = make_format_args (ecx , input , nl) else { return ExpandResult :: Retry (()) ; } ; match mac { Ok (format_args) => { MacEager :: expr (ecx . expr (sp , ExprKind :: FormatArgs (Box :: new (format_args)))) } Err (guar) => MacEager :: expr (DummyResult :: raw_expr (sp , Some (guar))) , } } Err (err) => { let guar = err . emit () ; DummyResult :: any (sp , guar) } }) }
}

macro_rules! expand_format_args_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function expand_format_args in module {}", module_path!());
    };
}

mkfn!{
    expand_format_args_introspect!();
    pub (crate) fn expand_format_args < 'cx > (ecx : & 'cx mut ExtCtxt < '_ > , sp : Span , tts : TokenStream ,) -> MacroExpanderResult < 'cx > { expand_format_args_impl (ecx , sp , tts , false) }
}

macro_rules! expand_format_args_nl_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function expand_format_args_nl in module {}", module_path!());
    };
}

mkfn!{
    expand_format_args_nl_introspect!();
    pub (crate) fn expand_format_args_nl < 'cx > (ecx : & 'cx mut ExtCtxt < '_ > , sp : Span , tts : TokenStream ,) -> MacroExpanderResult < 'cx > { expand_format_args_impl (ecx , sp , tts , true) }
}