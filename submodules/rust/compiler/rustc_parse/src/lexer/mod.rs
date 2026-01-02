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
mkuse!{use diagnostics :: make_errors_for_mismatched_closing_delims ;}
mkuse!{use rustc_ast :: ast :: { self , AttrStyle } ;}
mkuse!{use rustc_ast :: token :: { self , CommentKind , Delimiter , IdentIsRaw , Token , TokenKind } ;}
mkuse!{use rustc_ast :: tokenstream :: TokenStream ;}
mkuse!{use rustc_ast :: util :: unicode :: { TEXT_FLOW_CONTROL_CHARS , contains_text_flow_control_chars } ;}
mkuse!{use rustc_errors :: codes :: * ;}
mkuse!{use rustc_errors :: { Applicability , Diag , DiagCtxtHandle , StashKey } ;}
mkuse!{use rustc_lexer :: { Base , Cursor , DocStyle , FrontmatterAllowed , LiteralKind , RawStrError , is_horizontal_whitespace , } ;}
mkuse!{use rustc_literal_escaper :: { EscapeError , Mode , check_for_errors } ;}
mkuse!{use rustc_session :: lint :: BuiltinLintDiag ;}
mkuse!{use rustc_session :: lint :: builtin :: { RUST_2021_PREFIXES_INCOMPATIBLE_SYNTAX , RUST_2024_GUARDED_STRING_INCOMPATIBLE_SYNTAX , TEXT_DIRECTION_CODEPOINT_IN_COMMENT , TEXT_DIRECTION_CODEPOINT_IN_LITERAL , } ;}
mkuse!{use rustc_session :: parse :: ParseSess ;}
mkuse!{use rustc_span :: { BytePos , Pos , Span , Symbol , sym } ;}
mkuse!{use tracing :: debug ;}
mkuse!{use crate :: errors ;}
mkuse!{use crate :: lexer :: diagnostics :: TokenTreeDiagInfo ;}
mkuse!{use crate :: lexer :: unicode_chars :: UNICODE_ARRAY ;}
mkmod!{diagnostics, { 
                getname!(diagnostics);
                getsrc!(diagnostics);
                getpath!(diagnostics);
                get_deps!(diagnostics);
                get_crates!(diagnostics);
                mkinclude!(diagnostics);
                 
            }}
mkmod!{tokentrees, { 
                getname!(tokentrees);
                getsrc!(tokentrees);
                getpath!(tokentrees);
                get_deps!(tokentrees);
                get_crates!(tokentrees);
                mkinclude!(tokentrees);
                 
            }}
mkmod!{unescape_error_reporting, { 
                getname!(unescape_error_reporting);
                getsrc!(unescape_error_reporting);
                getpath!(unescape_error_reporting);
                get_deps!(unescape_error_reporting);
                get_crates!(unescape_error_reporting);
                mkinclude!(unescape_error_reporting);
                 
            }}
mkmod!{unicode_chars, { 
                getname!(unicode_chars);
                getsrc!(unicode_chars);
                getpath!(unicode_chars);
                get_deps!(unicode_chars);
                get_crates!(unicode_chars);
                mkinclude!(unicode_chars);
                 
            }}
