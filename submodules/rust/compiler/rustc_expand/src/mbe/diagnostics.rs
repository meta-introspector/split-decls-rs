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
mkuse!{use std :: borrow :: Cow ;}
mkuse!{use rustc_ast :: token :: { self , Token } ;}
mkuse!{use rustc_ast :: tokenstream :: TokenStream ;}
mkuse!{use rustc_errors :: { Applicability , Diag , DiagCtxtHandle , DiagMessage } ;}
mkuse!{use rustc_macros :: Subdiagnostic ;}
mkuse!{use rustc_parse :: parser :: { Parser , Recovery , token_descr } ;}
mkuse!{use rustc_session :: parse :: ParseSess ;}
mkuse!{use rustc_span :: source_map :: SourceMap ;}
mkuse!{use rustc_span :: { DUMMY_SP , ErrorGuaranteed , Ident , Span } ;}
mkuse!{use tracing :: debug ;}
mkuse!{use super :: macro_rules :: { MacroRule , NoopTracker , parser_from_cx } ;}
mkuse!{use crate :: expand :: { AstFragmentKind , parse_ast_fragment } ;}
mkuse!{use crate :: mbe :: macro_parser :: ParseResult :: * ;}
mkuse!{use crate :: mbe :: macro_parser :: { MatcherLoc , NamedParseResult , TtParser } ;}
mkuse!{use crate :: mbe :: macro_rules :: { Tracker , try_match_macro , try_match_macro_attr , try_match_macro_derive , } ;}
mkitem!{mkenum!{pub (super) enum FailedMacro < 'a > { Func , Attr (& 'a TokenStream) , Derive , }}}

macro_rules! failed_to_match_macro_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function failed_to_match_macro in module {}", module_path!());
    };
}

