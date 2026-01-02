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
mkuse!{use std :: ffi :: { OsStr , OsString } ;}
mkuse!{use std :: fs :: { self , File } ;}
mkuse!{use std :: io :: prelude :: * ;}
mkuse!{use std :: path :: { Path , PathBuf } ;}
mkuse!{use std :: { env , io , iter , mem , str } ;}
mkuse!{use cc :: windows_registry ;}
mkuse!{use rustc_hir :: def_id :: { CrateNum , LOCAL_CRATE } ;}
mkuse!{use rustc_metadata :: { find_native_static_library , try_find_native_dynamic_library , try_find_native_static_library , } ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: middle :: dependency_format :: Linkage ;}
mkuse!{use rustc_middle :: middle :: exported_symbols :: { self , ExportedSymbol , SymbolExportInfo , SymbolExportKind , SymbolExportLevel , } ;}
mkuse!{use rustc_middle :: ty :: TyCtxt ;}
mkuse!{use rustc_session :: Session ;}
mkuse!{use rustc_session :: config :: { self , CrateType , DebugInfo , LinkerPluginLto , Lto , OptLevel , Strip } ;}
mkuse!{use rustc_span :: sym ;}
mkuse!{use rustc_target :: spec :: { Cc , LinkOutputKind , LinkerFlavor , Lld } ;}
mkuse!{use tracing :: { debug , warn } ;}
mkuse!{use super :: command :: Command ;}
mkuse!{use super :: symbol_export ;}
mkuse!{use crate :: back :: symbol_export :: allocator_shim_symbols ;}
mkuse!{use crate :: base :: needs_allocator_shim_for_linking ;}
mkuse!{use crate :: errors ;}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                 
            }}

macro_rules! disable_localization_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function disable_localization in module {}", module_path!());
    };
}

mkfn!{
    disable_localization_introspect!();
    #[doc = " Disables non-English messages from localized linkers."] #[doc = " Such messages may cause issues with text encoding on Windows (#35785)"] #[doc = " and prevent inspection of linker output in case of errors, which we occasionally do."] #[doc = " This should be acceptable because other messages from rustc are in English anyway,"] #[doc = " and may also be desirable to improve searchability of the linker diagnostics."] pub (crate) fn disable_localization (linker : & mut Command) { linker . env ("LC_ALL" , "C") ; linker . env ("VSLANG" , "1033") ; }
}

macro_rules! get_linker_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_linker in module {}", module_path!());
    };
}

mkfn!{
    get_linker_introspect!();
    #[doc = " The third parameter is for env vars, used on windows to set up the"] #[doc = " path for MSVC to find its DLLs, and gcc to find its bundled"] #[doc = " toolchain"] pub (crate) fn get_linker < 'a > (sess : & 'a Session , linker : & Path , flavor : LinkerFlavor , self_contained : bool , target_cpu : & 'a str ,) -> Box < dyn Linker + 'a > { let msvc_tool = windows_registry :: find_tool (& sess . target . arch , "link.exe") ; let mut cmd = match linker . to_str () { Some (linker) if cfg ! (windows) && linker . ends_with (".bat") => Command :: bat_script (linker) , _ => match flavor { LinkerFlavor :: Gnu (Cc :: No , Lld :: Yes) | LinkerFlavor :: Darwin (Cc :: No , Lld :: Yes) | LinkerFlavor :: WasmLld (Cc :: No) | LinkerFlavor :: Msvc (Lld :: Yes) => Command :: lld (linker , flavor . lld_flavor ()) , LinkerFlavor :: Msvc (Lld :: No) if sess . opts . cg . linker . is_none () && sess . target . linker . is_none () => { Command :: new (msvc_tool . as_ref () . map_or (linker , | t | t . path ())) } _ => Command :: new (linker) , } , } ; let t = & sess . target ; if matches ! (flavor , LinkerFlavor :: Msvc (..)) && t . vendor == "uwp" { if let Some (ref tool) = msvc_tool { let original_path = tool . path () ; if let Some (root_lib_path) = original_path . ancestors () . nth (4) { let arch = match t . arch . as_ref () { "x86_64" => Some ("x64") , "x86" => Some ("x86") , "aarch64" => Some ("arm64") , "arm" => Some ("arm") , _ => None , } ; if let Some (ref a) = arch { let mut arg = OsString :: from ("/LIBPATH:") ; arg . push (format ! ("{}\\lib\\{}\\store" , root_lib_path . display () , a)) ; cmd . arg (& arg) ; } else { warn ! ("arch is not supported") ; } } else { warn ! ("MSVC root path lib location not found") ; } } else { warn ! ("link.exe not found") ; } } let mut new_path = sess . get_tools_search_paths (self_contained) ; let mut msvc_changed_path = false ; if sess . target . is_like_msvc && let Some (ref tool) = msvc_tool { cmd . args (tool . args ()) ; for (k , v) in tool . env () { if k == "PATH" { new_path . extend (env :: split_paths (v)) ; msvc_changed_path = true ; } else { cmd . env (k , v) ; } } } if ! msvc_changed_path && let Some (path) = env :: var_os ("PATH") { new_path . extend (env :: split_paths (& path)) ; } cmd . env ("PATH" , env :: join_paths (new_path) . unwrap ()) ; assert ! (cmd . get_args () . is_empty () || sess . target . vendor == "uwp") ; match flavor { LinkerFlavor :: Unix (Cc :: No) if sess . target . os == "l4re" => { Box :: new (L4Bender :: new (cmd , sess)) as Box < dyn Linker > } LinkerFlavor :: Unix (Cc :: No) if sess . target . os == "aix" => { Box :: new (AixLinker :: new (cmd , sess)) as Box < dyn Linker > } LinkerFlavor :: WasmLld (Cc :: No) => Box :: new (WasmLd :: new (cmd , sess)) as Box < dyn Linker > , LinkerFlavor :: Gnu (cc , _) | LinkerFlavor :: Darwin (cc , _) | LinkerFlavor :: WasmLld (cc) | LinkerFlavor :: Unix (cc) => Box :: new (GccLinker { cmd , sess , target_cpu , hinted_static : None , is_ld : cc == Cc :: No , is_gnu : flavor . is_gnu () , uses_lld : flavor . uses_lld () , }) as Box < dyn Linker > , LinkerFlavor :: Msvc (..) => Box :: new (MsvcLinker { cmd , sess }) as Box < dyn Linker > , LinkerFlavor :: EmCc => Box :: new (EmLinker { cmd , sess }) as Box < dyn Linker > , LinkerFlavor :: Bpf => Box :: new (BpfLinker { cmd , sess }) as Box < dyn Linker > , LinkerFlavor :: Llbc => Box :: new (LlbcLinker { cmd , sess }) as Box < dyn Linker > , LinkerFlavor :: Ptx => Box :: new (PtxLinker { cmd , sess }) as Box < dyn Linker > , } }
}

macro_rules! verbatim_args_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function verbatim_args in module {}", module_path!());
    };
}

mkfn!{
    verbatim_args_introspect!();
    #[doc = " Just pass the arguments to the linker as is."] #[doc = " It is assumed that they are correctly prepared in advance."] fn verbatim_args < L : Linker + ? Sized > (l : & mut L , args : impl IntoIterator < Item : AsRef < OsStr > > ,) -> & mut L { for arg in args { l . cmd () . arg (arg) ; } l }
}

macro_rules! convert_link_args_to_cc_args_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function convert_link_args_to_cc_args in module {}", module_path!());
    };
}