mkuse!{use unescape_error_reporting :: { emit_unescape_error , escaped_char } ;}
mkitem!{#[cfg (target_pointer_width = "64")] rustc_data_structures :: static_assert_size ! (rustc_lexer :: Token , 12) ;}
mkitem!{mkstruct!{#[derive (Clone , Debug)] pub (crate) struct UnmatchedDelim { pub found_delim : Option < Delimiter > , pub found_span : Span , pub unclosed_span : Option < Span > , pub candidate_span : Option < Span > , }}}
mkitem!{mkenum!{#[doc = " Which tokens should be stripped before lexing the tokens."] pub enum StripTokens { #[doc = " Strip both shebang and frontmatter."] ShebangAndFrontmatter , #[doc = " Strip the shebang but not frontmatter."] #[doc = ""] #[doc = " That means that char sequences looking like frontmatter are simply"] #[doc = " interpreted as regular Rust lexemes."] Shebang , #[doc = " Strip nothing."] #[doc = ""] #[doc = " In other words, char sequences looking like a shebang or frontmatter"] #[doc = " are simply interpreted as regular Rust lexemes."] Nothing , }}}

macro_rules! lex_token_trees_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lex_token_trees in module {}", module_path!());
    };
}

mkfn!{
    lex_token_trees_introspect!();
    pub (crate) fn lex_token_trees < 'psess , 'src > (psess : & 'psess ParseSess , mut src : & 'src str , mut start_pos : BytePos , override_span : Option < Span > , strip_tokens : StripTokens ,) -> Result < TokenStream , Vec < Diag < 'psess > > > { match strip_tokens { StripTokens :: Shebang | StripTokens :: ShebangAndFrontmatter => { if let Some (shebang_len) = rustc_lexer :: strip_shebang (src) { src = & src [shebang_len ..] ; start_pos = start_pos + BytePos :: from_usize (shebang_len) ; } } StripTokens :: Nothing => { } } let frontmatter_allowed = match strip_tokens { StripTokens :: ShebangAndFrontmatter => FrontmatterAllowed :: Yes , StripTokens :: Shebang | StripTokens :: Nothing => FrontmatterAllowed :: No , } ; let cursor = Cursor :: new (src , frontmatter_allowed) ; let mut lexer = Lexer { psess , start_pos , pos : start_pos , src , cursor , override_span , nbsp_is_whitespace : false , last_lifetime : None , token : Token :: dummy () , diag_info : TokenTreeDiagInfo :: default () , } ; let res = lexer . lex_token_trees (false) ; let mut unmatched_closing_delims : Vec < _ > = make_errors_for_mismatched_closing_delims (& lexer . diag_info . unmatched_delims , psess) ; match res { Ok ((_open_spacing , stream)) => { if unmatched_closing_delims . is_empty () { Ok (stream) } else { Err (unmatched_closing_delims) } } Err (errs) => { unmatched_closing_delims . extend (errs) ; Err (unmatched_closing_delims) } } }
}
mkitem!{mkstruct!{struct Lexer < 'psess , 'src > { psess : & 'psess ParseSess , #[doc = " Initial position, read-only."] start_pos : BytePos , #[doc = " The absolute offset within the source_map of the current character."] pos : BytePos , #[doc = " Source text to tokenize."] src : & 'src str , #[doc = " Cursor for getting lexer tokens."] cursor : Cursor < 'src > , override_span : Option < Span > , #[doc = " When a \"unknown start of token: \\u{a0}\" has already been emitted earlier"] #[doc = " in this file, it's safe to treat further occurrences of the non-breaking"] #[doc = " space character as whitespace."] nbsp_is_whitespace : bool , #[doc = " Track the `Span` for the leading `'` of the last lifetime. Used for"] #[doc = " diagnostics to detect possible typo where `\"` was meant."] last_lifetime : Option < Span > , #[doc = " The current token."] token : Token , diag_info : TokenTreeDiagInfo , }}}
mkitem!{mkimpl!{impl < 'psess , 'src > Lexer < 'psess , 'src > { fn dcx (& self) -> DiagCtxtHandle < 'psess > { self . psess . dcx () } fn mk_sp (& self , lo : BytePos , hi : BytePos) -> Span { self . override_span . unwrap_or_else (| | Span :: with_root_ctxt (lo , hi)) } #[doc = " Returns the next token, paired with a bool indicating if the token was"] #[doc = " preceded by whitespace."] fn next_token_from_cursor (& mut self) -> (Token , bool) { let mut preceded_by_whitespace = false ; let mut swallow_next_invalid = 0 ; loop { let str_before = self . cursor . as_str () ; let token = self . cursor . advance_token () ; let start = self . pos ; self . pos = self . pos + BytePos (token . len) ; debug ! ("next_token: {:?}({:?})" , token . kind , self . str_from (start)) ; if let rustc_lexer :: TokenKind :: Semi | rustc_lexer :: TokenKind :: LineComment { .. } | rustc_lexer :: TokenKind :: BlockComment { .. } | rustc_lexer :: TokenKind :: CloseParen | rustc_lexer :: TokenKind :: CloseBrace | rustc_lexer :: TokenKind :: CloseBracket = token . kind { self . last_lifetime = None ; } let kind = match token . kind { rustc_lexer :: TokenKind :: LineComment { doc_style } => { let Some (doc_style) = doc_style else { self . lint_unicode_text_flow (start) ; preceded_by_whitespace = true ; continue ; } ; let content_start = start + BytePos (3) ; let content = self . str_from (content_start) ; self . lint_doc_comment_unicode_text_flow (start , content) ; self . cook_doc_comment (content_start , content , CommentKind :: Line , doc_style) } rustc_lexer :: TokenKind :: BlockComment { doc_style , terminated } => { if ! terminated { self . report_unterminated_block_comment (start , doc_style) ; } let Some (doc_style) = doc_style else { self . lint_unicode_text_flow (start) ; preceded_by_whitespace = true ; continue ; } ; let content_start = start + BytePos (3) ; let content_end = self . pos - BytePos (if terminated { 2 } else { 0 }) ; let content = self . str_from_to (content_start , content_end) ; self . lint_doc_comment_unicode_text_flow (start , content) ; self . cook_doc_comment (content_start , content , CommentKind :: Block , doc_style) } rustc_lexer :: TokenKind :: Frontmatter { has_invalid_preceding_whitespace , invalid_infostring } => { self . validate_frontmatter (start , has_invalid_preceding_whitespace , invalid_infostring) ; preceded_by_whitespace = true ; continue ; } rustc_lexer :: TokenKind :: Whitespace => { preceded_by_whitespace = true ; continue ; } rustc_lexer :: TokenKind :: Ident => self . ident (start) , rustc_lexer :: TokenKind :: RawIdent => { let sym = nfc_normalize (self . str_from (start + BytePos (2))) ; let span = self . mk_sp (start , self . pos) ; self . psess . symbol_gallery . insert (sym , span) ; if ! sym . can_be_raw () { self . dcx () . emit_err (errors :: CannotBeRawIdent { span , ident : sym }) ; } self . psess . raw_identifier_spans . push (span) ; token :: Ident (sym , IdentIsRaw :: Yes) } rustc_lexer :: TokenKind :: UnknownPrefix => { self . report_unknown_prefix (start) ; self . ident (start) } rustc_lexer :: TokenKind :: UnknownPrefixLifetime => { self . report_unknown_prefix (start) ; let lifetime_name = self . str_from (start) ; self . last_lifetime = Some (self . mk_sp (start , start + BytePos (1))) ; let ident = Symbol :: intern (lifetime_name) ; token :: Lifetime (ident , IdentIsRaw :: No) } rustc_lexer :: TokenKind :: InvalidIdent if ! UNICODE_ARRAY . iter () . any (| & (c , _ , _) | { let sym = self . str_from (start) ; sym . chars () . count () == 1 && c == sym . chars () . next () . unwrap () }) => { let sym = nfc_normalize (self . str_from (start)) ; let span = self . mk_sp (start , self . pos) ; self . psess . bad_unicode_identifiers . borrow_mut () . entry (sym) . or_default () . push (span) ; token :: Ident (sym , IdentIsRaw :: No) } rustc_lexer :: TokenKind :: Literal { kind : kind @ (LiteralKind :: CStr { .. } | LiteralKind :: RawCStr { .. }) , suffix_start : _ , } if ! self . mk_sp (start , self . pos) . edition () . at_least_rust_2021 () => { let prefix_len = match kind { LiteralKind :: CStr { .. } => 1 , LiteralKind :: RawCStr { .. } => 2 , _ => unreachable ! () , } ; let lit_start = start + BytePos (prefix_len) ; self . pos = lit_start ; self . cursor = Cursor :: new (& str_before [prefix_len as usize ..] , FrontmatterAllowed :: No) ; self . report_unknown_prefix (start) ; let prefix_span = self . mk_sp (start , lit_start) ; return (Token :: new (self . ident (start) , prefix_span) , preceded_by_whitespace) ; } rustc_lexer :: TokenKind :: GuardedStrPrefix => { self . maybe_report_guarded_str (start , str_before) } rustc_lexer :: TokenKind :: Literal { kind , suffix_start } => { let suffix_start = start + BytePos (suffix_start) ; let (kind , symbol) = self . cook_lexer_literal (start , suffix_start , kind) ; let suffix = if suffix_start < self . pos { let string = self . str_from (suffix_start) ; if string == "_" { self . dcx () . emit_err (errors :: UnderscoreLiteralSuffix { span : self . mk_sp (suffix_start , self . pos) , }) ; None } else { Some (Symbol :: intern (string)) } } else { None } ; self . lint_literal_unicode_text_flow (symbol , kind , self . mk_sp (start , self . pos) , "literal") ; token :: Literal (token :: Lit { kind , symbol , suffix }) } rustc_lexer :: TokenKind :: Lifetime { starts_with_number } => { let lifetime_name = self . str_from (start) ; self . last_lifetime = Some (self . mk_sp (start , start + BytePos (1))) ; if starts_with_number { let span = self . mk_sp (start , self . pos) ; self . dcx () . struct_err ("lifetimes cannot start with a number") . with_span (span) . stash (span , StashKey :: LifetimeIsChar) ; } let ident = Symbol :: intern (lifetime_name) ; token :: Lifetime (ident , IdentIsRaw :: No) } rustc_lexer :: TokenKind :: RawLifetime => { self . last_lifetime = Some (self . mk_sp (start , start + BytePos (1))) ; let ident_start = start + BytePos (3) ; let prefix_span = self . mk_sp (start , ident_start) ; if prefix_span . at_least_rust_2021 () { if self . cursor . as_str () . starts_with ('\'') { let lit_span = self . mk_sp (start , self . pos + BytePos (1)) ; let contents = self . str_from_to (start + BytePos (1) , self . pos) ; emit_unescape_error (self . dcx () , contents , lit_span , lit_span , Mode :: Char , 0 .. contents . len () , EscapeError :: MoreThanOneChar ,) . expect ("expected error") ; } let span = self . mk_sp (start , self . pos) ; let lifetime_name_without_tick = Symbol :: intern (& self . str_from (ident_start)) ; if ! lifetime_name_without_tick . can_be_raw () { self . dcx () . emit_err (errors :: CannotBeRawLifetime { span , ident : lifetime_name_without_tick }) ; } let mut lifetime_name = String :: with_capacity (lifetime_name_without_tick . as_str () . len () + 1) ; lifetime_name . push ('\'') ; lifetime_name += lifetime_name_without_tick . as_str () ; let sym = Symbol :: intern (& lifetime_name) ; self . psess . raw_identifier_spans . push (span) ; token :: Lifetime (sym , IdentIsRaw :: Yes) } else { self . psess . buffer_lint (RUST_2021_PREFIXES_INCOMPATIBLE_SYNTAX , prefix_span , ast :: CRATE_NODE_ID , BuiltinLintDiag :: RawPrefix (prefix_span) ,) ; let lt_start = start + BytePos (2) ; self . pos = lt_start ; self . cursor = Cursor :: new (& str_before [2 as usize ..] , FrontmatterAllowed :: No) ; let lifetime_name = self . str_from (start) ; let ident = Symbol :: intern (lifetime_name) ; token :: Lifetime (ident , IdentIsRaw :: No) } } rustc_lexer :: TokenKind :: Semi => token :: Semi , rustc_lexer :: TokenKind :: Comma => token :: Comma , rustc_lexer :: TokenKind :: Dot => token :: Dot , rustc_lexer :: TokenKind :: OpenParen => token :: OpenParen , rustc_lexer :: TokenKind :: CloseParen => token :: CloseParen , rustc_lexer :: TokenKind :: OpenBrace => token :: OpenBrace , rustc_lexer :: TokenKind :: CloseBrace => token :: CloseBrace , rustc_lexer :: TokenKind :: OpenBracket => token :: OpenBracket , rustc_lexer :: TokenKind :: CloseBracket => token :: CloseBracket , rustc_lexer :: TokenKind :: At => token :: At , rustc_lexer :: TokenKind :: Pound => token :: Pound , rustc_lexer :: TokenKind :: Tilde => token :: Tilde , rustc_lexer :: TokenKind :: Question => token :: Question , rustc_lexer :: TokenKind :: Colon => token :: Colon , rustc_lexer :: TokenKind :: Dollar => token :: Dollar , rustc_lexer :: TokenKind :: Eq => token :: Eq , rustc_lexer :: TokenKind :: Bang => token :: Bang , rustc_lexer :: TokenKind :: Lt => token :: Lt , rustc_lexer :: TokenKind :: Gt => token :: Gt , rustc_lexer :: TokenKind :: Minus => token :: Minus , rustc_lexer :: TokenKind :: And => token :: And , rustc_lexer :: TokenKind :: Or => token :: Or , rustc_lexer :: TokenKind :: Plus => token :: Plus , rustc_lexer :: TokenKind :: Star => token :: Star , rustc_lexer :: TokenKind :: Slash => token :: Slash , rustc_lexer :: TokenKind :: Caret => token :: Caret , rustc_lexer :: TokenKind :: Percent => token :: Percent , rustc_lexer :: TokenKind :: Unknown | rustc_lexer :: TokenKind :: InvalidIdent => { if swallow_next_invalid > 0 { swallow_next_invalid -= 1 ; continue ; } let mut it = self . str_from_to_end (start) . chars () ; let c = it . next () . unwrap () ; if c == '\u{00a0}' { if self . nbsp_is_whitespace { preceded_by_whitespace = true ; continue ; } self . nbsp_is_whitespace = true ; } let repeats = it . take_while (| c1 | * c1 == c) . count () ; let (token , sugg) = unicode_chars :: check_for_substitution (self , start , c , repeats + 1) ; self . dcx () . emit_err (errors :: UnknownTokenStart { span : self . mk_sp (start , self . pos + Pos :: from_usize (repeats * c . len_utf8 ())) , escaped : escaped_char (c) , sugg , null : if c == '\x00' { Some (errors :: UnknownTokenNull) } else { None } , repeat : if repeats > 0 { swallow_next_invalid = repeats ; Some (errors :: UnknownTokenRepeat { repeats }) } else { None } , }) ; if let Some (token) = token { token } else { preceded_by_whitespace = true ; continue ; } } rustc_lexer :: TokenKind :: Eof => token :: Eof , } ; let span = self . mk_sp (start , self . pos) ; return (Token :: new (kind , span) , preceded_by_whitespace) ; } } fn ident (& self , start : BytePos) -> TokenKind { let sym = nfc_normalize (self . str_from (start)) ; let span = self . mk_sp (start , self . pos) ; self . psess . symbol_gallery . insert (sym , span) ; token :: Ident (sym , IdentIsRaw :: No) } #[doc = " Detect usages of Unicode codepoints changing the direction of the text on screen and loudly"] #[doc = " complain about it."] fn lint_unicode_text_flow (& self , start : BytePos) { let content_start = start + BytePos (2) ; let content = self . str_from (content_start) ; if contains_text_flow_control_chars (content) { let span = self . mk_sp (start , self . pos) ; self . psess . buffer_lint (TEXT_DIRECTION_CODEPOINT_IN_COMMENT , span , ast :: CRATE_NODE_ID , BuiltinLintDiag :: UnicodeTextFlow (span , content . to_string ()) ,) ; } } fn lint_doc_comment_unicode_text_flow (& mut self , start : BytePos , content : & str) { if contains_text_flow_control_chars (content) { self . report_text_direction_codepoint (content , self . mk_sp (start , self . pos) , 0 , false , "doc comment" ,) ; } } fn lint_literal_unicode_text_flow (& mut self , text : Symbol , lit_kind : token :: LitKind , span : Span , label : & 'static str ,) { if ! contains_text_flow_control_chars (text . as_str ()) { return ; } let (padding , point_at_inner_spans) = match lit_kind { token :: LitKind :: Str | token :: LitKind :: Char => (1 , true) , token :: LitKind :: CStr => (2 , true) , token :: LitKind :: StrRaw (n) => (n as u32 + 2 , true) , token :: LitKind :: CStrRaw (n) => (n as u32 + 3 , true) , token :: LitKind :: Err (_) => return , _ => (0 , false) , } ; self . report_text_direction_codepoint (text . as_str () , span , padding , point_at_inner_spans , label ,) ; } fn report_text_direction_codepoint (& self , text : & str , span : Span , padding : u32 , point_at_inner_spans : bool , label : & str ,) { let spans : Vec < _ > = text . char_indices () . filter_map (| (i , c) | { TEXT_FLOW_CONTROL_CHARS . contains (& c) . then (| | { let lo = span . lo () + BytePos (i as u32 + padding) ; (c , span . with_lo (lo) . with_hi (lo + BytePos (c . len_utf8 () as u32))) }) }) . collect () ; let label = label . to_string () ; let count = spans . len () ; let labels = point_at_inner_spans . then_some (errors :: HiddenUnicodeCodepointsDiagLabels { spans : spans . clone () }) ; let sub = if point_at_inner_spans && ! spans . is_empty () { errors :: HiddenUnicodeCodepointsDiagSub :: Escape { spans } } else { errors :: HiddenUnicodeCodepointsDiagSub :: NoEscape { spans } } ; self . psess . buffer_lint (TEXT_DIRECTION_CODEPOINT_IN_LITERAL , span , ast :: CRATE_NODE_ID , errors :: HiddenUnicodeCodepointsDiag { label , count , span_label : span , labels , sub } ,) ; } fn validate_frontmatter (& self , start : BytePos , has_invalid_preceding_whitespace : bool , invalid_infostring : bool ,) { let s = self . str_from (start) ; let real_start = s . find ("---") . unwrap () ; let frontmatter_opening_pos = BytePos (real_start as u32) + start ; let s_new = & s [real_start ..] ; let within = s_new . trim_start_matches ('-') ; let len_opening = s_new . len () - within . len () ; let frontmatter_opening_end_pos = frontmatter_opening_pos + BytePos (len_opening as u32) ; if has_invalid_preceding_whitespace { let line_start = BytePos (s [.. real_start] . rfind ("\n") . map_or (0 , | i | i as u32 + 1)) + start ; let span = self . mk_sp (line_start , frontmatter_opening_end_pos) ; let label_span = self . mk_sp (line_start , frontmatter_opening_pos) ; self . dcx () . emit_err (errors :: FrontmatterInvalidOpeningPrecedingWhitespace { span , note_span : label_span , }) ; } if invalid_infostring { let line_end = s [real_start ..] . find ('\n') . unwrap_or (s [real_start ..] . len ()) ; let span = self . mk_sp (frontmatter_opening_end_pos , frontmatter_opening_pos + BytePos (line_end as u32) ,) ; self . dcx () . emit_err (errors :: FrontmatterInvalidInfostring { span }) ; } let last_line_start = within . rfind ('\n') . map_or (0 , | i | i + 1) ; let last_line = & within [last_line_start ..] ; let last_line_trimmed = last_line . trim_start_matches (is_horizontal_whitespace) ; let last_line_start_pos = frontmatter_opening_end_pos + BytePos (last_line_start as u32) ; let frontmatter_span = self . mk_sp (frontmatter_opening_pos , self . pos) ; self . psess . gated_spans . gate (sym :: frontmatter , frontmatter_span) ; if ! last_line_trimmed . starts_with ("---") { let label_span = self . mk_sp (frontmatter_opening_pos , frontmatter_opening_end_pos) ; self . dcx () . emit_err (errors :: FrontmatterUnclosed { span : frontmatter_span , note_span : label_span , }) ; return ; } if last_line_trimmed . len () != last_line . len () { let line_end = last_line_start_pos + BytePos (last_line . len () as u32) ; let span = self . mk_sp (last_line_start_pos , line_end) ; let whitespace_end = last_line_start_pos + BytePos ((last_line . len () - last_line_trimmed . len ()) as u32) ; let label_span = self . mk_sp (last_line_start_pos , whitespace_end) ; self . dcx () . emit_err (errors :: FrontmatterInvalidClosingPrecedingWhitespace { span , note_span : label_span , }) ; } let rest = last_line_trimmed . trim_start_matches ('-') ; let len_close = last_line_trimmed . len () - rest . len () ; if len_close != len_opening { let span = self . mk_sp (frontmatter_opening_pos , self . pos) ; let opening = self . mk_sp (frontmatter_opening_pos , frontmatter_opening_end_pos) ; let last_line_close_pos = last_line_start_pos + BytePos (len_close as u32) ; let close = self . mk_sp (last_line_start_pos , last_line_close_pos) ; self . dcx () . emit_err (errors :: FrontmatterLengthMismatch { span , opening , close , len_opening , len_close , }) ; } if ! rest . trim_matches (is_horizontal_whitespace) . is_empty () { let span = self . mk_sp (last_line_start_pos , self . pos) ; self . dcx () . emit_err (errors :: FrontmatterExtraCharactersAfterClose { span }) ; } } fn cook_doc_comment (& self , content_start : BytePos , content : & str , comment_kind : CommentKind , doc_style : DocStyle ,) -> TokenKind { if content . contains ('\r') { for (idx , _) in content . char_indices () . filter (| & (_ , c) | c == '\r') { let span = self . mk_sp (content_start + BytePos (idx as u32) , content_start + BytePos (idx as u32 + 1) ,) ; let block = matches ! (comment_kind , CommentKind :: Block) ; self . dcx () . emit_err (errors :: CrDocComment { span , block }) ; } } let attr_style = match doc_style { DocStyle :: Outer => AttrStyle :: Outer , DocStyle :: Inner => AttrStyle :: Inner , } ; token :: DocComment (comment_kind , attr_style , Symbol :: intern (content)) } fn cook_lexer_literal (& self , start : BytePos , end : BytePos , kind : rustc_lexer :: LiteralKind ,) -> (token :: LitKind , Symbol) { match kind { rustc_lexer :: LiteralKind :: Char { terminated } => { if ! terminated { let mut err = self . dcx () . struct_span_fatal (self . mk_sp (start , end) , "unterminated character literal") . with_code (E0762) ; if let Some (lt_sp) = self . last_lifetime { err . multipart_suggestion ("if you meant to write a string literal, use double quotes" , vec ! [(lt_sp , "\"" . to_string ()) , (self . mk_sp (start , start + BytePos (1)) , "\"" . to_string ()) ,] , Applicability :: MaybeIncorrect ,) ; } err . emit () } self . cook_quoted (token :: Char , Mode :: Char , start , end , 1 , 1) } rustc_lexer :: LiteralKind :: Byte { terminated } => { if ! terminated { self . dcx () . struct_span_fatal (self . mk_sp (start + BytePos (1) , end) , "unterminated byte constant" ,) . with_code (E0763) . emit () } self . cook_quoted (token :: Byte , Mode :: Byte , start , end , 2 , 1) } rustc_lexer :: LiteralKind :: Str { terminated } => { if ! terminated { self . dcx () . struct_span_fatal (self . mk_sp (start , end) , "unterminated double quote string" ,) . with_code (E0765) . emit () } self . cook_quoted (token :: Str , Mode :: Str , start , end , 1 , 1) } rustc_lexer :: LiteralKind :: ByteStr { terminated } => { if ! terminated { self . dcx () . struct_span_fatal (self . mk_sp (start + BytePos (1) , end) , "unterminated double quote byte string" ,) . with_code (E0766) . emit () } self . cook_quoted (token :: ByteStr , Mode :: ByteStr , start , end , 2 , 1) } rustc_lexer :: LiteralKind :: CStr { terminated } => { if ! terminated { self . dcx () . struct_span_fatal (self . mk_sp (start + BytePos (1) , end) , "unterminated C string" ,) . with_code (E0767) . emit () } self . cook_quoted (token :: CStr , Mode :: CStr , start , end , 2 , 1) } rustc_lexer :: LiteralKind :: RawStr { n_hashes } => { if let Some (n_hashes) = n_hashes { let n = u32 :: from (n_hashes) ; let kind = token :: StrRaw (n_hashes) ; self . cook_quoted (kind , Mode :: RawStr , start , end , 2 + n , 1 + n) } else { self . report_raw_str_error (start , 1) ; } } rustc_lexer :: LiteralKind :: RawByteStr { n_hashes } => { if let Some (n_hashes) = n_hashes { let n = u32 :: from (n_hashes) ; let kind = token :: ByteStrRaw (n_hashes) ; self . cook_quoted (kind , Mode :: RawByteStr , start , end , 3 + n , 1 + n) } else { self . report_raw_str_error (start , 2) ; } } rustc_lexer :: LiteralKind :: RawCStr { n_hashes } => { if let Some (n_hashes) = n_hashes { let n = u32 :: from (n_hashes) ; let kind = token :: CStrRaw (n_hashes) ; self . cook_quoted (kind , Mode :: RawCStr , start , end , 3 + n , 1 + n) } else { self . report_raw_str_error (start , 2) ; } } rustc_lexer :: LiteralKind :: Int { base , empty_int } => { let mut kind = token :: Integer ; if empty_int { let span = self . mk_sp (start , end) ; let guar = self . dcx () . emit_err (errors :: NoDigitsLiteral { span }) ; kind = token :: Err (guar) ; } else if matches ! (base , Base :: Binary | Base :: Octal) { let base = base as u32 ; let s = self . str_from_to (start + BytePos (2) , end) ; for (idx , c) in s . char_indices () { let span = self . mk_sp (start + BytePos :: from_usize (2 + idx) , start + BytePos :: from_usize (2 + idx + c . len_utf8 ()) ,) ; if c != '_' && c . to_digit (base) . is_none () { let guar = self . dcx () . emit_err (errors :: InvalidDigitLiteral { span , base }) ; kind = token :: Err (guar) ; } } } (kind , self . symbol_from_to (start , end)) } rustc_lexer :: LiteralKind :: Float { base , empty_exponent } => { let mut kind = token :: Float ; if empty_exponent { let span = self . mk_sp (start , self . pos) ; let guar = self . dcx () . emit_err (errors :: EmptyExponentFloat { span }) ; kind = token :: Err (guar) ; } let base = match base { Base :: Hexadecimal => Some ("hexadecimal") , Base :: Octal => Some ("octal") , Base :: Binary => Some ("binary") , _ => None , } ; if let Some (base) = base { let span = self . mk_sp (start , end) ; let guar = self . dcx () . emit_err (errors :: FloatLiteralUnsupportedBase { span , base }) ; kind = token :: Err (guar) } (kind , self . symbol_from_to (start , end)) } } } #[inline] fn src_index (& self , pos : BytePos) -> usize { (pos - self . start_pos) . to_usize () } #[doc = " Slice of the source text from `start` up to but excluding `self.pos`,"] #[doc = " meaning the slice does not include the character `self.ch`."] fn str_from (& self , start : BytePos) -> & 'src str { self . str_from_to (start , self . pos) } #[doc = " As symbol_from, with an explicit endpoint."] fn symbol_from_to (& self , start : BytePos , end : BytePos) -> Symbol { debug ! ("taking an ident from {:?} to {:?}" , start , end) ; Symbol :: intern (self . str_from_to (start , end)) } #[doc = " Slice of the source text spanning from `start` up to but excluding `end`."] fn str_from_to (& self , start : BytePos , end : BytePos) -> & 'src str { & self . src [self . src_index (start) .. self . src_index (end)] } #[doc = " Slice of the source text spanning from `start` until the end"] fn str_from_to_end (& self , start : BytePos) -> & 'src str { & self . src [self . src_index (start) ..] } fn report_raw_str_error (& self , start : BytePos , prefix_len : u32) -> ! { match rustc_lexer :: validate_raw_str (self . str_from (start) , prefix_len) { Err (RawStrError :: InvalidStarter { bad_char }) => { self . report_non_started_raw_string (start , bad_char) } Err (RawStrError :: NoTerminator { expected , found , possible_terminator_offset }) => self . report_unterminated_raw_string (start , expected , possible_terminator_offset , found) , Err (RawStrError :: TooManyDelimiters { found }) => { self . report_too_many_hashes (start , found) } Ok (()) => panic ! ("no error found for supposedly invalid raw string literal") , } } fn report_non_started_raw_string (& self , start : BytePos , bad_char : char) -> ! { self . dcx () . struct_span_fatal (self . mk_sp (start , self . pos) , format ! ("found invalid character; only `#` is allowed in raw string delimitation: {}" , escaped_char (bad_char)) ,) . emit () } fn report_unterminated_raw_string (& self , start : BytePos , n_hashes : u32 , possible_offset : Option < u32 > , found_terminators : u32 ,) -> ! { let mut err = self . dcx () . struct_span_fatal (self . mk_sp (start , start) , "unterminated raw string") ; err . code (E0748) ; err . span_label (self . mk_sp (start , start) , "unterminated raw string") ; if n_hashes > 0 { err . note (format ! ("this raw string should be terminated with `\"{}`" , "#" . repeat (n_hashes as usize))) ; } if let Some (possible_offset) = possible_offset { let lo = start + BytePos (possible_offset) ; let hi = lo + BytePos (found_terminators) ; let span = self . mk_sp (lo , hi) ; err . span_suggestion (span , "consider terminating the string here" , "#" . repeat (n_hashes as usize) , Applicability :: MaybeIncorrect ,) ; } err . emit () } fn report_unterminated_block_comment (& self , start : BytePos , doc_style : Option < DocStyle >) { let msg = match doc_style { Some (_) => "unterminated block doc-comment" , None => "unterminated block comment" , } ; let last_bpos = self . pos ; let mut err = self . dcx () . struct_span_fatal (self . mk_sp (start , last_bpos) , msg) ; err . code (E0758) ; let mut nested_block_comment_open_idxs = vec ! [] ; let mut last_nested_block_comment_idxs = None ; let mut content_chars = self . str_from (start) . char_indices () . peekable () ; while let Some ((idx , current_char)) = content_chars . next () { match content_chars . peek () { Some ((_ , '*')) if current_char == '/' => { nested_block_comment_open_idxs . push (idx) ; } Some ((_ , '/')) if current_char == '*' => { last_nested_block_comment_idxs = nested_block_comment_open_idxs . pop () . map (| open_idx | (open_idx , idx)) ; } _ => { } } ; } if let Some ((nested_open_idx , nested_close_idx)) = last_nested_block_comment_idxs { err . span_label (self . mk_sp (start , start + BytePos (2)) , msg) . span_label (self . mk_sp (start + BytePos (nested_open_idx as u32) , start + BytePos (nested_open_idx as u32 + 2) ,) , "...as last nested comment starts here, maybe you want to close this instead?" ,) . span_label (self . mk_sp (start + BytePos (nested_close_idx as u32) , start + BytePos (nested_close_idx as u32 + 2) ,) , "...and last nested comment terminates here." ,) ; } err . emit () ; } fn report_unknown_prefix (& self , start : BytePos) { let prefix_span = self . mk_sp (start , self . pos) ; let prefix = self . str_from_to (start , self . pos) ; let expn_data = prefix_span . ctxt () . outer_expn_data () ; if expn_data . edition . at_least_rust_2021 () { let sugg = if prefix == "rb" { Some (errors :: UnknownPrefixSugg :: UseBr (prefix_span)) } else if prefix == "rc" { Some (errors :: UnknownPrefixSugg :: UseCr (prefix_span)) } else if expn_data . is_root () { if self . cursor . first () == '\'' && let Some (start) = self . last_lifetime && self . cursor . third () != '\'' && let end = self . mk_sp (self . pos , self . pos + BytePos (1)) && ! self . psess . source_map () . is_multiline (start . until (end)) { Some (errors :: UnknownPrefixSugg :: MeantStr { start , end }) } else { Some (errors :: UnknownPrefixSugg :: Whitespace (prefix_span . shrink_to_hi ())) } } else { None } ; self . dcx () . emit_err (errors :: UnknownPrefix { span : prefix_span , prefix , sugg }) ; } else { self . psess . buffer_lint (RUST_2021_PREFIXES_INCOMPATIBLE_SYNTAX , prefix_span , ast :: CRATE_NODE_ID , BuiltinLintDiag :: ReservedPrefix (prefix_span , prefix . to_string ()) ,) ; } } #[doc = " Detect guarded string literal syntax"] #[doc = ""] #[doc = " RFC 3593 reserved this syntax for future use. As of Rust 2024,"] #[doc = " using this syntax produces an error. In earlier editions, however, it"] #[doc = " only results in an (allowed by default) lint, and is treated as"] #[doc = " separate tokens."] fn maybe_report_guarded_str (& mut self , start : BytePos , str_before : & 'src str) -> TokenKind { let span = self . mk_sp (start , self . pos) ; let edition2024 = span . edition () . at_least_rust_2024 () ; let space_pos = start + BytePos (1) ; let space_span = self . mk_sp (space_pos , space_pos) ; let mut cursor = Cursor :: new (str_before , FrontmatterAllowed :: No) ; let (is_string , span , unterminated) = match cursor . guarded_double_quoted_string () { Some (rustc_lexer :: GuardedStr { n_hashes , terminated , token_len }) => { let end = start + BytePos (token_len) ; let span = self . mk_sp (start , end) ; let str_start = start + BytePos (n_hashes) ; if edition2024 { self . cursor = cursor ; self . pos = end ; } let unterminated = if terminated { None } else { Some (str_start) } ; (true , span , unterminated) } None => { debug_assert_eq ! (self . str_from_to (start , start + BytePos (2)) , "##") ; (false , span , None) } } ; if edition2024 { if let Some (str_start) = unterminated { self . dcx () . struct_span_fatal (self . mk_sp (str_start , self . pos) , "unterminated double quote string" ,) . with_code (E0765) . emit () } let sugg = if span . from_expansion () { None } else { Some (errors :: GuardedStringSugg (space_span)) } ; let err = if is_string { self . dcx () . emit_err (errors :: ReservedString { span , sugg }) } else { self . dcx () . emit_err (errors :: ReservedMultihash { span , sugg }) } ; token :: Literal (token :: Lit { kind : token :: Err (err) , symbol : self . symbol_from_to (start , self . pos) , suffix : None , }) } else { self . psess . buffer_lint (RUST_2024_GUARDED_STRING_INCOMPATIBLE_SYNTAX , span , ast :: CRATE_NODE_ID , BuiltinLintDiag :: ReservedString { is_string , suggestion : space_span } ,) ; self . pos = start + BytePos (1) ; self . cursor = Cursor :: new (& str_before [1 ..] , FrontmatterAllowed :: No) ; token :: Pound } } fn report_too_many_hashes (& self , start : BytePos , num : u32) -> ! { self . dcx () . emit_fatal (errors :: TooManyHashes { span : self . mk_sp (start , self . pos) , num }) ; } fn cook_quoted (& self , mut kind : token :: LitKind , mode : Mode , start : BytePos , end : BytePos , prefix_len : u32 , postfix_len : u32 ,) -> (token :: LitKind , Symbol) { let content_start = start + BytePos (prefix_len) ; let content_end = end - BytePos (postfix_len) ; let lit_content = self . str_from_to (content_start , content_end) ; check_for_errors (lit_content , mode , | range , err | { let span_with_quotes = self . mk_sp (start , end) ; let (start , end) = (range . start as u32 , range . end as u32) ; let lo = content_start + BytePos (start) ; let hi = lo + BytePos (end - start) ; let span = self . mk_sp (lo , hi) ; let is_fatal = err . is_fatal () ; if let Some (guar) = emit_unescape_error (self . dcx () , lit_content , span_with_quotes , span , mode , range , err ,) { assert ! (is_fatal) ; kind = token :: Err (guar) ; } }) ; let sym = if ! matches ! (kind , token :: Err (_)) { Symbol :: intern (lit_content) } else { self . symbol_from_to (start , end) } ; (kind , sym) } }}}

macro_rules! nfc_normalize_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function nfc_normalize in module {}", module_path!());
    };
}

mkfn!{
    nfc_normalize_introspect!();
    pub fn nfc_normalize (string : & str) -> Symbol { use unicode_normalization :: { IsNormalized , UnicodeNormalization , is_nfc_quick } ; match is_nfc_quick (string . chars ()) { IsNormalized :: Yes => Symbol :: intern (string) , _ => { let normalized_str : String = string . chars () . nfc () . collect () ; Symbol :: intern (& normalized_str) } } }
}