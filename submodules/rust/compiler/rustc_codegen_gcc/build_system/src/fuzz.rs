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
mkuse!{use std :: ffi :: OsStr ;}
mkuse!{use std :: path :: Path ;}
mkmod!{reduce, { 
                getname!(reduce);
                getsrc!(reduce);
                getpath!(reduce);
                get_deps!(reduce);
                get_crates!(reduce);
                mkinclude!(reduce);
                 
            }}
mkuse!{use crate :: utils :: run_command_with_output ;}

macro_rules! show_usage_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function show_usage in module {}", module_path!());
    };
}

mkfn!{
    show_usage_introspect!();
    fn show_usage () { println ! (r#"
`fuzz` command help:
    --reduce               : Reduces a file generated by rustlantis
    --help                 : Show this help
    --start                : Start of the fuzzed range
    --count                : The number of cases to fuzz
    -j --jobs              : The number of threads to use during fuzzing"#) ; }
}

macro_rules! run_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function run in module {}", module_path!());
    };
}

mkfn!{
    run_introspect!();
    pub fn run () -> Result < () , String > { let mut args = std :: env :: args () . skip (2) ; let mut start = 0 ; let mut count = 100 ; let mut threads = std :: thread :: available_parallelism () . map (| threads | threads . get ()) . unwrap_or (1) ; while let Some (arg) = args . next () { match arg . as_str () { "--reduce" => { let Some (path) = args . next () else { return Err ("--reduce must be provided with a path" . into ()) ; } ; if ! std :: fs :: exists (& path) . unwrap_or (false) { return Err ("--reduce must be provided with a valid path" . into ()) ; } reduce :: reduce (& path) ; return Ok (()) ; } "--help" => { show_usage () ; return Ok (()) ; } "--start" => { start = str :: parse (& args . next () . ok_or_else (| | "Fuzz start not provided!" . to_string ()) ?) . map_err (| err | format ! ("Fuzz start not a number {err:?}!")) ? ; } "--count" => { count = str :: parse (& args . next () . ok_or_else (| | "Fuzz count not provided!" . to_string ()) ?) . map_err (| err | format ! ("Fuzz count not a number {err:?}!")) ? ; } "-j" | "--jobs" => { threads = str :: parse (& args . next () . ok_or_else (| | "Fuzz thread count not provided!" . to_string ()) ? ,) . map_err (| err | format ! ("Fuzz thread count not a number {err:?}!")) ? ; } _ => return Err (format ! ("Unknown option {arg}")) , } } crate :: utils :: git_clone ("https://github.com/cbeuw/rustlantis.git" , Some ("clones/rustlantis" . as_ref ()) , true ,) . map_err (| err | format ! ("Git clone failed with message: {err:?}!")) ? ; let cmd : & [& dyn AsRef < OsStr >] = & [& "git" , & "pull" , & "origin"] ; run_command_with_output (cmd , Some (Path :: new ("clones/rustlantis"))) ? ; let cmd : & [& dyn AsRef < OsStr >] = & [& "cargo" , & "build" , & "--release"] ; run_command_with_output (cmd , Some (Path :: new ("clones/rustlantis"))) ? ; fuzz_range (start , start + count , threads) ; Ok (()) }
}

macro_rules! fuzz_range_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fuzz_range in module {}", module_path!());
    };
}

mkfn!{
    fuzz_range_introspect!();
    #[doc = " Fuzzes a range `start..end` with `threads`."] fn fuzz_range (start : u64 , end : u64 , threads : usize) { use std :: sync :: Arc ; use std :: sync :: atomic :: { AtomicU64 , Ordering } ; use std :: time :: { Duration , Instant } ; let total = end - start ; let start = Arc :: new (AtomicU64 :: new (start)) ; let start_time = Instant :: now () ; let mut workers = Vec :: with_capacity (threads) ; for _ in 0 .. threads { let start = start . clone () ; workers . push (std :: thread :: spawn (move | | { while start . load (Ordering :: Relaxed) < end { let next = start . fetch_add (1 , Ordering :: Relaxed) ; match test (next , false) { Err (err) => { println ! ("test({next}) failed because {err:?}") ; let mut out_path : std :: path :: PathBuf = "target/fuzz/compiletime_error" . into () ; std :: fs :: create_dir_all (& out_path) . unwrap () ; out_path . push (format ! ("fuzz{next}.rs")) ; std :: fs :: copy (err , out_path) . unwrap () ; } Ok (Err (err)) => { println ! ("The LLVM and GCC results don't match for {err:?}") ; let mut out_path : std :: path :: PathBuf = "target/fuzz/runtime_error" . into () ; std :: fs :: create_dir_all (& out_path) . unwrap () ; let Ok (Err (tmp_print_err)) = test (next , true) else { out_path . push (format ! ("fuzz{next}.rs")) ; std :: fs :: copy (err , & out_path) . unwrap () ; continue ; } ; out_path . push (format ! ("fuzz{next}.rs")) ; std :: fs :: copy (tmp_print_err , & out_path) . unwrap () ; reduce :: reduce (& out_path) ; } Ok (Ok (())) => () , } } })) ; } while start . load (Ordering :: Relaxed) < end || ! workers . iter () . all (| t | t . is_finished ()) { let five_hundred_millis = Duration :: from_millis (500) ; std :: thread :: sleep (five_hundred_millis) ; let remaining = end - start . load (Ordering :: Relaxed) ; let fuzzed = (total - remaining) . saturating_sub (threads as u64) ; let iter_per_sec = fuzzed as f64 / start_time . elapsed () . as_secs_f64 () ; println ! ("fuzzed {fuzzed} cases({}%), at rate {iter_per_sec} iter/s, remaining ~{}s" , (100 * fuzzed) as f64 / total as f64 , (remaining as f64) / iter_per_sec) } drop (workers) ; }
}