mkfn!{
    convert_link_args_to_cc_args_introspect!();
    #[doc = " Add underlying linker arguments to C compiler command, by wrapping them in"] #[doc = " `-Wl` or `-Xlinker`."] fn convert_link_args_to_cc_args (cmd : & mut Command , args : impl IntoIterator < Item : AsRef < OsStr > >) { let mut combined_arg = OsString :: from ("-Wl") ; for arg in args { if arg . as_ref () . as_encoded_bytes () . contains (& b',') { if combined_arg != OsStr :: new ("-Wl") { cmd . arg (combined_arg) ; combined_arg = OsString :: from ("-Wl") ; } cmd . arg ("-Xlinker") ; cmd . arg (arg) ; } else { combined_arg . push (",") ; combined_arg . push (arg) ; } } if combined_arg != OsStr :: new ("-Wl") { cmd . arg (combined_arg) ; } }
}

macro_rules! link_args_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function link_args in module {}", module_path!());
    };
}

mkfn!{
    link_args_introspect!();
    #[doc = " Arguments for the underlying linker."] #[doc = " Add options to pass them through cc wrapper if `Linker` is a cc wrapper."] fn link_args < L : Linker + ? Sized > (l : & mut L , args : impl IntoIterator < Item : AsRef < OsStr > >) -> & mut L { if ! l . is_cc () { verbatim_args (l , args) ; } else { convert_link_args_to_cc_args (l . cmd () , args) ; } l }
}

macro_rules! cc_args_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function cc_args in module {}", module_path!());
    };
}

mkfn!{
    cc_args_introspect!();
    #[doc = " Arguments for the cc wrapper specifically."] #[doc = " Check that it's indeed a cc wrapper and pass verbatim."] fn cc_args < L : Linker + ? Sized > (l : & mut L , args : impl IntoIterator < Item : AsRef < OsStr > >) -> & mut L { assert ! (l . is_cc ()) ; verbatim_args (l , args) }
}

macro_rules! link_or_cc_args_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function link_or_cc_args in module {}", module_path!());
    };
}

