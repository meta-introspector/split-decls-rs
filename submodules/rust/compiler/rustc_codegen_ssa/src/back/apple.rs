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
mkuse!{use std :: ffi :: OsString ;}
mkuse!{use std :: path :: PathBuf ;}
mkuse!{use std :: process :: Command ;}
mkuse!{use itertools :: Itertools ;}
mkuse!{use rustc_middle :: middle :: exported_symbols :: SymbolExportKind ;}
mkuse!{use rustc_session :: Session ;}
mkuse!{use rustc_target :: spec :: Target ;}
mkuse!{pub (super) use rustc_target :: spec :: apple :: OSVersion ;}
mkuse!{use tracing :: debug ;}
mkuse!{use crate :: errors :: { XcrunError , XcrunSdkPathWarning } ;}
mkuse!{use crate :: fluent_generated as fluent ;}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                 
            }}

macro_rules! sdk_name_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sdk_name in module {}", module_path!());
    };
}

mkfn!{
    sdk_name_introspect!();
    # [doc = " The canonical name of the desired SDK for a given target."] pub (super) fn sdk_name (target : & Target) -> & 'static str { match (& * target . os , & * target . env) { ("macos" , "") => "MacOSX" , ("ios" , "") => "iPhoneOS" , ("ios" , "sim") => "iPhoneSimulator" , ("ios" , "macabi") => "MacOSX" , ("tvos" , "") => "AppleTVOS" , ("tvos" , "sim") => "AppleTVSimulator" , ("visionos" , "") => "XROS" , ("visionos" , "sim") => "XRSimulator" , ("watchos" , "") => "WatchOS" , ("watchos" , "sim") => "WatchSimulator" , (os , abi) => unreachable ! ("invalid os '{os}' / abi '{abi}' combination for Apple target") , } }
}

macro_rules! macho_platform_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function macho_platform in module {}", module_path!());
    };
}

mkfn!{
    macho_platform_introspect!();
    pub (super) fn macho_platform (target : & Target) -> u32 { match (& * target . os , & * target . env) { ("macos" , _) => object :: macho :: PLATFORM_MACOS , ("ios" , "macabi") => object :: macho :: PLATFORM_MACCATALYST , ("ios" , "sim") => object :: macho :: PLATFORM_IOSSIMULATOR , ("ios" , _) => object :: macho :: PLATFORM_IOS , ("watchos" , "sim") => object :: macho :: PLATFORM_WATCHOSSIMULATOR , ("watchos" , _) => object :: macho :: PLATFORM_WATCHOS , ("tvos" , "sim") => object :: macho :: PLATFORM_TVOSSIMULATOR , ("tvos" , _) => object :: macho :: PLATFORM_TVOS , ("visionos" , "sim") => object :: macho :: PLATFORM_XROSSIMULATOR , ("visionos" , _) => object :: macho :: PLATFORM_XROS , _ => unreachable ! ("tried to get Mach-O platform for non-Apple target") , } }
}

macro_rules! add_data_and_relocation_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function add_data_and_relocation in module {}", module_path!());
    };
}

