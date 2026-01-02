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
mkuse!{use std :: mem ;}
mkuse!{use rustc_ast :: token :: { self , Delimiter , IdentIsRaw , InvisibleOrigin , Lit , LitKind , MetaVarKind , Token , TokenKind , } ;}
mkuse!{use rustc_ast :: tokenstream :: { DelimSpacing , DelimSpan , Spacing , TokenStream , TokenTree } ;}
mkuse!{use rustc_ast :: { ExprKind , StmtKind , TyKind , UnOp } ;}
mkuse!{use rustc_data_structures :: fx :: FxHashMap ;}
mkuse!{use rustc_errors :: { Diag , DiagCtxtHandle , PResult , pluralize } ;}
mkuse!{use rustc_parse :: lexer :: nfc_normalize ;}
mkuse!{use rustc_parse :: parser :: ParseNtResult ;}
mkuse!{use rustc_session :: parse :: ParseSess ;}
mkuse!{use rustc_span :: hygiene :: { LocalExpnId , Transparency } ;}
mkuse!{use rustc_span :: { Ident , MacroRulesNormalizedIdent , Span , Symbol , SyntaxContext , sym , with_metavar_spans , } ;}
mkuse!{use smallvec :: { SmallVec , smallvec } ;}
mkuse!{use crate :: errors :: { CountRepetitionMisplaced , MetaVarsDifSeqMatchers , MustRepeatOnce , MveUnrecognizedVar , NoSyntaxVarsExprRepeat , VarStillRepeating , } ;}
mkuse!{use crate :: mbe :: macro_parser :: NamedMatch ;}
mkuse!{use crate :: mbe :: macro_parser :: NamedMatch :: * ;}
mkuse!{use crate :: mbe :: metavar_expr :: { MetaVarExprConcatElem , RAW_IDENT_ERR } ;}
mkuse!{use crate :: mbe :: { self , KleeneOp , MetaVarExpr } ;}
mkitem!{mkstruct!{#[doc = " Context needed to perform transcription of metavariable expressions."] struct TranscrCtx < 'psess , 'itp > { psess : & 'psess ParseSess , #[doc = " Map from metavars to matched tokens"] interp : & 'itp FxHashMap < MacroRulesNormalizedIdent , NamedMatch > , #[doc = " Allow marking spans."] marker : Marker , #[doc = " The stack of things yet to be completely expanded."] #[doc = ""] #[doc = " We descend into the RHS (`src`), expanding things as we go. This stack contains the things"] #[doc = " we have yet to expand/are still expanding. We start the stack off with the whole RHS. The"] #[doc = " choice of spacing values doesn't matter."] stack : SmallVec < [Frame < 'itp > ; 1] > , #[doc = " A stack of where we are in the repeat expansion."] #[doc = ""] #[doc = " As we descend in the RHS, we will need to be able to match nested sequences of matchers."] #[doc = " `repeats` keeps track of where we are in matching at each level, with the last element"] #[doc = " being the most deeply nested sequence. This is used as a stack."] repeats : Vec < (usize , usize) > , #[doc = " The resulting token stream from the `TokenTree` we just finished processing."] #[doc = ""] #[doc = " At the end, this will contain the full result of transcription, but at arbitrary points"] #[doc = " during `transcribe`, `result` will contain subsets of the final result."] #[doc = ""] #[doc = " Specifically, as we descend into each TokenTree, we will push the existing results onto the"] #[doc = " `result_stack` and clear `results`. We will then produce the results of transcribing the"] #[doc = " TokenTree into `results`. Then, as we unwind back out of the `TokenTree`, we will pop the"] #[doc = " `result_stack` and append `results` too it to produce the new `results` up to that point."] #[doc = ""] #[doc = " Thus, if we try to pop the `result_stack` and it is empty, we have reached the top-level"] #[doc = " again, and we are done transcribing."] result : Vec < TokenTree > , #[doc = " The in-progress `result` lives at the top of this stack. Each entered `TokenTree` adds a"] #[doc = " new entry."] result_stack : Vec < Vec < TokenTree > > , }}}
mkitem!{mkimpl!{impl < 'psess > TranscrCtx < 'psess , '_ > { #[doc = " Span marked with the correct expansion and transparency."] fn visited_dspan (& mut self , dspan : DelimSpan) -> Span { let mut span = dspan . entire () ; self . marker . mark_span (& mut span) ; span } }}}
mkitem!{mkstruct!{#[doc = " A Marker adds the given mark to the syntax context."] struct Marker { expand_id : LocalExpnId , transparency : Transparency , cache : FxHashMap < SyntaxContext , SyntaxContext > , }}}
mkitem!{mkimpl!{impl Marker { #[doc = " Mark a span with the stored expansion ID and transparency."] fn mark_span (& mut self , span : & mut Span) { * span = span . map_ctxt (| ctxt | { * self . cache . entry (ctxt) . or_insert_with (| | ctxt . apply_mark (self . expand_id . to_expn_id () , self . transparency)) }) ; } }}}
mkitem!{mkstruct!{#[doc = " An iterator over the token trees in a delimited token tree (`{ ... }`) or a sequence (`$(...)`)."] struct Frame < 'a > { tts : & 'a [mbe :: TokenTree] , idx : usize , kind : FrameKind , }}}
mkitem!{mkenum!{enum FrameKind { Delimited { delim : Delimiter , span : DelimSpan , spacing : DelimSpacing } , Sequence { sep : Option < Token > , kleene_op : KleeneOp } , }}}
mkitem!{mkimpl!{impl < 'a > Frame < 'a > { fn new_delimited (src : & 'a mbe :: Delimited , span : DelimSpan , spacing : DelimSpacing) -> Frame < 'a > { Frame { tts : & src . tts , idx : 0 , kind : FrameKind :: Delimited { delim : src . delim , span , spacing } , } } fn new_sequence (src : & 'a mbe :: SequenceRepetition , sep : Option < Token > , kleene_op : KleeneOp ,) -> Frame < 'a > { Frame { tts : & src . tts , idx : 0 , kind : FrameKind :: Sequence { sep , kleene_op } } } }}}
mkitem!{mkimpl!{impl < 'a > Iterator for Frame < 'a > { type Item = & 'a mbe :: TokenTree ; fn next (& mut self) -> Option < & 'a mbe :: TokenTree > { let res = self . tts . get (self . idx) ; self . idx += 1 ; res } }}}

macro_rules! transcribe_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function transcribe in module {}", module_path!());
    };
}

mkfn!{
    transcribe_introspect!();
    #[doc = " This can do Macro-By-Example transcription."] #[doc = " - `interp` is a map of meta-variables to the tokens (non-terminals) they matched in the"] #[doc = "   invocation. We are assuming we already know there is a match."] #[doc = " - `src` is the RHS of the MBE, that is, the \"example\" we are filling in."] #[doc = ""] #[doc = " For example,"] #[doc = ""] #[doc = " ```rust"] #[doc = " macro_rules! foo {"] #[doc = "     ($id:ident) => { println!(\"{}\", stringify!($id)); }"] #[doc = " }"] #[doc = ""] #[doc = " foo!(bar);"] #[doc = " ```"] #[doc = ""] #[doc = " `interp` would contain `$id => bar` and `src` would contain `println!(\"{}\", stringify!($id));`."] #[doc = ""] #[doc = " `transcribe` would return a `TokenStream` containing `println!(\"{}\", stringify!(bar));`."] #[doc = ""] #[doc = " Along the way, we do some additional error checking."] pub (super) fn transcribe < 'a > (psess : & 'a ParseSess , interp : & FxHashMap < MacroRulesNormalizedIdent , NamedMatch > , src : & mbe :: Delimited , src_span : DelimSpan , transparency : Transparency , expand_id : LocalExpnId ,) -> PResult < 'a , TokenStream > { if src . tts . is_empty () { return Ok (TokenStream :: default ()) ; } let mut tscx = TranscrCtx { psess , interp , marker : Marker { expand_id , transparency , cache : Default :: default () } , repeats : Vec :: new () , stack : smallvec ! [Frame :: new_delimited (src , src_span , DelimSpacing :: new (Spacing :: Alone , Spacing :: Alone))] , result : Vec :: new () , result_stack : Vec :: new () , } ; loop { let Some (tree) = tscx . stack . last_mut () . unwrap () . next () else { let frame = tscx . stack . last_mut () . unwrap () ; if let FrameKind :: Sequence { sep , .. } = & frame . kind { let (repeat_idx , repeat_len) = tscx . repeats . last_mut () . unwrap () ; * repeat_idx += 1 ; if repeat_idx < repeat_len { frame . idx = 0 ; if let Some (sep) = sep { tscx . result . push (TokenTree :: Token (* sep , Spacing :: Alone)) ; } continue ; } } match tscx . stack . pop () . unwrap () . kind { FrameKind :: Sequence { .. } => { tscx . repeats . pop () ; } FrameKind :: Delimited { delim , span , mut spacing , .. } => { if delim == Delimiter :: Bracket { spacing . close = Spacing :: Alone ; } if tscx . result_stack . is_empty () { return Ok (TokenStream :: new (tscx . result)) ; } let tree = TokenTree :: Delimited (span , spacing , delim , TokenStream :: new (tscx . result)) ; tscx . result = tscx . result_stack . pop () . unwrap () ; tscx . result . push (tree) ; } } continue ; } ; match tree { seq @ mbe :: TokenTree :: Sequence (_ , seq_rep) => { transcribe_sequence (& mut tscx , seq , seq_rep) ? ; } & mbe :: TokenTree :: MetaVar (sp , original_ident) => { transcribe_metavar (& mut tscx , sp , original_ident) ? ; } mbe :: TokenTree :: MetaVarExpr (dspan , expr) => { transcribe_metavar_expr (& mut tscx , * dspan , expr) ? ; } & mbe :: TokenTree :: Delimited (mut span , ref spacing , ref delimited) => { tscx . marker . mark_span (& mut span . open) ; tscx . marker . mark_span (& mut span . close) ; tscx . stack . push (Frame :: new_delimited (delimited , span , * spacing)) ; tscx . result_stack . push (mem :: take (& mut tscx . result)) ; } & mbe :: TokenTree :: Token (mut token) => { tscx . marker . mark_span (& mut token . span) ; if let token :: NtIdent (ident , _) | token :: NtLifetime (ident , _) = & mut token . kind { tscx . marker . mark_span (& mut ident . span) ; } let tt = TokenTree :: Token (token , Spacing :: Alone) ; tscx . result . push (tt) ; } mbe :: TokenTree :: MetaVarDecl { .. } => panic ! ("unexpected `TokenTree::MetaVarDecl`") , } } }
}

macro_rules! transcribe_sequence_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function transcribe_sequence in module {}", module_path!());
    };
}

mkfn!{
    transcribe_sequence_introspect!();
    #[doc = " Turn `$(...)*` sequences into tokens."] fn transcribe_sequence < 'tx , 'itp > (tscx : & mut TranscrCtx < 'tx , 'itp > , seq : & mbe :: TokenTree , seq_rep : & 'itp mbe :: SequenceRepetition ,) -> PResult < 'tx , () > { let dcx = tscx . psess . dcx () ; match lockstep_iter_size (seq , tscx . interp , & tscx . repeats) { LockstepIterSize :: Unconstrained => { return Err (dcx . create_err (NoSyntaxVarsExprRepeat { span : seq . span () })) ; } LockstepIterSize :: Contradiction (msg) => { return Err (dcx . create_err (MetaVarsDifSeqMatchers { span : seq . span () , msg })) ; } LockstepIterSize :: Constraint (len , _) => { let mbe :: TokenTree :: Sequence (sp , seq) = seq else { unreachable ! () } ; if len == 0 { if seq . kleene . op == KleeneOp :: OneOrMore { return Err (dcx . create_err (MustRepeatOnce { span : sp . entire () })) ; } } else { tscx . repeats . push ((0 , len)) ; tscx . stack . push (Frame :: new_sequence (seq_rep , seq . separator . clone () , seq . kleene . op)) ; } } } Ok (()) }
}

macro_rules! transcribe_metavar_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function transcribe_metavar in module {}", module_path!());
    };
}

mkfn!{
    transcribe_metavar_introspect!();
    #[doc = " Find the matched nonterminal from the macro invocation, and use it to replace"] #[doc = " the meta-var."] #[doc = ""] #[doc = " We use `Spacing::Alone` everywhere here, because that's the conservative choice"] #[doc = " and spacing of declarative macros is tricky. E.g. in this macro:"] #[doc = " ```"] #[doc = " macro_rules! idents {"] #[doc = "     ($($a:ident,)*) => { stringify!($($a)*) }"] #[doc = " }"] #[doc = " ```"] #[doc = " `$a` has no whitespace after it and will be marked `JointHidden`. If you then"] #[doc = " call `idents!(x,y,z,)`, each of `x`, `y`, and `z` will be marked as `Joint`. So"] #[doc = " if you choose to use `$x`'s spacing or the identifier's spacing, you'll end up"] #[doc = " producing \"xyz\", which is bad because it effectively merges tokens."] #[doc = " `Spacing::Alone` is the safer option. Fortunately, `space_between` will avoid"] #[doc = " some of the unnecessary whitespace."] fn transcribe_metavar < 'tx > (tscx : & mut TranscrCtx < 'tx , '_ > , mut sp : Span , mut original_ident : Ident ,) -> PResult < 'tx , () > { let dcx = tscx . psess . dcx () ; let ident = MacroRulesNormalizedIdent :: new (original_ident) ; let Some (cur_matched) = lookup_cur_matched (ident , tscx . interp , & tscx . repeats) else { tscx . marker . mark_span (& mut sp) ; tscx . marker . mark_span (& mut original_ident . span) ; tscx . result . push (TokenTree :: token_joint_hidden (token :: Dollar , sp)) ; tscx . result . push (TokenTree :: Token (Token :: from_ast_ident (original_ident) , Spacing :: Alone)) ; return Ok (()) ; } ; let mut mk_delimited = | mk_span , mv_kind , mut stream : TokenStream | { if stream . len () == 1 { let tree = stream . iter () . next () . unwrap () ; if let TokenTree :: Delimited (_ , _ , delim , inner) = tree && let Delimiter :: Invisible (InvisibleOrigin :: MetaVar (mvk)) = delim && mv_kind == * mvk { stream = inner . clone () ; } } tscx . marker . mark_span (& mut sp) ; with_metavar_spans (| mspans | mspans . insert (mk_span , sp)) ; TokenTree :: Delimited (DelimSpan :: from_single (sp) , DelimSpacing :: new (Spacing :: Alone , Spacing :: Alone) , Delimiter :: Invisible (InvisibleOrigin :: MetaVar (mv_kind)) , stream ,) } ; let tt = match cur_matched { MatchedSingle (ParseNtResult :: Tt (tt)) => { maybe_use_metavar_location (tscx . psess , & tscx . stack , sp , tt , & mut tscx . marker) } MatchedSingle (ParseNtResult :: Ident (ident , is_raw)) => { tscx . marker . mark_span (& mut sp) ; with_metavar_spans (| mspans | mspans . insert (ident . span , sp)) ; let kind = token :: NtIdent (* ident , * is_raw) ; TokenTree :: token_alone (kind , sp) } MatchedSingle (ParseNtResult :: Lifetime (ident , is_raw)) => { tscx . marker . mark_span (& mut sp) ; with_metavar_spans (| mspans | mspans . insert (ident . span , sp)) ; let kind = token :: NtLifetime (* ident , * is_raw) ; TokenTree :: token_alone (kind , sp) } MatchedSingle (ParseNtResult :: Item (item)) => { mk_delimited (item . span , MetaVarKind :: Item , TokenStream :: from_ast (item)) } MatchedSingle (ParseNtResult :: Block (block)) => { mk_delimited (block . span , MetaVarKind :: Block , TokenStream :: from_ast (block)) } MatchedSingle (ParseNtResult :: Stmt (stmt)) => { let stream = if let StmtKind :: Empty = stmt . kind { TokenStream :: token_alone (token :: Semi , stmt . span) } else { TokenStream :: from_ast (stmt) } ; mk_delimited (stmt . span , MetaVarKind :: Stmt , stream) } MatchedSingle (ParseNtResult :: Pat (pat , pat_kind)) => { mk_delimited (pat . span , MetaVarKind :: Pat (* pat_kind) , TokenStream :: from_ast (pat)) } MatchedSingle (ParseNtResult :: Expr (expr , kind)) => { let (can_begin_literal_maybe_minus , can_begin_string_literal) = match & expr . kind { ExprKind :: Lit (_) => (true , true) , ExprKind :: Unary (UnOp :: Neg , e) if matches ! (& e . kind , ExprKind :: Lit (_)) => { (true , false) } _ => (false , false) , } ; mk_delimited (expr . span , MetaVarKind :: Expr { kind : * kind , can_begin_literal_maybe_minus , can_begin_string_literal , } , TokenStream :: from_ast (expr) ,) } MatchedSingle (ParseNtResult :: Literal (lit)) => { mk_delimited (lit . span , MetaVarKind :: Literal , TokenStream :: from_ast (lit)) } MatchedSingle (ParseNtResult :: Ty (ty)) => { let is_path = matches ! (& ty . kind , TyKind :: Path (None , _path)) ; mk_delimited (ty . span , MetaVarKind :: Ty { is_path } , TokenStream :: from_ast (ty)) } MatchedSingle (ParseNtResult :: Meta (attr_item)) => { let has_meta_form = attr_item . meta_kind () . is_some () ; mk_delimited (attr_item . span () , MetaVarKind :: Meta { has_meta_form } , TokenStream :: from_ast (attr_item) ,) } MatchedSingle (ParseNtResult :: Path (path)) => { mk_delimited (path . span , MetaVarKind :: Path , TokenStream :: from_ast (path)) } MatchedSingle (ParseNtResult :: Vis (vis)) => { mk_delimited (vis . span , MetaVarKind :: Vis , TokenStream :: from_ast (vis)) } MatchedSeq (..) => { return Err (dcx . create_err (VarStillRepeating { span : sp , ident })) ; } } ; tscx . result . push (tt) ; Ok (()) }
}

macro_rules! transcribe_metavar_expr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function transcribe_metavar_expr in module {}", module_path!());
    };
}

mkfn!{
    transcribe_metavar_expr_introspect!();
    #[doc = " Turn `${expr(...)}` metavariable expressionss into tokens."] fn transcribe_metavar_expr < 'tx > (tscx : & mut TranscrCtx < 'tx , '_ > , dspan : DelimSpan , expr : & MetaVarExpr ,) -> PResult < 'tx , () > { let dcx = tscx . psess . dcx () ; let tt = match * expr { MetaVarExpr :: Concat (ref elements) => metavar_expr_concat (tscx , dspan , elements) ? , MetaVarExpr :: Count (original_ident , depth) => { let matched = matched_from_ident (dcx , original_ident , tscx . interp) ? ; let count = count_repetitions (dcx , depth , matched , & tscx . repeats , & dspan) ? ; TokenTree :: token_alone (TokenKind :: lit (token :: Integer , sym :: integer (count) , None) , tscx . visited_dspan (dspan) ,) } MetaVarExpr :: Ignore (original_ident) => { let _ = matched_from_ident (dcx , original_ident , tscx . interp) ? ; return Ok (()) ; } MetaVarExpr :: Index (depth) => match tscx . repeats . iter () . nth_back (depth) { Some ((index , _)) => TokenTree :: token_alone (TokenKind :: lit (token :: Integer , sym :: integer (* index) , None) , tscx . visited_dspan (dspan) ,) , None => { return Err (out_of_bounds_err (dcx , tscx . repeats . len () , dspan . entire () , "index")) ; } } , MetaVarExpr :: Len (depth) => match tscx . repeats . iter () . nth_back (depth) { Some ((_ , length)) => TokenTree :: token_alone (TokenKind :: lit (token :: Integer , sym :: integer (* length) , None) , tscx . visited_dspan (dspan) ,) , None => { return Err (out_of_bounds_err (dcx , tscx . repeats . len () , dspan . entire () , "len")) ; } } , } ; tscx . result . push (tt) ; Ok (()) }
}

macro_rules! metavar_expr_concat_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function metavar_expr_concat in module {}", module_path!());
    };
}

mkfn!{
    metavar_expr_concat_introspect!();
    #[doc = " Handle the `${concat(...)}` metavariable expression."] fn metavar_expr_concat < 'tx > (tscx : & mut TranscrCtx < 'tx , '_ > , dspan : DelimSpan , elements : & [MetaVarExprConcatElem] ,) -> PResult < 'tx , TokenTree > { let dcx = tscx . psess . dcx () ; let mut concatenated = String :: new () ; for element in elements . into_iter () { let symbol = match element { MetaVarExprConcatElem :: Ident (elem) => elem . name , MetaVarExprConcatElem :: Literal (elem) => * elem , MetaVarExprConcatElem :: Var (ident) => { match matched_from_ident (dcx , * ident , tscx . interp) ? { NamedMatch :: MatchedSeq (named_matches) => { let Some ((curr_idx , _)) = tscx . repeats . last () else { return Err (dcx . struct_span_err (dspan . entire () , "invalid syntax")) ; } ; match & named_matches [* curr_idx] { MatchedSeq (_) => { return Err (dcx . struct_span_err (ident . span , "nested repetitions with `${concat(...)}` metavariable expressions are not yet supported" ,)) ; } MatchedSingle (pnr) => extract_symbol_from_pnr (dcx , pnr , ident . span) ? , } } NamedMatch :: MatchedSingle (pnr) => { extract_symbol_from_pnr (dcx , pnr , ident . span) ? } } } } ; concatenated . push_str (symbol . as_str ()) ; } let symbol = nfc_normalize (& concatenated) ; let concatenated_span = tscx . visited_dspan (dspan) ; if ! rustc_lexer :: is_ident (symbol . as_str ()) { return Err (dcx . struct_span_err (concatenated_span , "`${concat(..)}` is not generating a valid identifier" ,)) ; } tscx . psess . symbol_gallery . insert (symbol , concatenated_span) ; Ok (TokenTree :: Token (Token :: from_ast_ident (Ident :: new (symbol , concatenated_span)) , Spacing :: Alone ,)) }
}

macro_rules! maybe_use_metavar_location_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function maybe_use_metavar_location in module {}", module_path!());
    };
}

mkfn!{
    maybe_use_metavar_location_introspect!();
    #[doc = " Store the metavariable span for this original span into a side table."] #[doc = " FIXME: Try to put the metavariable span into `SpanData` instead of a side table (#118517)."] #[doc = " An optimal encoding for inlined spans will need to be selected to minimize regressions."] #[doc = " The side table approach is relatively good, but not perfect due to collisions."] #[doc = " In particular, collisions happen when token is passed as an argument through several macro"] #[doc = " calls, like in recursive macros."] #[doc = " The old heuristic below is used to improve spans in case of collisions, but diagnostics are"] #[doc = " still degraded sometimes in those cases."] #[doc = ""] #[doc = " The old heuristic:"] #[doc = ""] #[doc = " Usually metavariables `$var` produce interpolated tokens, which have an additional place for"] #[doc = " keeping both the original span and the metavariable span. For `tt` metavariables that's not the"] #[doc = " case however, and there's no place for keeping a second span. So we try to give the single"] #[doc = " produced span a location that would be most useful in practice (the hygiene part of the span"] #[doc = " must not be changed)."] #[doc = ""] #[doc = " Different locations are useful for different purposes:"] #[doc = " - The original location is useful when we need to report a diagnostic for the original token in"] #[doc = "   isolation, without combining it with any surrounding tokens. This case occurs, but it is not"] #[doc = "   very common in practice."] #[doc = " - The metavariable location is useful when we need to somehow combine the token span with spans"] #[doc = "   of its surrounding tokens. This is the most common way to use token spans."] #[doc = ""] #[doc = " So this function replaces the original location with the metavariable location in all cases"] #[doc = " except these two:"] #[doc = " - The metavariable is an element of undelimited sequence `$($tt)*`."] #[doc = "   These are typically used for passing larger amounts of code, and tokens in that code usually"] #[doc = "   combine with each other and not with tokens outside of the sequence."] #[doc = " - The metavariable span comes from a different crate, then we prefer the more local span."] fn maybe_use_metavar_location (psess : & ParseSess , stack : & [Frame < '_ >] , mut metavar_span : Span , orig_tt : & TokenTree , marker : & mut Marker ,) -> TokenTree { let undelimited_seq = matches ! (stack . last () , Some (Frame { tts : [_] , kind : FrameKind :: Sequence { sep : None , kleene_op : KleeneOp :: ZeroOrMore | KleeneOp :: OneOrMore , .. } , .. })) ; if undelimited_seq { return orig_tt . clone () ; } marker . mark_span (& mut metavar_span) ; let no_collision = match orig_tt { TokenTree :: Token (token , ..) => { with_metavar_spans (| mspans | mspans . insert (token . span , metavar_span)) } TokenTree :: Delimited (dspan , ..) => with_metavar_spans (| mspans | { mspans . insert (dspan . open , metavar_span) && mspans . insert (dspan . close , metavar_span) && mspans . insert (dspan . entire () , metavar_span) }) , } ; if no_collision || psess . source_map () . is_imported (metavar_span) { return orig_tt . clone () ; } match orig_tt { TokenTree :: Token (Token { kind , span } , spacing) => { let span = metavar_span . with_ctxt (span . ctxt ()) ; with_metavar_spans (| mspans | mspans . insert (span , metavar_span)) ; TokenTree :: Token (Token { kind : kind . clone () , span } , * spacing) } TokenTree :: Delimited (dspan , dspacing , delimiter , tts) => { let open = metavar_span . with_ctxt (dspan . open . ctxt ()) ; let close = metavar_span . with_ctxt (dspan . close . ctxt ()) ; with_metavar_spans (| mspans | { mspans . insert (open , metavar_span) && mspans . insert (close , metavar_span) }) ; let dspan = DelimSpan :: from_pair (open , close) ; TokenTree :: Delimited (dspan , * dspacing , * delimiter , tts . clone ()) } } }
}

macro_rules! lookup_cur_matched_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lookup_cur_matched in module {}", module_path!());
    };
}