mkfn!{
    link_or_cc_args_introspect!();
    #[doc = " Arguments supported by both underlying linker and cc wrapper, pass verbatim."] fn link_or_cc_args < L : Linker + ? Sized > (l : & mut L , args : impl IntoIterator < Item : AsRef < OsStr > > ,) -> & mut L { verbatim_args (l , args) }
}
mkitem!{macro_rules ! generate_arg_methods { ($ ($ ty : ty) *) => { $ (impl $ ty { #[allow (unused)] pub (crate) fn verbatim_args (& mut self , args : impl IntoIterator < Item : AsRef < OsStr >>) -> & mut Self { verbatim_args (self , args) } #[allow (unused)] pub (crate) fn verbatim_arg (& mut self , arg : impl AsRef < OsStr >) -> & mut Self { verbatim_args (self , iter :: once (arg)) } #[allow (unused)] pub (crate) fn link_args (& mut self , args : impl IntoIterator < Item : AsRef < OsStr >>) -> & mut Self { link_args (self , args) } #[allow (unused)] pub (crate) fn link_arg (& mut self , arg : impl AsRef < OsStr >) -> & mut Self { link_args (self , iter :: once (arg)) } #[allow (unused)] pub (crate) fn cc_args (& mut self , args : impl IntoIterator < Item : AsRef < OsStr >>) -> & mut Self { cc_args (self , args) } #[allow (unused)] pub (crate) fn cc_arg (& mut self , arg : impl AsRef < OsStr >) -> & mut Self { cc_args (self , iter :: once (arg)) } #[allow (unused)] pub (crate) fn link_or_cc_args (& mut self , args : impl IntoIterator < Item : AsRef < OsStr >>) -> & mut Self { link_or_cc_args (self , args) } #[allow (unused)] pub (crate) fn link_or_cc_arg (& mut self , arg : impl AsRef < OsStr >) -> & mut Self { link_or_cc_args (self , iter :: once (arg)) } }) * } }}
mkitem!{generate_arg_methods ! { GccLinker <'_ > MsvcLinker <'_ > EmLinker <'_ > WasmLd <'_ > L4Bender <'_ > AixLinker <'_ > LlbcLinker <'_ > PtxLinker <'_ > BpfLinker <'_ > dyn Linker + '_ }}
mkitem!{mktrait!{#[doc = " Linker abstraction used by `back::link` to build up the command to invoke a"] #[doc = " linker."] #[doc = ""] #[doc = " This trait is the total list of requirements needed by `back::link` and"] #[doc = " represents the meaning of each option being passed down. This trait is then"] #[doc = " used to dispatch on whether a GNU-like linker (generally `ld.exe`) or an"] #[doc = " MSVC linker (e.g., `link.exe`) is being used."] pub (crate) trait Linker { fn cmd (& mut self) -> & mut Command ; fn is_cc (& self) -> bool { false } fn set_output_kind (& mut self , output_kind : LinkOutputKind , crate_type : CrateType , out_filename : & Path ,) ; fn link_dylib_by_name (& mut self , _name : & str , _verbatim : bool , _as_needed : bool) { bug ! ("dylib linked with unsupported linker") } fn link_dylib_by_path (& mut self , _path : & Path , _as_needed : bool) { bug ! ("dylib linked with unsupported linker") } fn link_framework_by_name (& mut self , _name : & str , _verbatim : bool , _as_needed : bool) { bug ! ("framework linked with unsupported linker") } fn link_staticlib_by_name (& mut self , name : & str , verbatim : bool , whole_archive : bool) ; fn link_staticlib_by_path (& mut self , path : & Path , whole_archive : bool) ; fn include_path (& mut self , path : & Path) { link_or_cc_args (link_or_cc_args (self , & ["-L"]) , & [path]) ; } fn framework_path (& mut self , _path : & Path) { bug ! ("framework path set with unsupported linker") } fn output_filename (& mut self , path : & Path) { link_or_cc_args (link_or_cc_args (self , & ["-o"]) , & [path]) ; } fn add_object (& mut self , path : & Path) { link_or_cc_args (self , & [path]) ; } fn gc_sections (& mut self , keep_metadata : bool) ; fn full_relro (& mut self) ; fn partial_relro (& mut self) ; fn no_relro (& mut self) ; fn optimize (& mut self) ; fn pgo_gen (& mut self) ; fn control_flow_guard (& mut self) ; fn ehcont_guard (& mut self) ; fn debuginfo (& mut self , strip : Strip , natvis_debugger_visualizers : & [PathBuf]) ; fn no_crt_objects (& mut self) ; fn no_default_libraries (& mut self) ; fn export_symbols (& mut self , tmpdir : & Path , crate_type : CrateType , symbols : & [(String , SymbolExportKind)] ,) ; fn subsystem (& mut self , subsystem : & str) ; fn linker_plugin_lto (& mut self) ; fn add_eh_frame_header (& mut self) { } fn add_no_exec (& mut self) { } fn add_as_needed (& mut self) { } fn reset_per_library_state (& mut self) { } }}}
mkitem!{mkimpl!{impl dyn Linker + '_ { pub (crate) fn take_cmd (& mut self) -> Command { mem :: replace (self . cmd () , Command :: new ("")) } }}}
mkitem!{mkstruct!{struct GccLinker < 'a > { cmd : Command , sess : & 'a Session , target_cpu : & 'a str , hinted_static : Option < bool > , is_ld : bool , is_gnu : bool , uses_lld : bool , }}}
mkitem!{mkimpl!{impl < 'a > GccLinker < 'a > { fn takes_hints (& self) -> bool { ! self . sess . target . is_like_darwin && ! self . sess . target . is_like_wasm } fn hint_static (& mut self) { if ! self . takes_hints () { return ; } if self . hinted_static != Some (true) { self . link_arg ("-Bstatic") ; self . hinted_static = Some (true) ; } } fn hint_dynamic (& mut self) { if ! self . takes_hints () { return ; } if self . hinted_static != Some (false) { self . link_arg ("-Bdynamic") ; self . hinted_static = Some (false) ; } } fn push_linker_plugin_lto_args (& mut self , plugin_path : Option < & OsStr >) { if let Some (plugin_path) = plugin_path { let mut arg = OsString :: from ("-plugin=") ; arg . push (plugin_path) ; self . link_arg (& arg) ; } let opt_level = match self . sess . opts . optimize { config :: OptLevel :: No => "O0" , config :: OptLevel :: Less => "O1" , config :: OptLevel :: More | config :: OptLevel :: Size | config :: OptLevel :: SizeMin => "O2" , config :: OptLevel :: Aggressive => "O3" , } ; if let Some (path) = & self . sess . opts . unstable_opts . profile_sample_use { self . link_arg (& format ! ("-plugin-opt=sample-profile={}" , path . display ())) ; } ; self . link_args (& [& format ! ("-plugin-opt={opt_level}") , & format ! ("-plugin-opt=mcpu={}" , self . target_cpu) ,]) ; } fn build_dylib (& mut self , crate_type : CrateType , out_filename : & Path) { if self . sess . target . is_like_darwin { if self . is_cc () { self . cc_arg ("-dynamiclib") ; } else { self . link_arg ("-dylib") ; } if self . sess . opts . cg . rpath || self . sess . opts . unstable_opts . osx_rpath_install_name { let mut rpath = OsString :: from ("@rpath/") ; rpath . push (out_filename . file_name () . unwrap ()) ; self . link_arg ("-install_name") . link_arg (rpath) ; } } else { self . link_or_cc_arg ("-shared") ; if let Some (name) = out_filename . file_name () { if self . sess . target . is_like_windows { let (prefix , suffix) = self . sess . staticlib_components (false) ; let mut implib_name = OsString :: from (prefix) ; implib_name . push (name) ; implib_name . push (suffix) ; let mut out_implib = OsString :: from ("--out-implib=") ; out_implib . push (out_filename . with_file_name (implib_name)) ; self . link_arg (out_implib) ; } else if crate_type == CrateType :: Dylib { let mut soname = OsString :: from ("-soname=") ; soname . push (name) ; self . link_arg (soname) ; } } } } fn with_as_needed (& mut self , as_needed : bool , f : impl FnOnce (& mut Self)) { if ! as_needed { if self . sess . target . is_like_darwin { self . sess . dcx () . emit_warn (errors :: Ld64UnimplementedModifier) ; } else if self . is_gnu && ! self . sess . target . is_like_windows { self . link_arg ("--no-as-needed") ; } else { self . sess . dcx () . emit_warn (errors :: LinkerUnsupportedModifier) ; } } f (self) ; if ! as_needed { if self . sess . target . is_like_darwin { } else if self . is_gnu && ! self . sess . target . is_like_windows { self . link_arg ("--as-needed") ; } } } }}}
mkitem!{mkimpl!{impl < 'a > Linker for GccLinker < 'a > { fn cmd (& mut self) -> & mut Command { & mut self . cmd } fn is_cc (& self) -> bool { ! self . is_ld } fn set_output_kind (& mut self , output_kind : LinkOutputKind , crate_type : CrateType , out_filename : & Path ,) { match output_kind { LinkOutputKind :: DynamicNoPicExe => { if ! self . is_ld && self . is_gnu { self . cc_arg ("-no-pie") ; } } LinkOutputKind :: DynamicPicExe => { if ! self . sess . target . is_like_windows { self . link_or_cc_arg ("-pie") ; } } LinkOutputKind :: StaticNoPicExe => { self . link_or_cc_arg ("-static") ; if ! self . is_ld && self . is_gnu { self . cc_arg ("-no-pie") ; } } LinkOutputKind :: StaticPicExe => { if ! self . is_ld { self . cc_arg ("-static-pie") ; } else { self . link_args (& ["-static" , "-pie" , "--no-dynamic-linker" , "-z" , "text"]) ; } } LinkOutputKind :: DynamicDylib => self . build_dylib (crate_type , out_filename) , LinkOutputKind :: StaticDylib => { self . link_or_cc_arg ("-static") ; self . build_dylib (crate_type , out_filename) ; } LinkOutputKind :: WasiReactorExe => { self . link_args (& ["--entry" , "_initialize"]) ; } } if self . sess . target . os == "vxworks" && matches ! (output_kind , LinkOutputKind :: StaticNoPicExe | LinkOutputKind :: StaticPicExe | LinkOutputKind :: StaticDylib) { self . cc_arg ("--static-crt") ; } if self . sess . target . arch == "avr" && ! self . uses_lld { self . verbatim_arg (format ! ("-mmcu={}" , self . target_cpu)) ; } } fn link_dylib_by_name (& mut self , name : & str , verbatim : bool , as_needed : bool) { if self . sess . target . os == "illumos" && name == "c" { return ; } self . hint_dynamic () ; self . with_as_needed (as_needed , | this | { let colon = if verbatim && this . is_gnu { ":" } else { "" } ; this . link_or_cc_arg (format ! ("-l{colon}{name}")) ; }) ; } fn link_dylib_by_path (& mut self , path : & Path , as_needed : bool) { self . hint_dynamic () ; self . with_as_needed (as_needed , | this | { this . link_or_cc_arg (path) ; }) } fn link_framework_by_name (& mut self , name : & str , _verbatim : bool , as_needed : bool) { self . hint_dynamic () ; if ! as_needed { self . sess . dcx () . emit_warn (errors :: Ld64UnimplementedModifier) ; } self . link_or_cc_args (& ["-framework" , name]) ; } fn link_staticlib_by_name (& mut self , name : & str , verbatim : bool , whole_archive : bool) { self . hint_static () ; let colon = if verbatim && self . is_gnu { ":" } else { "" } ; if ! whole_archive { self . link_or_cc_arg (format ! ("-l{colon}{name}")) ; } else if self . sess . target . is_like_darwin { self . link_arg ("-force_load") ; self . link_arg (find_native_static_library (name , verbatim , self . sess)) ; } else { self . link_arg ("--whole-archive") . link_or_cc_arg (format ! ("-l{colon}{name}")) . link_arg ("--no-whole-archive") ; } } fn link_staticlib_by_path (& mut self , path : & Path , whole_archive : bool) { self . hint_static () ; if ! whole_archive { self . link_or_cc_arg (path) ; } else if self . sess . target . is_like_darwin { self . link_arg ("-force_load") . link_arg (path) ; } else { self . link_arg ("--whole-archive") . link_arg (path) . link_arg ("--no-whole-archive") ; } } fn framework_path (& mut self , path : & Path) { self . link_or_cc_arg ("-F") . link_or_cc_arg (path) ; } fn full_relro (& mut self) { self . link_args (& ["-z" , "relro" , "-z" , "now"]) ; } fn partial_relro (& mut self) { self . link_args (& ["-z" , "relro"]) ; } fn no_relro (& mut self) { self . link_args (& ["-z" , "norelro"]) ; } fn gc_sections (& mut self , keep_metadata : bool) { if self . sess . target . is_like_darwin { self . link_arg ("-dead_strip") ; } else if (self . is_gnu || self . sess . target . is_like_wasm) && ! keep_metadata { self . link_arg ("--gc-sections") ; } } fn optimize (& mut self) { if ! self . is_gnu && ! self . sess . target . is_like_wasm { return ; } if self . sess . opts . optimize == config :: OptLevel :: More || self . sess . opts . optimize == config :: OptLevel :: Aggressive { self . link_arg ("-O1") ; } } fn pgo_gen (& mut self) { if ! self . is_gnu { return ; } self . link_or_cc_args (& ["-u" , "__llvm_profile_runtime"]) ; } fn control_flow_guard (& mut self) { } fn ehcont_guard (& mut self) { } fn debuginfo (& mut self , strip : Strip , _ : & [PathBuf]) { if self . sess . target . is_like_darwin { return ; } match strip { Strip :: None => { } Strip :: Debuginfo => { if ! self . sess . target . is_like_solaris { self . link_arg ("--strip-debug") ; } } Strip :: Symbols => { self . link_arg ("--strip-all") ; } } match self . sess . opts . unstable_opts . debuginfo_compression { config :: DebugInfoCompression :: None => { } config :: DebugInfoCompression :: Zlib => { self . link_arg ("--compress-debug-sections=zlib") ; } config :: DebugInfoCompression :: Zstd => { self . link_arg ("--compress-debug-sections=zstd") ; } } } fn no_crt_objects (& mut self) { if ! self . is_ld { self . cc_arg ("-nostartfiles") ; } } fn no_default_libraries (& mut self) { if ! self . is_ld { self . cc_arg ("-nodefaultlibs") ; } } fn export_symbols (& mut self , tmpdir : & Path , crate_type : CrateType , symbols : & [(String , SymbolExportKind)] ,) { if crate_type == CrateType :: Executable { let should_export_executable_symbols = self . sess . opts . unstable_opts . export_executable_symbols ; if self . sess . target . override_export_symbols . is_none () && ! should_export_executable_symbols { return ; } } if ! self . sess . target . limit_rdylib_exports { return ; } let path = tmpdir . join (if self . sess . target . is_like_windows { "list.def" } else { "list" }) ; debug ! ("EXPORTED SYMBOLS:") ; if self . sess . target . is_like_darwin { let res : io :: Result < () > = try { let mut f = File :: create_buffered (& path) ? ; for (sym , _) in symbols { debug ! ("  _{sym}") ; writeln ! (f , "_{sym}") ? ; } } ; if let Err (error) = res { self . sess . dcx () . emit_fatal (errors :: LibDefWriteFailure { error }) ; } self . link_arg ("-exported_symbols_list") . link_arg (path) ; } else if self . sess . target . is_like_windows { let res : io :: Result < () > = try { let mut f = File :: create_buffered (& path) ? ; writeln ! (f , "EXPORTS") ? ; for (symbol , kind) in symbols { let kind_marker = if * kind == SymbolExportKind :: Data { " DATA" } else { "" } ; debug ! ("  _{symbol}") ; writeln ! (f , "  \"{symbol}\"{kind_marker}") ? ; } } ; if let Err (error) = res { self . sess . dcx () . emit_fatal (errors :: LibDefWriteFailure { error }) ; } self . link_arg (path) ; } else if crate_type == CrateType :: Executable && ! self . sess . target . is_like_solaris { let res : io :: Result < () > = try { let mut f = File :: create_buffered (& path) ? ; writeln ! (f , "{{") ? ; for (sym , _) in symbols { debug ! (sym) ; writeln ! (f , "  {sym};") ? ; } writeln ! (f , "}};") ? ; } ; if let Err (error) = res { self . sess . dcx () . emit_fatal (errors :: VersionScriptWriteFailure { error }) ; } self . link_arg ("--dynamic-list") . link_arg (path) ; } else { let res : io :: Result < () > = try { let mut f = File :: create_buffered (& path) ? ; writeln ! (f , "{{") ? ; if ! symbols . is_empty () { writeln ! (f , "  global:") ? ; for (sym , _) in symbols { debug ! ("    {sym};") ; writeln ! (f , "    {sym};") ? ; } } writeln ! (f , "\n  local:\n    *;\n}};") ? ; } ; if let Err (error) = res { self . sess . dcx () . emit_fatal (errors :: VersionScriptWriteFailure { error }) ; } if self . sess . target . is_like_solaris { self . link_arg ("-M") . link_arg (path) ; } else { let mut arg = OsString :: from ("--version-script=") ; arg . push (path) ; self . link_arg (arg) . link_arg ("--no-undefined-version") ; } } } fn subsystem (& mut self , subsystem : & str) { self . link_args (& ["--subsystem" , subsystem]) ; } fn reset_per_library_state (& mut self) { self . hint_dynamic () ; } fn linker_plugin_lto (& mut self) { match self . sess . opts . cg . linker_plugin_lto { LinkerPluginLto :: Disabled => { } LinkerPluginLto :: LinkerPluginAuto => { self . push_linker_plugin_lto_args (None) ; } LinkerPluginLto :: LinkerPlugin (ref path) => { self . push_linker_plugin_lto_args (Some (path . as_os_str ())) ; } } } fn add_eh_frame_header (& mut self) { self . link_arg ("--eh-frame-hdr") ; } fn add_no_exec (& mut self) { if self . sess . target . is_like_windows { self . link_arg ("--nxcompat") ; } else if self . is_gnu { self . link_args (& ["-z" , "noexecstack"]) ; } } fn add_as_needed (& mut self) { if self . is_gnu && ! self . sess . target . is_like_windows { self . link_arg ("--as-needed") ; } else if self . sess . target . is_like_solaris { self . link_args (& ["-z" , "ignore"]) ; } } }}}
mkitem!{mkstruct!{struct MsvcLinker < 'a > { cmd : Command , sess : & 'a Session , }}}
mkitem!{mkimpl!{impl < 'a > Linker for MsvcLinker < 'a > { fn cmd (& mut self) -> & mut Command { & mut self . cmd } fn set_output_kind (& mut self , output_kind : LinkOutputKind , _crate_type : CrateType , out_filename : & Path ,) { match output_kind { LinkOutputKind :: DynamicNoPicExe | LinkOutputKind :: DynamicPicExe | LinkOutputKind :: StaticNoPicExe | LinkOutputKind :: StaticPicExe => { } LinkOutputKind :: DynamicDylib | LinkOutputKind :: StaticDylib => { self . link_arg ("/DLL") ; let mut arg : OsString = "/IMPLIB:" . into () ; arg . push (out_filename . with_extension ("dll.lib")) ; self . link_arg (arg) ; } LinkOutputKind :: WasiReactorExe => { panic ! ("can't link as reactor on non-wasi target") ; } } } fn link_dylib_by_name (& mut self , name : & str , verbatim : bool , _as_needed : bool) { if let Some (path) = try_find_native_dynamic_library (self . sess , name , verbatim) { self . link_arg (path) ; } else { self . link_arg (format ! ("{}{}" , name , if verbatim { "" } else { ".lib" })) ; } } fn link_dylib_by_path (& mut self , path : & Path , _as_needed : bool) { let implib_path = path . with_extension ("dll.lib") ; if implib_path . exists () { self . link_or_cc_arg (implib_path) ; } } fn link_staticlib_by_name (& mut self , name : & str , verbatim : bool , whole_archive : bool) { if let Some (path) = try_find_native_static_library (self . sess , name , verbatim) { self . link_staticlib_by_path (& path , whole_archive) ; } else { let opts = if whole_archive { "/WHOLEARCHIVE:" } else { "" } ; let (prefix , suffix) = self . sess . staticlib_components (verbatim) ; self . link_arg (format ! ("{opts}{prefix}{name}{suffix}")) ; } } fn link_staticlib_by_path (& mut self , path : & Path , whole_archive : bool) { if ! whole_archive { self . link_arg (path) ; } else { let mut arg = OsString :: from ("/WHOLEARCHIVE:") ; arg . push (path) ; self . link_arg (arg) ; } } fn gc_sections (& mut self , _keep_metadata : bool) { if self . sess . opts . optimize != config :: OptLevel :: No { self . link_arg ("/OPT:REF,ICF") ; } else { self . link_arg ("/OPT:REF,NOICF") ; } } fn full_relro (& mut self) { } fn partial_relro (& mut self) { } fn no_relro (& mut self) { } fn no_crt_objects (& mut self) { } fn no_default_libraries (& mut self) { self . link_arg ("/NODEFAULTLIB") ; } fn include_path (& mut self , path : & Path) { let mut arg = OsString :: from ("/LIBPATH:") ; arg . push (path) ; self . link_arg (& arg) ; } fn output_filename (& mut self , path : & Path) { let mut arg = OsString :: from ("/OUT:") ; arg . push (path) ; self . link_arg (& arg) ; } fn optimize (& mut self) { } fn pgo_gen (& mut self) { } fn control_flow_guard (& mut self) { self . link_arg ("/guard:cf") ; } fn ehcont_guard (& mut self) { if self . sess . target . pointer_width == 64 { self . link_arg ("/guard:ehcont") ; } } fn debuginfo (& mut self , _strip : Strip , natvis_debugger_visualizers : & [PathBuf]) { self . link_arg ("/DEBUG") ; self . link_arg ("/PDBALTPATH:%_PDB%") ; let natvis_dir_path = self . sess . opts . sysroot . path () . join ("lib\\rustlib\\etc") ; if let Ok (natvis_dir) = fs :: read_dir (& natvis_dir_path) { for entry in natvis_dir { match entry { Ok (entry) => { let path = entry . path () ; if path . extension () == Some ("natvis" . as_ref ()) { let mut arg = OsString :: from ("/NATVIS:") ; arg . push (path) ; self . link_arg (arg) ; } } Err (error) => { self . sess . dcx () . emit_warn (errors :: NoNatvisDirectory { error }) ; } } } } for path in natvis_debugger_visualizers { let mut arg = OsString :: from ("/NATVIS:") ; arg . push (path) ; self . link_arg (arg) ; } } fn export_symbols (& mut self , tmpdir : & Path , crate_type : CrateType , symbols : & [(String , SymbolExportKind)] ,) { if crate_type == CrateType :: Executable { let should_export_executable_symbols = self . sess . opts . unstable_opts . export_executable_symbols ; if ! should_export_executable_symbols { return ; } } let path = tmpdir . join ("lib.def") ; let res : io :: Result < () > = try { let mut f = File :: create_buffered (& path) ? ; writeln ! (f , "LIBRARY") ? ; writeln ! (f , "EXPORTS") ? ; for (symbol , kind) in symbols { let kind_marker = if * kind == SymbolExportKind :: Data { " DATA" } else { "" } ; debug ! ("  _{symbol}") ; writeln ! (f , "  {symbol}{kind_marker}") ? ; } } ; if let Err (error) = res { self . sess . dcx () . emit_fatal (errors :: LibDefWriteFailure { error }) ; } let mut arg = OsString :: from ("/DEF:") ; arg . push (path) ; self . link_arg (& arg) ; } fn subsystem (& mut self , subsystem : & str) { self . link_arg (& format ! ("/SUBSYSTEM:{subsystem}")) ; if subsystem == "windows" { self . link_arg ("/ENTRY:mainCRTStartup") ; } } fn linker_plugin_lto (& mut self) { } fn add_no_exec (& mut self) { self . link_arg ("/NXCOMPAT") ; } }}}
mkitem!{mkstruct!{struct EmLinker < 'a > { cmd : Command , sess : & 'a Session , }}}
mkitem!{mkimpl!{impl < 'a > Linker for EmLinker < 'a > { fn cmd (& mut self) -> & mut Command { & mut self . cmd } fn is_cc (& self) -> bool { true } fn set_output_kind (& mut self , _output_kind : LinkOutputKind , _crate_type : CrateType , _out_filename : & Path ,) { } fn link_dylib_by_name (& mut self , name : & str , _verbatim : bool , _as_needed : bool) { self . link_or_cc_args (& ["-l" , name]) ; } fn link_dylib_by_path (& mut self , path : & Path , _as_needed : bool) { self . link_or_cc_arg (path) ; } fn link_staticlib_by_name (& mut self , name : & str , _verbatim : bool , _whole_archive : bool) { self . link_or_cc_args (& ["-l" , name]) ; } fn link_staticlib_by_path (& mut self , path : & Path , _whole_archive : bool) { self . link_or_cc_arg (path) ; } fn full_relro (& mut self) { } fn partial_relro (& mut self) { } fn no_relro (& mut self) { } fn gc_sections (& mut self , _keep_metadata : bool) { } fn optimize (& mut self) { self . cc_arg (match self . sess . opts . optimize { OptLevel :: No => "-O0" , OptLevel :: Less => "-O1" , OptLevel :: More => "-O2" , OptLevel :: Aggressive => "-O3" , OptLevel :: Size => "-Os" , OptLevel :: SizeMin => "-Oz" , }) ; } fn pgo_gen (& mut self) { } fn control_flow_guard (& mut self) { } fn ehcont_guard (& mut self) { } fn debuginfo (& mut self , _strip : Strip , _ : & [PathBuf]) { self . cc_arg (match self . sess . opts . debuginfo { DebugInfo :: None => "-g0" , DebugInfo :: Limited | DebugInfo :: LineTablesOnly | DebugInfo :: LineDirectivesOnly => { "--profiling-funcs" } DebugInfo :: Full => "-g" , }) ; } fn no_crt_objects (& mut self) { } fn no_default_libraries (& mut self) { self . cc_arg ("-nodefaultlibs") ; } fn export_symbols (& mut self , _tmpdir : & Path , _crate_type : CrateType , symbols : & [(String , SymbolExportKind)] ,) { debug ! ("EXPORTED SYMBOLS:") ; self . cc_arg ("-s") ; let mut arg = OsString :: from ("EXPORTED_FUNCTIONS=") ; let encoded = serde_json :: to_string (& symbols . iter () . map (| (sym , _) | "_" . to_owned () + sym) . collect :: < Vec < _ > > () ,) . unwrap () ; debug ! ("{encoded}") ; arg . push (encoded) ; self . cc_arg (arg) ; } fn subsystem (& mut self , _subsystem : & str) { } fn linker_plugin_lto (& mut self) { } }}}
mkitem!{mkstruct!{struct WasmLd < 'a > { cmd : Command , sess : & 'a Session , }}}
mkitem!{mkimpl!{impl < 'a > WasmLd < 'a > { fn new (cmd : Command , sess : & 'a Session) -> WasmLd < 'a > { let mut wasm_ld = WasmLd { cmd , sess } ; if sess . target_features . contains (& sym :: atomics) { wasm_ld . link_args (& ["--shared-memory" , "--max-memory=1073741824" , "--import-memory"]) ; if sess . target . os == "unknown" || sess . target . os == "none" { wasm_ld . link_args (& ["--export=__wasm_init_tls" , "--export=__tls_size" , "--export=__tls_align" , "--export=__tls_base" ,]) ; } } wasm_ld } }}}
mkitem!{mkimpl!{impl < 'a > Linker for WasmLd < 'a > { fn cmd (& mut self) -> & mut Command { & mut self . cmd } fn set_output_kind (& mut self , output_kind : LinkOutputKind , _crate_type : CrateType , _out_filename : & Path ,) { match output_kind { LinkOutputKind :: DynamicNoPicExe | LinkOutputKind :: DynamicPicExe | LinkOutputKind :: StaticNoPicExe | LinkOutputKind :: StaticPicExe => { } LinkOutputKind :: DynamicDylib | LinkOutputKind :: StaticDylib => { self . link_arg ("--no-entry") ; } LinkOutputKind :: WasiReactorExe => { self . link_args (& ["--entry" , "_initialize"]) ; } } } fn link_dylib_by_name (& mut self , name : & str , _verbatim : bool , _as_needed : bool) { self . link_or_cc_args (& ["-l" , name]) ; } fn link_dylib_by_path (& mut self , path : & Path , _as_needed : bool) { self . link_or_cc_arg (path) ; } fn link_staticlib_by_name (& mut self , name : & str , _verbatim : bool , whole_archive : bool) { if ! whole_archive { self . link_or_cc_args (& ["-l" , name]) ; } else { self . link_arg ("--whole-archive") . link_or_cc_args (& ["-l" , name]) . link_arg ("--no-whole-archive") ; } } fn link_staticlib_by_path (& mut self , path : & Path , whole_archive : bool) { if ! whole_archive { self . link_or_cc_arg (path) ; } else { self . link_arg ("--whole-archive") . link_or_cc_arg (path) . link_arg ("--no-whole-archive") ; } } fn full_relro (& mut self) { } fn partial_relro (& mut self) { } fn no_relro (& mut self) { } fn gc_sections (& mut self , _keep_metadata : bool) { self . link_arg ("--gc-sections") ; } fn optimize (& mut self) { self . link_arg (match self . sess . opts . optimize { OptLevel :: No => "-O0" , OptLevel :: Less => "-O1" , OptLevel :: More => "-O2" , OptLevel :: Aggressive => "-O3" , OptLevel :: Size => "-O2" , OptLevel :: SizeMin => "-O2" , }) ; } fn pgo_gen (& mut self) { } fn debuginfo (& mut self , strip : Strip , _ : & [PathBuf]) { match strip { Strip :: None => { } Strip :: Debuginfo => { self . link_arg ("--strip-debug") ; } Strip :: Symbols => { self . link_arg ("--strip-all") ; } } } fn control_flow_guard (& mut self) { } fn ehcont_guard (& mut self) { } fn no_crt_objects (& mut self) { } fn no_default_libraries (& mut self) { } fn export_symbols (& mut self , _tmpdir : & Path , _crate_type : CrateType , symbols : & [(String , SymbolExportKind)] ,) { for (sym , _) in symbols { self . link_args (& ["--export" , sym]) ; } if self . sess . target . os == "unknown" || self . sess . target . os == "none" { self . link_args (& ["--export=__heap_base" , "--export=__data_end"]) ; } } fn subsystem (& mut self , _subsystem : & str) { } fn linker_plugin_lto (& mut self) { match self . sess . opts . cg . linker_plugin_lto { LinkerPluginLto :: Disabled => { } LinkerPluginLto :: LinkerPluginAuto => { self . push_linker_plugin_lto_args () ; } LinkerPluginLto :: LinkerPlugin (_) => { self . push_linker_plugin_lto_args () ; } } } }}}
mkitem!{mkimpl!{impl < 'a > WasmLd < 'a > { fn push_linker_plugin_lto_args (& mut self) { let opt_level = match self . sess . opts . optimize { config :: OptLevel :: No => "O0" , config :: OptLevel :: Less => "O1" , config :: OptLevel :: More => "O2" , config :: OptLevel :: Aggressive => "O3" , config :: OptLevel :: Size | config :: OptLevel :: SizeMin => "O2" , } ; self . link_arg (& format ! ("--lto-{opt_level}")) ; } }}}
mkitem!{mkstruct!{#[doc = " Linker shepherd script for L4Re (Fiasco)"] struct L4Bender < 'a > { cmd : Command , sess : & 'a Session , hinted_static : bool , }}}
mkitem!{mkimpl!{impl < 'a > Linker for L4Bender < 'a > { fn cmd (& mut self) -> & mut Command { & mut self . cmd } fn set_output_kind (& mut self , _output_kind : LinkOutputKind , _crate_type : CrateType , _out_filename : & Path ,) { } fn link_staticlib_by_name (& mut self , name : & str , _verbatim : bool , whole_archive : bool) { self . hint_static () ; if ! whole_archive { self . link_arg (format ! ("-PC{name}")) ; } else { self . link_arg ("--whole-archive") . link_or_cc_arg (format ! ("-l{name}")) . link_arg ("--no-whole-archive") ; } } fn link_staticlib_by_path (& mut self , path : & Path , whole_archive : bool) { self . hint_static () ; if ! whole_archive { self . link_or_cc_arg (path) ; } else { self . link_arg ("--whole-archive") . link_or_cc_arg (path) . link_arg ("--no-whole-archive") ; } } fn full_relro (& mut self) { self . link_args (& ["-z" , "relro" , "-z" , "now"]) ; } fn partial_relro (& mut self) { self . link_args (& ["-z" , "relro"]) ; } fn no_relro (& mut self) { self . link_args (& ["-z" , "norelro"]) ; } fn gc_sections (& mut self , keep_metadata : bool) { if ! keep_metadata { self . link_arg ("--gc-sections") ; } } fn optimize (& mut self) { if self . sess . opts . optimize == config :: OptLevel :: More || self . sess . opts . optimize == config :: OptLevel :: Aggressive { self . link_arg ("-O1") ; } } fn pgo_gen (& mut self) { } fn debuginfo (& mut self , strip : Strip , _ : & [PathBuf]) { match strip { Strip :: None => { } Strip :: Debuginfo => { self . link_arg ("--strip-debug") ; } Strip :: Symbols => { self . link_arg ("--strip-all") ; } } } fn no_default_libraries (& mut self) { self . cc_arg ("-nostdlib") ; } fn export_symbols (& mut self , _ : & Path , _ : CrateType , _ : & [(String , SymbolExportKind)]) { self . sess . dcx () . emit_warn (errors :: L4BenderExportingSymbolsUnimplemented) ; } fn subsystem (& mut self , subsystem : & str) { self . link_arg (& format ! ("--subsystem {subsystem}")) ; } fn reset_per_library_state (& mut self) { self . hint_static () ; } fn linker_plugin_lto (& mut self) { } fn control_flow_guard (& mut self) { } fn ehcont_guard (& mut self) { } fn no_crt_objects (& mut self) { } }}}
mkitem!{mkimpl!{impl < 'a > L4Bender < 'a > { fn new (cmd : Command , sess : & 'a Session) -> L4Bender < 'a > { L4Bender { cmd , sess , hinted_static : false } } fn hint_static (& mut self) { if ! self . hinted_static { self . link_or_cc_arg ("-static") ; self . hinted_static = true ; } } }}}
mkitem!{mkstruct!{#[doc = " Linker for AIX."] struct AixLinker < 'a > { cmd : Command , sess : & 'a Session , hinted_static : Option < bool > , }}}
mkitem!{mkimpl!{impl < 'a > AixLinker < 'a > { fn new (cmd : Command , sess : & 'a Session) -> AixLinker < 'a > { AixLinker { cmd , sess , hinted_static : None } } fn hint_static (& mut self) { if self . hinted_static != Some (true) { self . link_arg ("-bstatic") ; self . hinted_static = Some (true) ; } } fn hint_dynamic (& mut self) { if self . hinted_static != Some (false) { self . link_arg ("-bdynamic") ; self . hinted_static = Some (false) ; } } fn build_dylib (& mut self , _out_filename : & Path) { self . link_args (& ["-bM:SRE" , "-bnoentry"]) ; self . link_arg ("-bexpfull") ; } }}}
mkitem!{mkimpl!{impl < 'a > Linker for AixLinker < 'a > { fn cmd (& mut self) -> & mut Command { & mut self . cmd } fn set_output_kind (& mut self , output_kind : LinkOutputKind , _crate_type : CrateType , out_filename : & Path ,) { match output_kind { LinkOutputKind :: DynamicDylib => { self . hint_dynamic () ; self . build_dylib (out_filename) ; } LinkOutputKind :: StaticDylib => { self . hint_static () ; self . build_dylib (out_filename) ; } _ => { } } } fn link_dylib_by_name (& mut self , name : & str , verbatim : bool , _as_needed : bool) { self . hint_dynamic () ; self . link_or_cc_arg (if verbatim { String :: from (name) } else { format ! ("-l{name}") }) ; } fn link_dylib_by_path (& mut self , path : & Path , _as_needed : bool) { self . hint_dynamic () ; self . link_or_cc_arg (path) ; } fn link_staticlib_by_name (& mut self , name : & str , verbatim : bool , whole_archive : bool) { self . hint_static () ; if ! whole_archive { self . link_or_cc_arg (if verbatim { String :: from (name) } else { format ! ("-l{name}") }) ; } else { let mut arg = OsString :: from ("-bkeepfile:") ; arg . push (find_native_static_library (name , verbatim , self . sess)) ; self . link_or_cc_arg (arg) ; } } fn link_staticlib_by_path (& mut self , path : & Path , whole_archive : bool) { self . hint_static () ; if ! whole_archive { self . link_or_cc_arg (path) ; } else { let mut arg = OsString :: from ("-bkeepfile:") ; arg . push (path) ; self . link_arg (arg) ; } } fn full_relro (& mut self) { } fn partial_relro (& mut self) { } fn no_relro (& mut self) { } fn gc_sections (& mut self , _keep_metadata : bool) { self . link_arg ("-bgc") ; } fn optimize (& mut self) { } fn pgo_gen (& mut self) { self . link_arg ("-bdbg:namedsects:ss") ; self . link_arg ("-u") ; self . link_arg ("__llvm_profile_runtime") ; } fn control_flow_guard (& mut self) { } fn ehcont_guard (& mut self) { } fn debuginfo (& mut self , _ : Strip , _ : & [PathBuf]) { } fn no_crt_objects (& mut self) { } fn no_default_libraries (& mut self) { } fn export_symbols (& mut self , tmpdir : & Path , _crate_type : CrateType , symbols : & [(String , SymbolExportKind)] ,) { let path = tmpdir . join ("list.exp") ; let res : io :: Result < () > = try { let mut f = File :: create_buffered (& path) ? ; for (symbol , _) in symbols { debug ! ("  _{symbol}") ; writeln ! (f , "  {symbol}") ? ; } } ; if let Err (e) = res { self . sess . dcx () . fatal (format ! ("failed to write export file: {e}")) ; } self . link_arg (format ! ("-bE:{}" , path . to_str () . unwrap ())) ; } fn subsystem (& mut self , _subsystem : & str) { } fn reset_per_library_state (& mut self) { self . hint_dynamic () ; } fn linker_plugin_lto (& mut self) { } fn add_eh_frame_header (& mut self) { } fn add_no_exec (& mut self) { } fn add_as_needed (& mut self) { } }}}

macro_rules! for_each_exported_symbols_include_dep_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function for_each_exported_symbols_include_dep in module {}", module_path!());
    };
}

mkfn!{
    for_each_exported_symbols_include_dep_introspect!();
    fn for_each_exported_symbols_include_dep < 'tcx > (tcx : TyCtxt < 'tcx > , crate_type : CrateType , mut callback : impl FnMut (ExportedSymbol < 'tcx > , SymbolExportInfo , CrateNum) ,) { let formats = tcx . dependency_formats (()) ; let deps = & formats [& crate_type] ; for (cnum , dep_format) in deps . iter_enumerated () { if * dep_format == Linkage :: Static { for & (symbol , info) in tcx . exported_non_generic_symbols (cnum) . iter () { callback (symbol , info , cnum) ; } for & (symbol , info) in tcx . exported_generic_symbols (cnum) . iter () { callback (symbol , info , cnum) ; } } } }
}

macro_rules! exported_symbols_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function exported_symbols in module {}", module_path!());
    };
}

mkfn!{
    exported_symbols_introspect!();
    pub (crate) fn exported_symbols (tcx : TyCtxt < '_ > , crate_type : CrateType ,) -> Vec < (String , SymbolExportKind) > { if let Some (ref exports) = tcx . sess . target . override_export_symbols { return exports . iter () . map (| name | { (name . to_string () , SymbolExportKind :: Text ,) }) . collect () ; } let mut symbols = if let CrateType :: ProcMacro = crate_type { exported_symbols_for_proc_macro_crate (tcx) } else { exported_symbols_for_non_proc_macro (tcx , crate_type) } ; if crate_type == CrateType :: Dylib || crate_type == CrateType :: ProcMacro { let metadata_symbol_name = exported_symbols :: metadata_symbol_name (tcx) ; symbols . push ((metadata_symbol_name , SymbolExportKind :: Data)) ; } symbols }
}

macro_rules! exported_symbols_for_non_proc_macro_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function exported_symbols_for_non_proc_macro in module {}", module_path!());
    };
}

mkfn!{
    exported_symbols_for_non_proc_macro_introspect!();
    fn exported_symbols_for_non_proc_macro (tcx : TyCtxt < '_ > , crate_type : CrateType ,) -> Vec < (String , SymbolExportKind) > { let mut symbols = Vec :: new () ; let export_threshold = symbol_export :: crates_export_threshold (& [crate_type]) ; for_each_exported_symbols_include_dep (tcx , crate_type , | symbol , info , cnum | { if info . level . is_below_threshold (export_threshold) && ! tcx . is_compiler_builtins (cnum) { symbols . push ((symbol_export :: exporting_symbol_name_for_instance_in_crate (tcx , symbol , cnum) , info . kind ,)) ; symbol_export :: extend_exported_symbols (& mut symbols , tcx , symbol , cnum) ; } }) ; if export_threshold == SymbolExportLevel :: Rust && needs_allocator_shim_for_linking (tcx . dependency_formats (()) , crate_type) && tcx . allocator_kind (()) . is_some () { symbols . extend (allocator_shim_symbols (tcx)) ; } symbols }
}

macro_rules! exported_symbols_for_proc_macro_crate_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function exported_symbols_for_proc_macro_crate in module {}", module_path!());
    };
}

mkfn!{
    exported_symbols_for_proc_macro_crate_introspect!();
    fn exported_symbols_for_proc_macro_crate (tcx : TyCtxt < '_ >) -> Vec < (String , SymbolExportKind) > { if ! tcx . sess . opts . output_types . should_codegen () { return Vec :: new () ; } let stable_crate_id = tcx . stable_crate_id (LOCAL_CRATE) ; let proc_macro_decls_name = tcx . sess . generate_proc_macro_decls_symbol (stable_crate_id) ; vec ! [(proc_macro_decls_name , SymbolExportKind :: Data)] }
}

macro_rules! linked_symbols_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function linked_symbols in module {}", module_path!());
    };
}

mkfn!{
    linked_symbols_introspect!();
    pub (crate) fn linked_symbols (tcx : TyCtxt < '_ > , crate_type : CrateType ,) -> Vec < (String , SymbolExportKind) > { match crate_type { CrateType :: Executable | CrateType :: ProcMacro | CrateType :: Cdylib | CrateType :: Dylib | CrateType :: Sdylib => () , CrateType :: Staticlib | CrateType :: Rlib => { return Vec :: new () ; } } match tcx . sess . lto () { Lto :: No | Lto :: ThinLocal => { } Lto :: Thin | Lto :: Fat => { return Vec :: new () ; } } let mut symbols = Vec :: new () ; let export_threshold = symbol_export :: crates_export_threshold (& [crate_type]) ; for_each_exported_symbols_include_dep (tcx , crate_type , | symbol , info , cnum | { if info . level . is_below_threshold (export_threshold) && ! tcx . is_compiler_builtins (cnum) || info . used || info . rustc_std_internal_symbol { symbols . push ((symbol_export :: linking_symbol_name_for_instance_in_crate (tcx , symbol , info . kind , cnum ,) , info . kind ,)) ; } }) ; symbols }
}
mkitem!{mkstruct!{#[doc = " Much simplified and explicit CLI for the NVPTX linker. The linker operates"] #[doc = " with bitcode and uses LLVM backend to generate a PTX assembly."] struct PtxLinker < 'a > { cmd : Command , sess : & 'a Session , }}}
mkitem!{mkimpl!{impl < 'a > Linker for PtxLinker < 'a > { fn cmd (& mut self) -> & mut Command { & mut self . cmd } fn set_output_kind (& mut self , _output_kind : LinkOutputKind , _crate_type : CrateType , _out_filename : & Path ,) { } fn link_staticlib_by_name (& mut self , _name : & str , _verbatim : bool , _whole_archive : bool) { panic ! ("staticlibs not supported") } fn link_staticlib_by_path (& mut self , path : & Path , _whole_archive : bool) { self . link_arg ("--rlib") . link_arg (path) ; } fn debuginfo (& mut self , _strip : Strip , _ : & [PathBuf]) { self . link_arg ("--debug") ; } fn add_object (& mut self , path : & Path) { self . link_arg ("--bitcode") . link_arg (path) ; } fn optimize (& mut self) { match self . sess . lto () { Lto :: Thin | Lto :: Fat | Lto :: ThinLocal => { self . link_arg ("-Olto") ; } Lto :: No => { } } } fn full_relro (& mut self) { } fn partial_relro (& mut self) { } fn no_relro (& mut self) { } fn gc_sections (& mut self , _keep_metadata : bool) { } fn pgo_gen (& mut self) { } fn no_crt_objects (& mut self) { } fn no_default_libraries (& mut self) { } fn control_flow_guard (& mut self) { } fn ehcont_guard (& mut self) { } fn export_symbols (& mut self , _tmpdir : & Path , _crate_type : CrateType , _symbols : & [(String , SymbolExportKind)] ,) { } fn subsystem (& mut self , _subsystem : & str) { } fn linker_plugin_lto (& mut self) { } }}}
mkitem!{mkstruct!{#[doc = " The `self-contained` LLVM bitcode linker"] struct LlbcLinker < 'a > { cmd : Command , sess : & 'a Session , }}}
mkitem!{mkimpl!{impl < 'a > Linker for LlbcLinker < 'a > { fn cmd (& mut self) -> & mut Command { & mut self . cmd } fn set_output_kind (& mut self , _output_kind : LinkOutputKind , _crate_type : CrateType , _out_filename : & Path ,) { } fn link_staticlib_by_name (& mut self , _name : & str , _verbatim : bool , _whole_archive : bool) { panic ! ("staticlibs not supported") } fn link_staticlib_by_path (& mut self , path : & Path , _whole_archive : bool) { self . link_or_cc_arg (path) ; } fn debuginfo (& mut self , _strip : Strip , _ : & [PathBuf]) { self . link_arg ("--debug") ; } fn optimize (& mut self) { self . link_arg (match self . sess . opts . optimize { OptLevel :: No => "-O0" , OptLevel :: Less => "-O1" , OptLevel :: More => "-O2" , OptLevel :: Aggressive => "-O3" , OptLevel :: Size => "-Os" , OptLevel :: SizeMin => "-Oz" , }) ; } fn full_relro (& mut self) { } fn partial_relro (& mut self) { } fn no_relro (& mut self) { } fn gc_sections (& mut self , _keep_metadata : bool) { } fn pgo_gen (& mut self) { } fn no_crt_objects (& mut self) { } fn no_default_libraries (& mut self) { } fn control_flow_guard (& mut self) { } fn ehcont_guard (& mut self) { } fn export_symbols (& mut self , _tmpdir : & Path , _crate_type : CrateType , symbols : & [(String , SymbolExportKind)] ,) { match _crate_type { CrateType :: Cdylib => { for (sym , _) in symbols { self . link_args (& ["--export-symbol" , sym]) ; } } _ => () , } } fn subsystem (& mut self , _subsystem : & str) { } fn linker_plugin_lto (& mut self) { } }}}
mkitem!{mkstruct!{struct BpfLinker < 'a > { cmd : Command , sess : & 'a Session , }}}
mkitem!{mkimpl!{impl < 'a > Linker for BpfLinker < 'a > { fn cmd (& mut self) -> & mut Command { & mut self . cmd } fn set_output_kind (& mut self , _output_kind : LinkOutputKind , _crate_type : CrateType , _out_filename : & Path ,) { } fn link_staticlib_by_name (& mut self , _name : & str , _verbatim : bool , _whole_archive : bool) { panic ! ("staticlibs not supported") } fn link_staticlib_by_path (& mut self , path : & Path , _whole_archive : bool) { self . link_or_cc_arg (path) ; } fn debuginfo (& mut self , _strip : Strip , _ : & [PathBuf]) { self . link_arg ("--debug") ; } fn optimize (& mut self) { self . link_arg (match self . sess . opts . optimize { OptLevel :: No => "-O0" , OptLevel :: Less => "-O1" , OptLevel :: More => "-O2" , OptLevel :: Aggressive => "-O3" , OptLevel :: Size => "-Os" , OptLevel :: SizeMin => "-Oz" , }) ; } fn full_relro (& mut self) { } fn partial_relro (& mut self) { } fn no_relro (& mut self) { } fn gc_sections (& mut self , _keep_metadata : bool) { } fn pgo_gen (& mut self) { } fn no_crt_objects (& mut self) { } fn no_default_libraries (& mut self) { } fn control_flow_guard (& mut self) { } fn ehcont_guard (& mut self) { } fn export_symbols (& mut self , tmpdir : & Path , _crate_type : CrateType , symbols : & [(String , SymbolExportKind)] ,) { let path = tmpdir . join ("symbols") ; let res : io :: Result < () > = try { let mut f = File :: create_buffered (& path) ? ; for (sym , _) in symbols { writeln ! (f , "{sym}") ? ; } } ; if let Err (error) = res { self . sess . dcx () . emit_fatal (errors :: SymbolFileWriteFailure { error }) ; } else { self . link_arg ("--export-symbols") . link_arg (& path) ; } } fn subsystem (& mut self , _subsystem : & str) { } fn linker_plugin_lto (& mut self) { } }}}