mkfn!{
    add_data_and_relocation_introspect!();
    # [doc = " Add relocation and section data needed for a symbol to be considered"] # [doc = " undefined by ld64."] # [doc = ""] # [doc = " The relocation must be valid, and hence must point to a valid piece of"] # [doc = " machine code, and hence this is unfortunately very architecture-specific."] # [doc = ""] # [doc = ""] # [doc = " # New architectures"] # [doc = ""] # [doc = " The values here are basically the same as emitted by the following program:"] # [doc = ""] # [doc = " ```c"] # [doc = " // clang -c foo.c -target $CLANG_TARGET"] # [doc = " void foo(void);"] # [doc = ""] # [doc = " extern int bar;"] # [doc = ""] # [doc = " void* foobar[2] = {"] # [doc = "     (void*)foo,"] # [doc = "     (void*)&bar,"] # [doc = "     // ..."] # [doc = " };"] # [doc = " ```"] # [doc = ""] # [doc = " Can be inspected with:"] # [doc = " ```console"] # [doc = " objdump --macho --reloc foo.o"] # [doc = " objdump --macho --full-contents foo.o"] # [doc = " ```"] pub (super) fn add_data_and_relocation (file : & mut object :: write :: Object < '_ > , section : object :: write :: SectionId , symbol : object :: write :: SymbolId , target : & Target , kind : SymbolExportKind ,) -> object :: write :: Result < () > { let authenticated_pointer = kind == SymbolExportKind :: Text && target . llvm_target . starts_with ("arm64e") ; let data : & [u8] = match target . pointer_width { _ if authenticated_pointer => & [0 , 0 , 0 , 0 , 0 , 0 , 0 , 0x80] , 32 => & [0 ; 4] , 64 => & [0 ; 8] , pointer_width => unimplemented ! ("unsupported Apple pointer width {pointer_width:?}") , } ; if target . arch == "x86_64" { file . section_mut (section) . append_data (& [] , 16) ; } else { file . section_mut (section) . append_data (& [] , target . pointer_width as u64) ; } let offset = file . section_mut (section) . append_data (data , data . len () as u64) ; let flags = if authenticated_pointer { object :: write :: RelocationFlags :: MachO { r_type : object :: macho :: ARM64_RELOC_AUTHENTICATED_POINTER , r_pcrel : false , r_length : 3 , } } else if target . arch == "arm" { object :: write :: RelocationFlags :: MachO { r_type : object :: macho :: ARM_RELOC_VANILLA , r_pcrel : false , r_length : 2 , } } else { object :: write :: RelocationFlags :: Generic { kind : object :: RelocationKind :: Absolute , encoding : object :: RelocationEncoding :: Generic , size : target . pointer_width as u8 , } } ; file . add_relocation (section , object :: write :: Relocation { offset , addend : 0 , symbol , flags }) ? ; Ok (()) }
}

macro_rules! add_version_to_llvm_target_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function add_version_to_llvm_target in module {}", module_path!());
    };
}

mkfn!{
    add_version_to_llvm_target_introspect!();
    pub (super) fn add_version_to_llvm_target (llvm_target : & str , deployment_target : OSVersion ,) -> String { let mut components = llvm_target . split ("-") ; let arch = components . next () . expect ("apple target should have arch") ; let vendor = components . next () . expect ("apple target should have vendor") ; let os = components . next () . expect ("apple target should have os") ; let environment = components . next () ; assert_eq ! (components . next () , None , "too many LLVM triple components") ; assert ! (! os . contains (| c : char | c . is_ascii_digit ()) , "LLVM target must not already be versioned") ; let version = deployment_target . fmt_full () ; if let Some (env) = environment { format ! ("{arch}-{vendor}-{os}{version}-{env}") } else { format ! ("{arch}-{vendor}-{os}{version}") } }
}

macro_rules! get_sdk_root_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_sdk_root in module {}", module_path!());
    };
}

mkfn!{
    get_sdk_root_introspect!();
    pub (super) fn get_sdk_root (sess : & Session) -> Option < PathBuf > { let sdk_name = sdk_name (& sess . target) ; match xcrun_show_sdk_path (sdk_name , false) { Ok ((path , stderr)) => { if ! stderr . is_empty () { sess . dcx () . emit_warn (XcrunSdkPathWarning { sdk_name , stderr }) ; } Some (path) } Err (err) => { let mut diag = sess . dcx () . create_warn (err) ; diag . note (fluent :: codegen_ssa_xcrun_about) ; if let Some (developer_dir) = xcode_select_developer_dir () { diag . arg ("developer_dir" , & developer_dir) ; diag . note (fluent :: codegen_ssa_xcrun_found_developer_dir) ; if developer_dir . as_os_str () . to_string_lossy () . contains ("CommandLineTools") { if sdk_name != "MacOSX" { diag . help (fluent :: codegen_ssa_xcrun_command_line_tools_insufficient) ; } } } else { diag . help (fluent :: codegen_ssa_xcrun_no_developer_dir) ; } diag . emit () ; None } } }
}