mkfn!{
    lookup_cur_matched_introspect!();
    #[doc = " Lookup the meta-var named `ident` and return the matched token tree from the invocation using"] #[doc = " the set of matches `interpolations`."] #[doc = ""] #[doc = " See the definition of `repeats` in the `transcribe` function. `repeats` is used to descend"] #[doc = " into the right place in nested matchers. If we attempt to descend too far, the macro writer has"] #[doc = " made a mistake, and we return `None`."] fn lookup_cur_matched < 'a > (ident : MacroRulesNormalizedIdent , interpolations : & 'a FxHashMap < MacroRulesNormalizedIdent , NamedMatch > , repeats : & [(usize , usize)] ,) -> Option < & 'a NamedMatch > { interpolations . get (& ident) . map (| mut matched | { for & (idx , _) in repeats { match matched { MatchedSingle (_) => break , MatchedSeq (ads) => matched = ads . get (idx) . unwrap () , } } matched }) }
}
mkitem!{mkenum!{#[doc = " An accumulator over a TokenTree to be used with `fold`. During transcription, we need to make"] #[doc = " sure that the size of each sequence and all of its nested sequences are the same as the sizes"] #[doc = " of all the matched (nested) sequences in the macro invocation. If they don't match, somebody"] #[doc = " has made a mistake (either the macro writer or caller)."] #[derive (Clone)] enum LockstepIterSize { #[doc = " No constraints on length of matcher. This is true for any TokenTree variants except a"] #[doc = " `MetaVar` with an actual `MatchedSeq` (as opposed to a `MatchedNonterminal`)."] Unconstrained , #[doc = " A `MetaVar` with an actual `MatchedSeq`. The length of the match and the name of the"] #[doc = " meta-var are returned."] Constraint (usize , MacroRulesNormalizedIdent) , #[doc = " Two `Constraint`s on the same sequence had different lengths. This is an error."] Contradiction (String) , }}}
mkitem!{mkimpl!{impl LockstepIterSize { #[doc = " Find incompatibilities in matcher/invocation sizes."] #[doc = " - `Unconstrained` is compatible with everything."] #[doc = " - `Contradiction` is incompatible with everything."] #[doc = " - `Constraint(len)` is only compatible with other constraints of the same length."] fn with (self , other : LockstepIterSize) -> LockstepIterSize { match self { LockstepIterSize :: Unconstrained => other , LockstepIterSize :: Contradiction (_) => self , LockstepIterSize :: Constraint (l_len , l_id) => match other { LockstepIterSize :: Unconstrained => self , LockstepIterSize :: Contradiction (_) => other , LockstepIterSize :: Constraint (r_len , _) if l_len == r_len => self , LockstepIterSize :: Constraint (r_len , r_id) => { let msg = format ! ("meta-variable `{}` repeats {} time{}, but `{}` repeats {} time{}" , l_id , l_len , pluralize ! (l_len) , r_id , r_len , pluralize ! (r_len) ,) ; LockstepIterSize :: Contradiction (msg) } } , } } }}}

macro_rules! lockstep_iter_size_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lockstep_iter_size in module {}", module_path!());
    };
}

mkfn!{
    lockstep_iter_size_introspect!();
    #[doc = " Given a `tree`, make sure that all sequences have the same length as the matches for the"] #[doc = " appropriate meta-vars in `interpolations`."] #[doc = ""] #[doc = " Note that if `repeats` does not match the exact correct depth of a meta-var,"] #[doc = " `lookup_cur_matched` will return `None`, which is why this still works even in the presence of"] #[doc = " multiple nested matcher sequences."] #[doc = ""] #[doc = " Example: `$($($x $y)+*);+` -- we need to make sure that `x` and `y` repeat the same amount as"] #[doc = " each other at the given depth when the macro was invoked. If they don't it might mean they were"] #[doc = " declared at depths which weren't equal or there was a compiler bug. For example, if we have 3 repetitions of"] #[doc = " the outer sequence and 4 repetitions of the inner sequence for `x`, we should have the same for"] #[doc = " `y`; otherwise, we can't transcribe them both at the given depth."] fn lockstep_iter_size (tree : & mbe :: TokenTree , interpolations : & FxHashMap < MacroRulesNormalizedIdent , NamedMatch > , repeats : & [(usize , usize)] ,) -> LockstepIterSize { use mbe :: TokenTree ; match tree { TokenTree :: Delimited (.. , delimited) => { delimited . tts . iter () . fold (LockstepIterSize :: Unconstrained , | size , tt | { size . with (lockstep_iter_size (tt , interpolations , repeats)) }) } TokenTree :: Sequence (_ , seq) => { seq . tts . iter () . fold (LockstepIterSize :: Unconstrained , | size , tt | { size . with (lockstep_iter_size (tt , interpolations , repeats)) }) } TokenTree :: MetaVar (_ , name) | TokenTree :: MetaVarDecl { name , .. } => { let name = MacroRulesNormalizedIdent :: new (* name) ; match lookup_cur_matched (name , interpolations , repeats) { Some (matched) => match matched { MatchedSingle (_) => LockstepIterSize :: Unconstrained , MatchedSeq (ads) => LockstepIterSize :: Constraint (ads . len () , name) , } , _ => LockstepIterSize :: Unconstrained , } } TokenTree :: MetaVarExpr (_ , expr) => { expr . for_each_metavar (LockstepIterSize :: Unconstrained , | lis , ident | { lis . with (lockstep_iter_size (& TokenTree :: MetaVar (ident . span , * ident) , interpolations , repeats ,)) }) } TokenTree :: Token (..) => LockstepIterSize :: Unconstrained , } }
}

macro_rules! count_repetitions_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function count_repetitions in module {}", module_path!());
    };
}

mkfn!{
    count_repetitions_introspect!();
    #[doc = " Used solely by the `count` meta-variable expression, counts the outermost repetitions at a"] #[doc = " given optional nested depth."] #[doc = ""] #[doc = " For example, a macro parameter of `$( { $( $foo:ident ),* } )*` called with `{ a, b } { c }`:"] #[doc = ""] #[doc = " * `[ $( ${count(foo)} ),* ]` will return [2, 1] with a, b = 2 and c = 1"] #[doc = " * `[ $( ${count(foo, 0)} ),* ]` will be the same as `[ $( ${count(foo)} ),* ]`"] #[doc = " * `[ $( ${count(foo, 1)} ),* ]` will return an error because `${count(foo, 1)}` is"] #[doc = "   declared inside a single repetition and the index `1` implies two nested repetitions."] fn count_repetitions < 'dx > (dcx : DiagCtxtHandle < 'dx > , depth_user : usize , mut matched : & NamedMatch , repeats : & [(usize , usize)] , sp : & DelimSpan ,) -> PResult < 'dx , usize > { fn count < 'a > (depth_curr : usize , depth_max : usize , matched : & NamedMatch) -> PResult < 'a , usize > { match matched { MatchedSingle (_) => Ok (1) , MatchedSeq (named_matches) => { if depth_curr == depth_max { Ok (named_matches . len ()) } else { named_matches . iter () . map (| elem | count (depth_curr + 1 , depth_max , elem)) . sum () } } } } #[doc = " Maximum depth"] fn depth (counter : usize , matched : & NamedMatch) -> usize { match matched { MatchedSingle (_) => counter , MatchedSeq (named_matches) => { let rslt = counter + 1 ; if let Some (elem) = named_matches . first () { depth (rslt , elem) } else { rslt } } } } let depth_max = depth (0 , matched) . checked_sub (1) . and_then (| el | el . checked_sub (repeats . len ())) . unwrap_or_default () ; if depth_user > depth_max { return Err (out_of_bounds_err (dcx , depth_max + 1 , sp . entire () , "count")) ; } for & (idx , _) in repeats { if let MatchedSeq (ads) = matched { matched = & ads [idx] ; } } if let MatchedSingle (_) = matched { return Err (dcx . create_err (CountRepetitionMisplaced { span : sp . entire () })) ; } count (depth_user , depth_max , matched) }
}

macro_rules! matched_from_ident_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function matched_from_ident in module {}", module_path!());
    };
}

