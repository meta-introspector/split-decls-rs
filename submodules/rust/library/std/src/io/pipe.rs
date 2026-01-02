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
mkuse!{use crate :: io ;}
mkuse!{use crate :: sys :: anonymous_pipe :: { AnonPipe , pipe as pipe_inner } ;}
mkuse!{use crate :: sys_common :: { FromInner , IntoInner } ;}

macro_rules! pipe_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function pipe in module {}", module_path!());
    };
}

mkfn!{
    pipe_introspect!();
    #[doc = " Creates an anonymous pipe."] #[doc = ""] #[doc = " # Behavior"] #[doc = ""] #[doc = " A pipe is a one-way data channel provided by the OS, which works across processes. A pipe is"] #[doc = " typically used to communicate between two or more separate processes, as there are better,"] #[doc = " faster ways to communicate within a single process."] #[doc = ""] #[doc = " In particular:"] #[doc = ""] #[doc = " * A read on a [`PipeReader`] blocks until the pipe is non-empty."] #[doc = " * A write on a [`PipeWriter`] blocks when the pipe is full."] #[doc = " * When all copies of a [`PipeWriter`] are closed, a read on the corresponding [`PipeReader`]"] #[doc = "   returns EOF."] #[doc = " * [`PipeWriter`] can be shared, and multiple processes or threads can write to it at once, but"] #[doc = "   writes (above a target-specific threshold) may have their data interleaved."] #[doc = " * [`PipeReader`] can be shared, and multiple processes or threads can read it at once. Any"] #[doc = "   given byte will only get consumed by one reader. There are no guarantees about data"] #[doc = "   interleaving."] #[doc = " * Portable applications cannot assume any atomicity of messages larger than a single byte."] #[doc = ""] #[doc = " # Platform-specific behavior"] #[doc = ""] #[doc = " This function currently corresponds to the `pipe` function on Unix and the"] #[doc = " `CreatePipe` function on Windows."] #[doc = ""] #[doc = " Note that this [may change in the future][changes]."] #[doc = ""] #[doc = " # Capacity"] #[doc = ""] #[doc = " Pipe capacity is platform dependent. To quote the Linux [man page]:"] #[doc = ""] #[doc = " > Different implementations have different limits for the pipe capacity. Applications should"] #[doc = " > not rely on a particular capacity: an application should be designed so that a reading process"] #[doc = " > consumes data as soon as it is available, so that a writing process does not remain blocked."] #[doc = ""] #[doc = " # Example"] #[doc = ""] #[doc = " ```no_run"] #[doc = " # #[cfg(miri)] fn main() {}"] #[doc = " # #[cfg(not(miri))]"] #[doc = " # fn main() -> std::io::Result<()> {"] #[doc = " use std::io::{Read, Write, pipe};"] #[doc = " use std::process::Command;"] #[doc = " let (ping_reader, mut ping_writer) = pipe()?;"] #[doc = " let (mut pong_reader, pong_writer) = pipe()?;"] #[doc = ""] #[doc = " // Spawn a child process that echoes its input."] #[doc = " let mut echo_command = Command::new(\"cat\");"] #[doc = " echo_command.stdin(ping_reader);"] #[doc = " echo_command.stdout(pong_writer);"] #[doc = " let mut echo_child = echo_command.spawn()?;"] #[doc = ""] #[doc = " // Send input to the child process. Note that because we're writing all the input before we"] #[doc = " // read any output, this could deadlock if the child's input and output pipe buffers both"] #[doc = " // filled up. Those buffers are usually at least a few KB, so \"hello\" is fine, but for longer"] #[doc = " // inputs we'd need to read and write at the same time, e.g. using threads."] #[doc = " ping_writer.write_all(b\"hello\")?;"] #[doc = ""] #[doc = " // `cat` exits when it reads EOF from stdin, but that can't happen while any ping writer"] #[doc = " // remains open. We need to drop our ping writer, or read_to_string will deadlock below."] #[doc = " drop(ping_writer);"] #[doc = ""] #[doc = " // The pong reader can't report EOF while any pong writer remains open. Our Command object is"] #[doc = " // holding a pong writer, and again read_to_string will deadlock if we don't drop it."] #[doc = " drop(echo_command);"] #[doc = ""] #[doc = " let mut buf = String::new();"] #[doc = " // Block until `cat` closes its stdout (a pong writer)."] #[doc = " pong_reader.read_to_string(&mut buf)?;"] #[doc = " assert_eq!(&buf, \"hello\");"] #[doc = ""] #[doc = " // At this point we know `cat` has exited, but we still need to wait to clean up the \"zombie\"."] #[doc = " echo_child.wait()?;"] #[doc = " # Ok(())"] #[doc = " # }"] #[doc = " ```"] #[doc = " [changes]: io#platform-specific-behavior"] #[doc = " [man page]: https://man7.org/linux/man-pages/man7/pipe.7.html"] #[stable (feature = "anonymous_pipe" , since = "1.87.0")] #[inline] pub fn pipe () -> io :: Result < (PipeReader , PipeWriter) > { pipe_inner () . map (| (reader , writer) | (PipeReader (reader) , PipeWriter (writer))) }
}
mkitem!{mkstruct!{#[doc = " Read end of an anonymous pipe."] #[stable (feature = "anonymous_pipe" , since = "1.87.0")] #[derive (Debug)] pub struct PipeReader (pub (crate) AnonPipe) ;}}
mkitem!{mkstruct!{#[doc = " Write end of an anonymous pipe."] #[stable (feature = "anonymous_pipe" , since = "1.87.0")] #[derive (Debug)] pub struct PipeWriter (pub (crate) AnonPipe) ;}}
mkitem!{mkimpl!{impl FromInner < AnonPipe > for PipeReader { fn from_inner (inner : AnonPipe) -> Self { Self (inner) } }}}
mkitem!{mkimpl!{impl IntoInner < AnonPipe > for PipeReader { fn into_inner (self) -> AnonPipe { self . 0 } }}}
mkitem!{mkimpl!{impl FromInner < AnonPipe > for PipeWriter { fn from_inner (inner : AnonPipe) -> Self { Self (inner) } }}}
mkitem!{mkimpl!{impl IntoInner < AnonPipe > for PipeWriter { fn into_inner (self) -> AnonPipe { self . 0 } }}}
mkitem!{mkimpl!{impl PipeReader { #[doc = " Creates a new [`PipeReader`] instance that shares the same underlying file description."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```no_run"] #[doc = " # #[cfg(miri)] fn main() {}"] #[doc = " # #[cfg(not(miri))]"] #[doc = " # fn main() -> std::io::Result<()> {"] #[doc = " use std::fs;"] #[doc = " use std::io::{pipe, Write};"] #[doc = " use std::process::Command;"] #[doc = " const NUM_SLOT: u8 = 2;"] #[doc = " const NUM_PROC: u8 = 5;"] #[doc = " const OUTPUT: &str = \"work.txt\";"] #[doc = ""] #[doc = " let mut jobs = vec![];"] #[doc = " let (reader, mut writer) = pipe()?;"] #[doc = ""] #[doc = " // Write NUM_SLOT characters the pipe."] #[doc = " writer.write_all(&[b'|'; NUM_SLOT as usize])?;"] #[doc = ""] #[doc = " // Spawn several processes that read a character from the pipe, do some work, then"] #[doc = " // write back to the pipe. When the pipe is empty, the processes block, so only"] #[doc = " // NUM_SLOT processes can be working at any given time."] #[doc = " for _ in 0..NUM_PROC {"] #[doc = "     jobs.push("] #[doc = "         Command::new(\"bash\")"] #[doc = "             .args([\"-c\","] #[doc = "                 &format!("] #[doc = "                      \"read -n 1\\n\\"] #[doc = "                       echo -n 'x' >> '{OUTPUT}'\\n\\"] #[doc = "                       echo -n '|'\","] #[doc = "                 ),"] #[doc = "             ])"] #[doc = "             .stdin(reader.try_clone()?)"] #[doc = "             .stdout(writer.try_clone()?)"] #[doc = "             .spawn()?,"] #[doc = "     );"] #[doc = " }"] #[doc = ""] #[doc = " // Wait for all jobs to finish."] #[doc = " for mut job in jobs {"] #[doc = "     job.wait()?;"] #[doc = " }"] #[doc = ""] #[doc = " // Check our work and clean up."] #[doc = " let xs = fs::read_to_string(OUTPUT)?;"] #[doc = " fs::remove_file(OUTPUT)?;"] #[doc = " assert_eq!(xs, \"x\".repeat(NUM_PROC.into()));"] #[doc = " # Ok(())"] #[doc = " # }"] #[doc = " ```"] #[stable (feature = "anonymous_pipe" , since = "1.87.0")] pub fn try_clone (& self) -> io :: Result < Self > { self . 0 . try_clone () . map (Self) } }}}
mkitem!{mkimpl!{impl PipeWriter { #[doc = " Creates a new [`PipeWriter`] instance that shares the same underlying file description."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```no_run"] #[doc = " # #[cfg(miri)] fn main() {}"] #[doc = " # #[cfg(not(miri))]"] #[doc = " # fn main() -> std::io::Result<()> {"] #[doc = " use std::process::Command;"] #[doc = " use std::io::{pipe, Read};"] #[doc = " let (mut reader, writer) = pipe()?;"] #[doc = ""] #[doc = " // Spawn a process that writes to stdout and stderr."] #[doc = " let mut peer = Command::new(\"bash\")"] #[doc = "     .args(["] #[doc = "         \"-c\","] #[doc = "         \"echo -n foo\\n\\"] #[doc = "          echo -n bar >&2\""] #[doc = "     ])"] #[doc = "     .stdout(writer.try_clone()?)"] #[doc = "     .stderr(writer)"] #[doc = "     .spawn()?;"] #[doc = ""] #[doc = " // Read and check the result."] #[doc = " let mut msg = String::new();"] #[doc = " reader.read_to_string(&mut msg)?;"] #[doc = " assert_eq!(&msg, \"foobar\");"] #[doc = ""] #[doc = " peer.wait()?;"] #[doc = " # Ok(())"] #[doc = " # }"] #[doc = " ```"] #[stable (feature = "anonymous_pipe" , since = "1.87.0")] pub fn try_clone (& self) -> io :: Result < Self > { self . 0 . try_clone () . map (Self) } }}}
mkitem!{mkimpl!{#[stable (feature = "anonymous_pipe" , since = "1.87.0")] impl io :: Read for & PipeReader { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { self . 0 . read (buf) } fn read_vectored (& mut self , bufs : & mut [io :: IoSliceMut < '_ >]) -> io :: Result < usize > { self . 0 . read_vectored (bufs) } #[inline] fn is_read_vectored (& self) -> bool { self . 0 . is_read_vectored () } fn read_to_end (& mut self , buf : & mut Vec < u8 >) -> io :: Result < usize > { self . 0 . read_to_end (buf) } fn read_buf (& mut self , buf : io :: BorrowedCursor < '_ >) -> io :: Result < () > { self . 0 . read_buf (buf) } }}}
mkitem!{mkimpl!{#[stable (feature = "anonymous_pipe" , since = "1.87.0")] impl io :: Read for PipeReader { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { self . 0 . read (buf) } fn read_vectored (& mut self , bufs : & mut [io :: IoSliceMut < '_ >]) -> io :: Result < usize > { self . 0 . read_vectored (bufs) } #[inline] fn is_read_vectored (& self) -> bool { self . 0 . is_read_vectored () } fn read_to_end (& mut self , buf : & mut Vec < u8 >) -> io :: Result < usize > { self . 0 . read_to_end (buf) } fn read_buf (& mut self , buf : io :: BorrowedCursor < '_ >) -> io :: Result < () > { self . 0 . read_buf (buf) } }}}
mkitem!{mkimpl!{#[stable (feature = "anonymous_pipe" , since = "1.87.0")] impl io :: Write for & PipeWriter { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . 0 . write (buf) } #[inline] fn flush (& mut self) -> io :: Result < () > { Ok (()) } fn write_vectored (& mut self , bufs : & [io :: IoSlice < '_ >]) -> io :: Result < usize > { self . 0 . write_vectored (bufs) } #[inline] fn is_write_vectored (& self) -> bool { self . 0 . is_write_vectored () } }}}
mkitem!{mkimpl!{#[stable (feature = "anonymous_pipe" , since = "1.87.0")] impl io :: Write for PipeWriter { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . 0 . write (buf) } #[inline] fn flush (& mut self) -> io :: Result < () > { Ok (()) } fn write_vectored (& mut self , bufs : & [io :: IoSlice < '_ >]) -> io :: Result < usize > { self . 0 . write_vectored (bufs) } #[inline] fn is_write_vectored (& self) -> bool { self . 0 . is_write_vectored () } }}}