macro_rules! debug_llvm_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function debug_llvm in module {}", module_path!());
    };
}

mkfn!{
    debug_llvm_introspect!();
    #[doc = " Builds & runs a file with LLVM."] fn debug_llvm (path : & std :: path :: Path) -> Result < Vec < u8 > , String > { let exe_path = path . with_extension ("llvm_elf") ; let output = std :: process :: Command :: new ("rustc") . arg (path) . arg ("-o") . arg (& exe_path) . output () . map_err (| err | format ! ("{err:?}")) ? ; if ! output . status . success () { return Err (format ! ("LLVM compilation failed:{output:?}")) ; } let output = std :: process :: Command :: new (& exe_path) . output () . map_err (| err | format ! ("{err:?}")) ? ; if ! output . status . success () { return Err (format ! ("The program at {path:?}, compiled with LLVM, exited unsuccessfully:{output:?}")) ; } std :: fs :: remove_file (exe_path) . map_err (| err | format ! ("{err:?}")) ? ; let mut res = output . stdout ; res . extend (output . stderr) ; Ok (res) }
}

macro_rules! release_gcc_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function release_gcc in module {}", module_path!());
    };
}

mkfn!{
    release_gcc_introspect!();
    #[doc = " Builds & runs a file with GCC."] fn release_gcc (path : & std :: path :: Path) -> Result < Vec < u8 > , String > { let exe_path = path . with_extension ("gcc_elf") ; let output = std :: process :: Command :: new ("./y.sh") . arg ("rustc") . arg (path) . arg ("-O") . arg ("-o") . arg (& exe_path) . output () . map_err (| err | format ! ("{err:?}")) ? ; if ! output . status . success () { return Err (format ! ("GCC compilation failed:{output:?}")) ; } let output = std :: process :: Command :: new (& exe_path) . output () . map_err (| err | format ! ("{err:?}")) ? ; if ! output . status . success () { return Err (format ! ("The program at {path:?}, compiled with GCC, exited unsuccessfully:{output:?}")) ; } std :: fs :: remove_file (exe_path) . map_err (| err | format ! ("{err:?}")) ? ; let mut res = output . stdout ; res . extend (output . stderr) ; Ok (res) }
}
mkitem!{type ResultCache = Option < (Vec < u8 > , Vec < u8 >) > ;}

macro_rules! test_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test in module {}", module_path!());
    };
}

mkfn!{
    test_introspect!();
    #[doc = " Generates a new rustlantis file, & compares the result of running it with GCC and LLVM."] fn test (seed : u64 , print_tmp_vars : bool) -> Result < Result < () , std :: path :: PathBuf > , String > { let source_file = generate (seed , print_tmp_vars) ? ; test_file (& source_file , true) }
}

macro_rules! test_cached_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_cached in module {}", module_path!());
    };
}

mkfn!{
    test_cached_introspect!();
    #[doc = " Tests a file with a cached LLVM result. Used for reduction, when it is known"] #[doc = " that a given transformation should not change the execution result."] fn test_cached (source_file : & Path , remove_tmps : bool , cache : & mut ResultCache ,) -> Result < Result < () , std :: path :: PathBuf > , String > { let gcc_res = release_gcc (source_file) ? ; if cache . is_none () { * cache = Some ((debug_llvm (source_file) ? , gcc_res . clone ())) ; } let (llvm_res , old_gcc) = cache . as_ref () . unwrap () ; if * llvm_res != gcc_res && gcc_res == * old_gcc { Ok (Err (source_file . to_path_buf ())) } else { if remove_tmps { std :: fs :: remove_file (source_file) . map_err (| err | format ! ("{err:?}")) ? ; } Ok (Ok (())) } }
}

macro_rules! test_file_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_file in module {}", module_path!());
    };
}

mkfn!{
    test_file_introspect!();
    fn test_file (source_file : & Path , remove_tmps : bool ,) -> Result < Result < () , std :: path :: PathBuf > , String > { let mut uncached = None ; test_cached (source_file , remove_tmps , & mut uncached) }
}

macro_rules! generate_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function generate in module {}", module_path!());
    };
}

mkfn!{
    generate_introspect!();
    #[doc = " Generates a new rustlantis file for us to run tests on."] fn generate (seed : u64 , print_tmp_vars : bool) -> Result < std :: path :: PathBuf , String > { use std :: io :: Write ; let mut out_path = std :: env :: temp_dir () ; out_path . push (format ! ("fuzz{seed}.rs")) ; let mut generate = std :: process :: Command :: new ("cargo") ; generate . args (["run" , "--release" , "--bin" , "generate"]) . arg (format ! ("{seed}")) . current_dir ("clones/rustlantis") ; if print_tmp_vars { generate . arg ("--debug") ; } let out = generate . output () . map_err (| err | format ! ("{err:?}")) ? ; std :: fs :: File :: create (& out_path) . map_err (| err | format ! ("{err:?}")) ? . write_all (& out . stdout) . map_err (| err | format ! ("{err:?}")) ? ; Ok (out_path) }
}