mkfn!{
    matched_from_ident_introspect!();
    #[doc = " Returns a `NamedMatch` item declared on the LHS given an arbitrary [Ident]"] fn matched_from_ident < 'ctx , 'interp , 'rslt > (dcx : DiagCtxtHandle < 'ctx > , ident : Ident , interp : & 'interp FxHashMap < MacroRulesNormalizedIdent , NamedMatch > ,) -> PResult < 'ctx , & 'rslt NamedMatch > where 'interp : 'rslt , { let span = ident . span ; let key = MacroRulesNormalizedIdent :: new (ident) ; interp . get (& key) . ok_or_else (| | dcx . create_err (MveUnrecognizedVar { span , key })) }
}

macro_rules! out_of_bounds_err_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function out_of_bounds_err in module {}", module_path!());
    };
}

mkfn!{
    out_of_bounds_err_introspect!();
    #[doc = " Used by meta-variable expressions when an user input is out of the actual declared bounds. For"] #[doc = " example, index(999999) in an repetition of only three elements."] fn out_of_bounds_err < 'a > (dcx : DiagCtxtHandle < 'a > , max : usize , span : Span , ty : & str) -> Diag < 'a > { let msg = if max == 0 { format ! ("meta-variable expression `{ty}` with depth parameter \
             must be called inside of a macro repetition") } else { format ! ("depth parameter of meta-variable expression `{ty}` \
             must be less than {max}") } ; dcx . struct_span_err (span , msg) }
}

