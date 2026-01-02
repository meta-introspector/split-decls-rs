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
mkuse!{use std :: path :: PathBuf ;}
mkuse!{use std :: result ;}
mkuse!{use std :: sync :: Arc ;}
mkuse!{use rustc_ast :: { LitKind , MetaItemKind , token } ;}
mkuse!{use rustc_codegen_ssa :: traits :: CodegenBackend ;}
mkuse!{use rustc_data_structures :: fx :: { FxHashMap , FxHashSet } ;}
mkuse!{use rustc_data_structures :: jobserver :: { self , Proxy } ;}
mkuse!{use rustc_data_structures :: stable_hasher :: StableHasher ;}
mkuse!{use rustc_errors :: registry :: Registry ;}
mkuse!{use rustc_errors :: { DiagCtxtHandle , ErrorGuaranteed } ;}
mkuse!{use rustc_lint :: LintStore ;}
mkuse!{use rustc_middle :: ty ;}
mkuse!{use rustc_middle :: ty :: CurrentGcx ;}
mkuse!{use rustc_middle :: util :: Providers ;}
mkuse!{use rustc_parse :: lexer :: StripTokens ;}
mkuse!{use rustc_parse :: new_parser_from_source_str ;}
mkuse!{use rustc_parse :: parser :: attr :: AllowLeadingUnsafe ;}
mkuse!{use rustc_query_impl :: QueryCtxt ;}
mkuse!{use rustc_query_system :: query :: print_query_stack ;}
mkuse!{use rustc_session :: config :: { self , Cfg , CheckCfg , ExpectedValues , Input , OutFileName } ;}
mkuse!{use rustc_session :: parse :: ParseSess ;}
mkuse!{use rustc_session :: { CompilerIO , EarlyDiagCtxt , Session , lint } ;}
mkuse!{use rustc_span :: source_map :: { FileLoader , RealFileLoader , SourceMapInputs } ;}
mkuse!{use rustc_span :: { FileName , sym } ;}
mkuse!{use tracing :: trace ;}
mkuse!{use crate :: util ;}
mkitem!{pub type Result < T > = result :: Result < T , ErrorGuaranteed > ;}
mkitem!{mkstruct!{#[doc = " Represents a compiler session. Note that every `Compiler` contains a"] #[doc = " `Session`, but `Compiler` also contains some things that cannot be in"] #[doc = " `Session`, due to `Session` being in a crate that has many fewer"] #[doc = " dependencies than this crate."] #[doc = ""] #[doc = " Can be used to run `rustc_interface` queries."] #[doc = " Created by passing [`Config`] to [`run_compiler`]."] pub struct Compiler { pub sess : Session , pub codegen_backend : Box < dyn CodegenBackend > , pub (crate) override_queries : Option < fn (& Session , & mut Providers) > , #[doc = " A reference to the current `GlobalCtxt` which we pass on to `GlobalCtxt`."] pub (crate) current_gcx : CurrentGcx , #[doc = " A jobserver reference which we pass on to `GlobalCtxt`."] pub (crate) jobserver_proxy : Arc < Proxy > , }}}

macro_rules! parse_cfg_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_cfg in module {}", module_path!());
    };
}

mkfn!{
    parse_cfg_introspect!();
    #[doc = " Converts strings provided as `--cfg [cfgspec]` into a `Cfg`."] pub (crate) fn parse_cfg (dcx : DiagCtxtHandle < '_ > , cfgs : Vec < String >) -> Cfg { cfgs . into_iter () . map (| s | { let psess = ParseSess :: with_fatal_emitter (vec ! [crate :: DEFAULT_LOCALE_RESOURCE , rustc_parse :: DEFAULT_LOCALE_RESOURCE] , format ! ("this error occurred on the command line: `--cfg={s}`") ,) ; let filename = FileName :: cfg_spec_source_code (& s) ; macro_rules ! error { ($ reason : expr) => { #[allow (rustc :: untranslatable_diagnostic)] #[allow (rustc :: diagnostic_outside_of_impl)] dcx . fatal (format ! (concat ! ("invalid `--cfg` argument: `{}` (" , $ reason , ")") , s)) ; } ; } match new_parser_from_source_str (& psess , filename , s . to_string () , StripTokens :: Nothing) { Ok (mut parser) => match parser . parse_meta_item (AllowLeadingUnsafe :: No) { Ok (meta_item) if parser . token == token :: Eof => { if meta_item . path . segments . len () != 1 { error ! ("argument key must be an identifier") ; } match & meta_item . kind { MetaItemKind :: List (..) => { } MetaItemKind :: NameValue (lit) if ! lit . kind . is_str () => { error ! ("argument value must be a string") ; } MetaItemKind :: NameValue (..) | MetaItemKind :: Word => { let ident = meta_item . ident () . expect ("multi-segment cfg key") ; return (ident . name , meta_item . value_str ()) ; } } } Ok (..) => { } Err (err) => err . cancel () , } , Err (errs) => errs . into_iter () . for_each (| err | err . cancel ()) , } if s . contains ('=') && ! s . contains ("=\"") && ! s . ends_with ('"') { error ! (concat ! (r#"expected `key` or `key="value"`, ensure escaping is appropriate"# , r#" for your shell, try 'key="value"' or key=\"value\""#)) ; } else { error ! (r#"expected `key` or `key="value"`"#) ; } }) . collect :: < Cfg > () }
}