macro_rules! xcrun_show_sdk_path_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function xcrun_show_sdk_path in module {}", module_path!());
    };
}

mkfn!{
    xcrun_show_sdk_path_introspect!();
    # [doc = " Invoke `xcrun --sdk $sdk_name --show-sdk-path` to get the SDK path."] # [doc = ""] # [doc = " The exact logic that `xcrun` uses is unspecified (see `man xcrun` for a few details), and may"] # [doc = " change between macOS and Xcode versions, but it roughly boils down to finding the active"] # [doc = " developer directory, and then invoking `xcodebuild -sdk $sdk_name -version` to get the SDK"] # [doc = " details."] # [doc = ""] # [doc = " Finding the developer directory is roughly done by looking at, in order:"] # [doc = " - The `DEVELOPER_DIR` environment variable."] # [doc = " - The `/var/db/xcode_select_link` symlink (set by `xcode-select --switch`)."] # [doc = " - `/Applications/Xcode.app` (hardcoded fallback path)."] # [doc = " - `/Library/Developer/CommandLineTools` (hardcoded fallback path)."] # [doc = ""] # [doc = " Note that `xcrun` caches its result, but with a cold cache this whole operation can be quite"] # [doc = " slow, especially so the first time it's run after a reboot."] fn xcrun_show_sdk_path (sdk_name : & 'static str , verbose : bool ,) -> Result < (PathBuf , String) , XcrunError > { let mut cmd = Command :: new ("xcrun") ; if verbose { cmd . arg ("--verbose") ; } cmd . arg ("--sdk") ; cmd . arg (& sdk_name . to_lowercase ()) ; cmd . arg ("--show-sdk-path") ; let output = cmd . output () . map_err (| error | XcrunError :: FailedInvoking { sdk_name , command_formatted : format ! ("{cmd:?}") , error , }) ? ; let stderr = String :: from_utf8_lossy_owned (output . stderr) ; if ! stderr . is_empty () { debug ! (stderr , "original xcrun stderr") ; } let stderr = stderr . lines () . filter (| line | { ! line . contains ("Writing error result bundle") && ! line . contains ("Requested but did not find extension point with identifier") }) . join ("\n") ; if output . status . success () { Ok ((stdout_to_path (output . stdout) , stderr)) } else { let stdout = String :: from_utf8_lossy_owned (output . stdout) . trim () . to_string () ; Err (XcrunError :: Unsuccessful { sdk_name , command_formatted : format ! ("{cmd:?}") , stdout , stderr , }) } }
}

macro_rules! xcode_select_developer_dir_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function xcode_select_developer_dir in module {}", module_path!());
    };
}

mkfn!{
    xcode_select_developer_dir_introspect!();
    # [doc = " Invoke `xcode-select --print-path`, and return the current developer directory."] # [doc = ""] # [doc = " NOTE: We don't do any error handling here, this is only used as a canary in diagnostics (`xcrun`"] # [doc = " will have already emitted the relevant error information)."] fn xcode_select_developer_dir () -> Option < PathBuf > { let mut cmd = Command :: new ("xcode-select") ; cmd . arg ("--print-path") ; let output = cmd . output () . ok () ? ; if ! output . status . success () { return None ; } Some (stdout_to_path (output . stdout)) }
}

macro_rules! stdout_to_path_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function stdout_to_path in module {}", module_path!());
    };
}

mkfn!{
    stdout_to_path_introspect!();
    fn stdout_to_path (mut stdout : Vec < u8 >) -> PathBuf { if let Some (b'\n') = stdout . last () { let _ = stdout . pop () . unwrap () ; } # [cfg (unix)] let path = < OsString as std :: os :: unix :: ffi :: OsStringExt > :: from_vec (stdout) ; # [cfg (not (unix))] let path = OsString :: from (String :: from_utf8 (stdout) . expect ("stdout must be UTF-8")) ; PathBuf :: from (path) }
}