macro_rules! extract_symbol_from_pnr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function extract_symbol_from_pnr in module {}", module_path!());
    };
}

mkfn!{
    extract_symbol_from_pnr_introspect!();
    #[doc = " Extracts an metavariable symbol that can be an identifier, a token tree or a literal."] fn extract_symbol_from_pnr < 'a > (dcx : DiagCtxtHandle < 'a > , pnr : & ParseNtResult , span_err : Span ,) -> PResult < 'a , Symbol > { match pnr { ParseNtResult :: Ident (nt_ident , is_raw) => { if let IdentIsRaw :: Yes = is_raw { Err (dcx . struct_span_err (span_err , RAW_IDENT_ERR)) } else { Ok (nt_ident . name) } } ParseNtResult :: Tt (TokenTree :: Token (Token { kind : TokenKind :: Ident (symbol , is_raw) , .. } , _ ,)) => { if let IdentIsRaw :: Yes = is_raw { Err (dcx . struct_span_err (span_err , RAW_IDENT_ERR)) } else { Ok (* symbol) } } ParseNtResult :: Tt (TokenTree :: Token (Token { kind : TokenKind :: Literal (Lit { kind : LitKind :: Str , symbol , suffix : None }) , .. } , _ ,)) => Ok (* symbol) , ParseNtResult :: Literal (expr) if let ExprKind :: Lit (Lit { kind : LitKind :: Str , symbol , suffix : None }) = & expr . kind => { Ok (* symbol) } ParseNtResult :: Literal (expr) if let ExprKind :: Lit (lit @ Lit { kind : LitKind :: Integer , symbol , suffix }) = & expr . kind => { if lit . is_semantic_float () { Err (dcx . struct_err ("floats are not supported as metavariables of `${concat(..)}`") . with_span (span_err)) } else if suffix . is_none () { Ok (* symbol) } else { Err (dcx . struct_err ("integer metavariables of `${concat(..)}` must not be suffixed") . with_span (span_err)) } } _ => Err (dcx . struct_err ("metavariables of `${concat(..)}` must be of type `ident`, `literal` or `tt`" ,) . with_note ("currently only string and integer literals are supported") . with_span (span_err)) , } }
}