mkfn!{
    failed_to_match_macro_introspect!();
    pub (super) fn failed_to_match_macro (psess : & ParseSess , sp : Span , def_span : Span , name : Ident , args : FailedMacro < '_ > , body : & TokenStream , rules : & [MacroRule] ,) -> (Span , ErrorGuaranteed) { debug ! ("failed to match macro") ; let def_head_span = if ! def_span . is_dummy () && ! psess . source_map () . is_imported (def_span) { psess . source_map () . guess_head_span (def_span) } else { DUMMY_SP } ; let mut tracker = CollectTrackerAndEmitter :: new (psess . dcx () , sp) ; let try_success_result = match args { FailedMacro :: Func => try_match_macro (psess , name , body , rules , & mut tracker) , FailedMacro :: Attr (attr_args) => { try_match_macro_attr (psess , name , attr_args , body , rules , & mut tracker) } FailedMacro :: Derive => try_match_macro_derive (psess , name , body , rules , & mut tracker) , } ; if try_success_result . is_ok () { assert ! (tracker . dcx . has_errors () . is_some () , "Macro matching returned a success on the second try") ; } if let Some (result) = tracker . result { return result ; } let Some (BestFailure { token , msg : label , remaining_matcher , .. }) = tracker . best_failure else { return (sp , psess . dcx () . span_delayed_bug (sp , "failed to match a macro")) ; } ; let span = token . span . substitute_dummy (sp) ; let mut err = psess . dcx () . struct_span_err (span , parse_failure_msg (& token , None)) ; err . span_label (span , label) ; if ! def_head_span . is_dummy () { err . span_label (def_head_span , "when calling this macro") ; } annotate_doc_comment (& mut err , psess . source_map () , span) ; if let Some (span) = remaining_matcher . span () { err . span_note (span , format ! ("while trying to match {remaining_matcher}")) ; } else { err . note (format ! ("while trying to match {remaining_matcher}")) ; } if let MatcherLoc :: Token { token : expected_token } = & remaining_matcher && (matches ! (expected_token . kind , token :: OpenInvisible (_)) || matches ! (token . kind , token :: OpenInvisible (_))) { err . note ("captured metavariables except for `:tt`, `:ident` and `:lifetime` cannot be compared to other tokens") ; err . note ("see <https://doc.rust-lang.org/nightly/reference/macros-by-example.html#forwarding-a-matched-fragment> for more information") ; if ! def_span . is_dummy () && ! psess . source_map () . is_imported (def_span) { err . help ("try using `:tt` instead in the macro definition") ; } } if let FailedMacro :: Func = args && let Some ((body , comma_span)) = body . add_comma () { for rule in rules { let MacroRule :: Func { lhs , .. } = rule else { continue } ; let parser = parser_from_cx (psess , body . clone () , Recovery :: Allowed) ; let mut tt_parser = TtParser :: new (name) ; if let Success (_) = tt_parser . parse_tt (& mut Cow :: Borrowed (& parser) , lhs , & mut NoopTracker) { if comma_span . is_dummy () { err . note ("you might be missing a comma") ; } else { err . span_suggestion_short (comma_span , "missing comma here" , ", " , Applicability :: MachineApplicable ,) ; } } } } let guar = err . emit () ; (sp , guar) }
}
mkitem!{mkstruct!{#[doc = " The tracker used for the slow error path that collects useful info for diagnostics."] struct CollectTrackerAndEmitter < 'dcx , 'matcher > { dcx : DiagCtxtHandle < 'dcx > , remaining_matcher : Option < & 'matcher MatcherLoc > , #[doc = " Which arm's failure should we report? (the one furthest along)"] best_failure : Option < BestFailure > , root_span : Span , result : Option < (Span , ErrorGuaranteed) > , }}}
mkitem!{mkstruct!{struct BestFailure { token : Token , position_in_tokenstream : (bool , u32) , msg : & 'static str , remaining_matcher : MatcherLoc , }}}
mkitem!{mkimpl!{impl BestFailure { fn is_better_position (& self , position : (bool , u32)) -> bool { position > self . position_in_tokenstream } }}}
mkitem!{mkimpl!{impl < 'dcx , 'matcher > Tracker < 'matcher > for CollectTrackerAndEmitter < 'dcx , 'matcher > { type Failure = (Token , u32 , & 'static str) ; fn build_failure (tok : Token , position : u32 , msg : & 'static str) -> Self :: Failure { (tok , position , msg) } fn before_match_loc (& mut self , parser : & TtParser , matcher : & 'matcher MatcherLoc) { if self . remaining_matcher . is_none () || (parser . has_no_remaining_items_for_step () && * matcher != MatcherLoc :: Eof) { self . remaining_matcher = Some (matcher) ; } } fn after_arm (& mut self , in_body : bool , result : & NamedParseResult < Self :: Failure >) { match result { Success (_) => { self . dcx . span_delayed_bug (self . root_span , "should not collect detailed info for successful macro match" ,) ; } Failure ((token , approx_position , msg)) => { debug ! (? token , ? msg , "a new failure of an arm") ; let position_in_tokenstream = (in_body , * approx_position) ; if self . best_failure . as_ref () . is_none_or (| failure | failure . is_better_position (position_in_tokenstream)) { self . best_failure = Some (BestFailure { token : * token , position_in_tokenstream , msg , remaining_matcher : self . remaining_matcher . expect ("must have collected matcher already") . clone () , }) } } Error (err_sp , msg) => { let span = err_sp . substitute_dummy (self . root_span) ; let guar = self . dcx . span_err (span , msg . clone ()) ; self . result = Some ((span , guar)) ; } ErrorReported (guar) => self . result = Some ((self . root_span , * guar)) , } } fn description () -> & 'static str { "detailed" } fn recovery () -> Recovery { Recovery :: Allowed } }}}
mkitem!{mkimpl!{impl < 'dcx > CollectTrackerAndEmitter < 'dcx , '_ > { fn new (dcx : DiagCtxtHandle < 'dcx > , root_span : Span) -> Self { Self { dcx , remaining_matcher : None , best_failure : None , root_span , result : None } } }}}

macro_rules! emit_frag_parse_err_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function emit_frag_parse_err in module {}", module_path!());
    };
}

mkfn!{
    emit_frag_parse_err_introspect!();
    pub (super) fn emit_frag_parse_err (mut e : Diag < '_ > , parser : & Parser < '_ > , orig_parser : & mut Parser < '_ > , site_span : Span , arm_span : Span , kind : AstFragmentKind ,) -> ErrorGuaranteed { if parser . token == token :: Eof && let DiagMessage :: Str (message) = & e . messages [0] . 0 && message . ends_with (", found `<eof>`") { let msg = & e . messages [0] ; e . messages [0] = (DiagMessage :: from (format ! ("macro expansion ends with an incomplete expression: {}" , message . replace (", found `<eof>`" , "") ,)) , msg . 1 ,) ; if ! e . span . is_dummy () { e . replace_span_with (parser . token . span . shrink_to_hi () , true) ; } } if e . span . is_dummy () { e . replace_span_with (site_span , true) ; if ! parser . psess . source_map () . is_imported (arm_span) { e . span_label (arm_span , "in this macro arm") ; } } else if parser . psess . source_map () . is_imported (parser . token . span) { e . span_label (site_span , "in this macro invocation") ; } match kind { AstFragmentKind :: Expr => match parse_ast_fragment (orig_parser , AstFragmentKind :: Stmts) { Err (err) => err . cancel () , Ok (_) => { e . note ("the macro call doesn't expand to an expression, but it can expand to a statement" ,) ; if parser . token == token :: Semi { if let Ok (snippet) = parser . psess . source_map () . span_to_snippet (site_span) { e . span_suggestion_verbose (site_span , "surround the macro invocation with `{}` to interpret the expansion as a statement" , format ! ("{{ {snippet}; }}") , Applicability :: MaybeIncorrect ,) ; } } else { e . span_suggestion_verbose (site_span . shrink_to_hi () , "add `;` to interpret the expansion as a statement" , ";" , Applicability :: MaybeIncorrect ,) ; } } } , _ => annotate_err_with_kind (& mut e , kind , site_span) , } ; e . emit () }
}

macro_rules! annotate_err_with_kind_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function annotate_err_with_kind in module {}", module_path!());
    };
}

mkfn!{
    annotate_err_with_kind_introspect!();
    pub (crate) fn annotate_err_with_kind (err : & mut Diag < '_ > , kind : AstFragmentKind , span : Span) { match kind { AstFragmentKind :: Ty => { err . span_label (span , "this macro call doesn't expand to a type") ; } AstFragmentKind :: Pat => { err . span_label (span , "this macro call doesn't expand to a pattern") ; } _ => { } } ; }
}
mkitem!{mkenum!{#[derive (Subdiagnostic)] enum ExplainDocComment { #[label (expand_explain_doc_comment_inner)] Inner { #[primary_span] span : Span , } , #[label (expand_explain_doc_comment_outer)] Outer { #[primary_span] span : Span , } , }}}

macro_rules! annotate_doc_comment_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function annotate_doc_comment in module {}", module_path!());
    };
}

mkfn!{
    annotate_doc_comment_introspect!();
    fn annotate_doc_comment (err : & mut Diag < '_ > , sm : & SourceMap , span : Span) { if let Ok (src) = sm . span_to_snippet (span) { if src . starts_with ("///") || src . starts_with ("/**") { err . subdiagnostic (ExplainDocComment :: Outer { span }) ; } else if src . starts_with ("//") || src . starts_with ("/*!") { err . subdiagnostic (ExplainDocComment :: Inner { span }) ; } } }
}

macro_rules! parse_failure_msg_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_failure_msg in module {}", module_path!());
    };
}

mkfn!{
    parse_failure_msg_introspect!();
    #[doc = " Generates an appropriate parsing failure message. For EOF, this is \"unexpected end...\". For"] #[doc = " other tokens, this is \"unexpected token...\"."] fn parse_failure_msg (tok : & Token , expected_token : Option < & Token >) -> Cow < 'static , str > { if let Some (expected_token) = expected_token { Cow :: from (format ! ("expected {}, found {}" , token_descr (expected_token) , token_descr (tok))) } else { match tok . kind { token :: Eof => Cow :: from ("unexpected end of macro invocation") , _ => Cow :: from (format ! ("no rules expected {}" , token_descr (tok))) , } } }
}