macro_rules! parse_check_cfg_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_check_cfg in module {}", module_path!());
    };
}

mkfn!{
    parse_check_cfg_introspect!();
    #[doc = " Converts strings provided as `--check-cfg [specs]` into a `CheckCfg`."] pub (crate) fn parse_check_cfg (dcx : DiagCtxtHandle < '_ > , specs : Vec < String >) -> CheckCfg { let exhaustive_names = ! specs . is_empty () ; let exhaustive_values = ! specs . is_empty () ; let mut check_cfg = CheckCfg { exhaustive_names , exhaustive_values , .. CheckCfg :: default () } ; for s in specs { let psess = ParseSess :: with_fatal_emitter (vec ! [crate :: DEFAULT_LOCALE_RESOURCE , rustc_parse :: DEFAULT_LOCALE_RESOURCE] , format ! ("this error occurred on the command line: `--check-cfg={s}`") ,) ; let filename = FileName :: cfg_spec_source_code (& s) ; const VISIT : & str = "visit <https://doc.rust-lang.org/nightly/rustc/check-cfg.html> for more details" ; macro_rules ! error { ($ reason : expr) => { #[allow (rustc :: untranslatable_diagnostic)] #[allow (rustc :: diagnostic_outside_of_impl)] { let mut diag = dcx . struct_fatal (format ! ("invalid `--check-cfg` argument: `{s}`")) ; diag . note ($ reason) ; diag . note (VISIT) ; diag . emit () } } ; (in $ arg : expr , $ reason : expr) => { #[allow (rustc :: untranslatable_diagnostic)] #[allow (rustc :: diagnostic_outside_of_impl)] { let mut diag = dcx . struct_fatal (format ! ("invalid `--check-cfg` argument: `{s}`")) ; let pparg = rustc_ast_pretty :: pprust :: meta_list_item_to_string ($ arg) ; if let Some (lit) = $ arg . lit () { let (lit_kind_article , lit_kind_descr) = { let lit_kind = lit . as_token_lit () . kind ; (lit_kind . article () , lit_kind . descr ()) } ; diag . note (format ! ("`{pparg}` is {lit_kind_article} {lit_kind_descr} literal")) ; } else { diag . note (format ! ("`{pparg}` is invalid")) ; } diag . note ($ reason) ; diag . note (VISIT) ; diag . emit () } } ; } let expected_error = | | -> ! { error ! ("expected `cfg(name, values(\"value1\", \"value2\", ... \"valueN\"))`") } ; let mut parser = match new_parser_from_source_str (& psess , filename , s . to_string () , StripTokens :: Nothing) { Ok (parser) => parser , Err (errs) => { errs . into_iter () . for_each (| err | err . cancel ()) ; expected_error () ; } } ; let meta_item = match parser . parse_meta_item (AllowLeadingUnsafe :: No) { Ok (meta_item) if parser . token == token :: Eof => meta_item , Ok (..) => expected_error () , Err (err) => { err . cancel () ; expected_error () ; } } ; let Some (args) = meta_item . meta_item_list () else { expected_error () ; } ; if ! meta_item . has_name (sym :: cfg) { expected_error () ; } let mut names = Vec :: new () ; let mut values : FxHashSet < _ > = Default :: default () ; let mut any_specified = false ; let mut values_specified = false ; let mut values_any_specified = false ; for arg in args { if arg . is_word () && let Some (ident) = arg . ident () { if values_specified { error ! ("`cfg()` names cannot be after values") ; } names . push (ident) ; } else if let Some (boolean) = arg . boolean_literal () { if values_specified { error ! ("`cfg()` names cannot be after values") ; } names . push (rustc_span :: Ident :: new (if boolean { rustc_span :: kw :: True } else { rustc_span :: kw :: False } , arg . span () ,)) ; } else if arg . has_name (sym :: any) && let Some (args) = arg . meta_item_list () { if any_specified { error ! ("`any()` cannot be specified multiple times") ; } any_specified = true ; if ! args . is_empty () { error ! (in arg , "`any()` takes no argument") ; } } else if arg . has_name (sym :: values) && let Some (args) = arg . meta_item_list () { if names . is_empty () { error ! ("`values()` cannot be specified before the names") ; } else if values_specified { error ! ("`values()` cannot be specified multiple times") ; } values_specified = true ; for arg in args { if let Some (LitKind :: Str (s , _)) = arg . lit () . map (| lit | & lit . kind) { values . insert (Some (* s)) ; } else if arg . has_name (sym :: any) && let Some (args) = arg . meta_item_list () { if values_any_specified { error ! (in arg , "`any()` in `values()` cannot be specified multiple times") ; } values_any_specified = true ; if ! args . is_empty () { error ! (in arg , "`any()` in `values()` takes no argument") ; } } else if arg . has_name (sym :: none) && let Some (args) = arg . meta_item_list () { values . insert (None) ; if ! args . is_empty () { error ! (in arg , "`none()` in `values()` takes no argument") ; } } else { error ! (in arg , "`values()` arguments must be string literals, `none()` or `any()`") ; } } } else { error ! (in arg , "`cfg()` arguments must be simple identifiers, `any()` or `values(...)`") ; } } if ! values_specified && ! any_specified { values . insert (None) ; } else if ! values . is_empty () && values_any_specified { error ! ("`values()` arguments cannot specify string literals and `any()` at the same time") ; } if any_specified { if names . is_empty () && values . is_empty () && ! values_specified && ! values_any_specified { check_cfg . exhaustive_names = false ; } else { error ! ("`cfg(any())` can only be provided in isolation") ; } } else { for name in names { check_cfg . expecteds . entry (name . name) . and_modify (| v | match v { ExpectedValues :: Some (v) if ! values_any_specified => { #[allow (rustc :: potential_query_instability)] v . extend (values . clone ()) } ExpectedValues :: Some (_) => * v = ExpectedValues :: Any , ExpectedValues :: Any => { } }) . or_insert_with (| | { if values_any_specified { ExpectedValues :: Any } else { ExpectedValues :: Some (values . clone ()) } }) ; } } } check_cfg }
}
mkitem!{mkstruct!{#[doc = " The compiler configuration"] pub struct Config { #[doc = " Command line options"] pub opts : config :: Options , #[doc = " Unparsed cfg! configuration in addition to the default ones."] pub crate_cfg : Vec < String > , pub crate_check_cfg : Vec < String > , pub input : Input , pub output_dir : Option < PathBuf > , pub output_file : Option < OutFileName > , pub ice_file : Option < PathBuf > , #[doc = " Load files from sources other than the file system."] #[doc = ""] #[doc = " Has no uses within this repository, but may be used in the future by"] #[doc = " bjorn3 for \"hooking rust-analyzer's VFS into rustc at some point for"] #[doc = " running rustc without having to save\". (See #102759.)"] pub file_loader : Option < Box < dyn FileLoader + Send + Sync > > , #[doc = " The list of fluent resources, used for lints declared with"] #[doc = " [`Diagnostic`](rustc_errors::Diagnostic) and [`LintDiagnostic`](rustc_errors::LintDiagnostic)."] pub locale_resources : Vec < & 'static str > , pub lint_caps : FxHashMap < lint :: LintId , lint :: Level > , #[doc = " This is a callback from the driver that is called when [`ParseSess`] is created."] pub psess_created : Option < Box < dyn FnOnce (& mut ParseSess) + Send > > , #[doc = " This is a callback to hash otherwise untracked state used by the caller, if the"] #[doc = " hash changes between runs the incremental cache will be cleared."] #[doc = ""] #[doc = " e.g. used by Clippy to hash its config file"] pub hash_untracked_state : Option < Box < dyn FnOnce (& Session , & mut StableHasher) + Send > > , #[doc = " This is a callback from the driver that is called when we're registering lints;"] #[doc = " it is called during lint loading when we have the LintStore in a non-shared state."] #[doc = ""] #[doc = " Note that if you find a Some here you probably want to call that function in the new"] #[doc = " function being registered."] pub register_lints : Option < Box < dyn Fn (& Session , & mut LintStore) + Send + Sync > > , #[doc = " This is a callback from the driver that is called just after we have populated"] #[doc = " the list of queries."] pub override_queries : Option < fn (& Session , & mut Providers) > , #[doc = " An extra set of symbols to add to the symbol interner, the symbol indices"] #[doc = " will start at [`PREDEFINED_SYMBOLS_COUNT`](rustc_span::symbol::PREDEFINED_SYMBOLS_COUNT)"] pub extra_symbols : Vec < & 'static str > , #[doc = " This is a callback from the driver that is called to create a codegen backend."] #[doc = ""] #[doc = " Has no uses within this repository, but is used by bjorn3 for \"the"] #[doc = " hotswapping branch of cg_clif\" for \"setting the codegen backend from a"] #[doc = " custom driver where the custom codegen backend has arbitrary data.\""] #[doc = " (See #102759.)"] pub make_codegen_backend : Option < Box < dyn FnOnce (& config :: Options) -> Box < dyn CodegenBackend > + Send > > , #[doc = " Registry of diagnostics codes."] pub registry : Registry , #[doc = " The inner atomic value is set to true when a feature marked as `internal` is"] #[doc = " enabled. Makes it so that \"please report a bug\" is hidden, as ICEs with"] #[doc = " internal features are wontfix, and they are usually the cause of the ICEs."] pub using_internal_features : & 'static std :: sync :: atomic :: AtomicBool , #[doc = " All commandline args used to invoke the compiler, with @file args fully expanded."] #[doc = " This will only be used within debug info, e.g. in the pdb file on windows"] #[doc = " This is mainly useful for other tools that reads that debuginfo to figure out"] #[doc = " how to call the compiler with the same arguments."] pub expanded_args : Vec < String > , }}}

macro_rules! initialize_checked_jobserver_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function initialize_checked_jobserver in module {}", module_path!());
    };
}

mkfn!{
    initialize_checked_jobserver_introspect!();
    #[doc = " Initialize jobserver before getting `jobserver::client` and `build_session`."] pub (crate) fn initialize_checked_jobserver (early_dcx : & EarlyDiagCtxt) { jobserver :: initialize_checked (| err | { #[allow (rustc :: untranslatable_diagnostic)] #[allow (rustc :: diagnostic_outside_of_impl)] early_dcx . early_struct_warn (err) . with_note ("the build environment is likely misconfigured") . emit () }) ; }
}

macro_rules! run_compiler_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function run_compiler in module {}", module_path!());
    };
}

mkfn!{
    run_compiler_introspect!();
    #[allow (rustc :: bad_opt_access)] pub fn run_compiler < R : Send > (config : Config , f : impl FnOnce (& Compiler) -> R + Send) -> R { trace ! ("run_compiler") ; rustc_data_structures :: sync :: set_dyn_thread_safe_mode (config . opts . unstable_opts . threads > 1) ; let early_dcx = EarlyDiagCtxt :: new (config . opts . error_format) ; initialize_checked_jobserver (& early_dcx) ; crate :: callbacks :: setup_callbacks () ; let target = config :: build_target_config (& early_dcx , & config . opts . target_triple , config . opts . sysroot . path () ,) ; let file_loader = config . file_loader . unwrap_or_else (| | Box :: new (RealFileLoader)) ; let path_mapping = config . opts . file_path_mapping () ; let hash_kind = config . opts . unstable_opts . src_hash_algorithm (& target) ; let checksum_hash_kind = config . opts . unstable_opts . checksum_hash_algorithm () ; util :: run_in_thread_pool_with_globals (& early_dcx , config . opts . edition , config . opts . unstable_opts . threads , & config . extra_symbols , SourceMapInputs { file_loader , path_mapping , hash_kind , checksum_hash_kind } , | current_gcx , jobserver_proxy | { let early_dcx = EarlyDiagCtxt :: new (config . opts . error_format) ; let codegen_backend = match config . make_codegen_backend { None => util :: get_codegen_backend (& early_dcx , & config . opts . sysroot , config . opts . unstable_opts . codegen_backend . as_deref () , & target ,) , Some (make_codegen_backend) => { make_codegen_backend (& config . opts) } } ; let temps_dir = config . opts . unstable_opts . temps_dir . as_deref () . map (PathBuf :: from) ; let bundle = match rustc_errors :: fluent_bundle (& config . opts . sysroot . all_paths () . collect :: < Vec < _ > > () , config . opts . unstable_opts . translate_lang . clone () , config . opts . unstable_opts . translate_additional_ftl . as_deref () , config . opts . unstable_opts . translate_directionality_markers ,) { Ok (bundle) => bundle , Err (e) => { #[allow (rustc :: untranslatable_diagnostic)] early_dcx . early_fatal (format ! ("failed to load fluent bundle: {e}")) } } ; let mut locale_resources = config . locale_resources ; locale_resources . push (codegen_backend . locale_resource ()) ; let mut sess = rustc_session :: build_session (config . opts , CompilerIO { input : config . input , output_dir : config . output_dir , output_file : config . output_file , temps_dir , } , bundle , config . registry , locale_resources , config . lint_caps , target , util :: rustc_version_str () . unwrap_or ("unknown") , config . ice_file , config . using_internal_features , config . expanded_args ,) ; codegen_backend . init (& sess) ; let cfg = parse_cfg (sess . dcx () , config . crate_cfg) ; let mut cfg = config :: build_configuration (& sess , cfg) ; util :: add_configuration (& mut cfg , & mut sess , & * codegen_backend) ; sess . psess . config = cfg ; let mut check_cfg = parse_check_cfg (sess . dcx () , config . crate_check_cfg) ; check_cfg . fill_well_known (& sess . target) ; sess . psess . check_config = check_cfg ; if let Some (psess_created) = config . psess_created { psess_created (& mut sess . psess) ; } if let Some (hash_untracked_state) = config . hash_untracked_state { let mut hasher = StableHasher :: new () ; hash_untracked_state (& sess , & mut hasher) ; sess . opts . untracked_state_hash = hasher . finish () } let mut lint_store = rustc_lint :: new_lint_store (sess . enable_internal_lints ()) ; if let Some (register_lints) = config . register_lints . as_deref () { register_lints (& sess , & mut lint_store) ; } sess . lint_store = Some (Arc :: new (lint_store)) ; util :: check_abi_required_features (& sess) ; let compiler = Compiler { sess , codegen_backend , override_queries : config . override_queries , current_gcx , jobserver_proxy , } ; let res = std :: panic :: catch_unwind (std :: panic :: AssertUnwindSafe (| | f (& compiler))) ; compiler . sess . finish_diagnostics () ; if res . is_ok () { compiler . sess . dcx () . abort_if_errors () ; } compiler . sess . dcx () . flush_delayed () ; let res = match res { Ok (res) => res , Err (err) => std :: panic :: resume_unwind (err) , } ; let prof = compiler . sess . prof . clone () ; prof . generic_activity ("drop_compiler") . run (move | | drop (compiler)) ; res } ,) }
}

macro_rules! try_print_query_stack_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function try_print_query_stack in module {}", module_path!());
    };
}

mkfn!{
    try_print_query_stack_introspect!();
    pub fn try_print_query_stack (dcx : DiagCtxtHandle < '_ > , limit_frames : Option < usize > , file : Option < std :: fs :: File > ,) { eprintln ! ("query stack during panic:") ; let all_frames = ty :: tls :: with_context_opt (| icx | { if let Some (icx) = icx { ty :: print :: with_no_queries ! (print_query_stack (QueryCtxt :: new (icx . tcx) , icx . query , dcx , limit_frames , file ,)) } else { 0 } }) ; if let Some (limit_frames) = limit_frames && all_frames > limit_frames { eprintln ! ("... and {} other queries... use `env RUST_BACKTRACE=1` to see the full query stack" , all_frames - limit_frames) ; } else { eprintln ! ("end of query stack") ; } }
}