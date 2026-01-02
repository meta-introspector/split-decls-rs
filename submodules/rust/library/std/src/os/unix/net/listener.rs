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
mkuse!{use super :: { SocketAddr , UnixStream , sockaddr_un } ;}
mkuse!{use crate :: os :: unix :: io :: { AsFd , AsRawFd , BorrowedFd , FromRawFd , IntoRawFd , OwnedFd , RawFd } ;}
mkuse!{use crate :: path :: Path ;}
mkuse!{use crate :: sys :: cvt ;}
mkuse!{use crate :: sys :: net :: Socket ;}
mkuse!{use crate :: sys_common :: { AsInner , FromInner , IntoInner } ;}
mkuse!{use crate :: { fmt , io , mem } ;}
mkitem!{mkstruct!{#[doc = " A structure representing a Unix domain socket server."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```no_run"] #[doc = " use std::thread;"] #[doc = " use std::os::unix::net::{UnixStream, UnixListener};"] #[doc = ""] #[doc = " fn handle_client(stream: UnixStream) {"] #[doc = "     // ..."] #[doc = " }"] #[doc = ""] #[doc = " fn main() -> std::io::Result<()> {"] #[doc = "     let listener = UnixListener::bind(\"/path/to/the/socket\")?;"] #[doc = ""] #[doc = "     // accept connections and process them, spawning a new thread for each one"] #[doc = "     for stream in listener.incoming() {"] #[doc = "         match stream {"] #[doc = "             Ok(stream) => {"] #[doc = "                 /* connection succeeded */"] #[doc = "                 thread::spawn(|| handle_client(stream));"] #[doc = "             }"] #[doc = "             Err(err) => {"] #[doc = "                 /* connection failed */"] #[doc = "                 break;"] #[doc = "             }"] #[doc = "         }"] #[doc = "     }"] #[doc = "     Ok(())"] #[doc = " }"] #[doc = " ```"] #[stable (feature = "unix_socket" , since = "1.10.0")] pub struct UnixListener (Socket) ;}}
mkitem!{mkimpl!{#[stable (feature = "unix_socket" , since = "1.10.0")] impl fmt :: Debug for UnixListener { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut builder = fmt . debug_struct ("UnixListener") ; builder . field ("fd" , self . 0 . as_inner ()) ; if let Ok (addr) = self . local_addr () { builder . field ("local" , & addr) ; } builder . finish () } }}}
mkitem!{mkimpl!{impl UnixListener { #[doc = " Creates a new `UnixListener` bound to the specified socket."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```no_run"] #[doc = " use std::os::unix::net::UnixListener;"] #[doc = ""] #[doc = " let listener = match UnixListener::bind(\"/path/to/the/socket\") {"] #[doc = "     Ok(sock) => sock,"] #[doc = "     Err(e) => {"] #[doc = "         println!(\"Couldn't connect: {e:?}\");"] #[doc = "         return"] #[doc = "     }"] #[doc = " };"] #[doc = " ```"] #[stable (feature = "unix_socket" , since = "1.10.0")] pub fn bind < P : AsRef < Path > > (path : P) -> io :: Result < UnixListener > { unsafe { let inner = Socket :: new_raw (libc :: AF_UNIX , libc :: SOCK_STREAM) ? ; let (addr , len) = sockaddr_un (path . as_ref ()) ? ; #[cfg (any (target_os = "windows" , target_os = "redox" , target_os = "espidf" , target_os = "horizon"))] const backlog : core :: ffi :: c_int = 128 ; #[cfg (any (target_os = "linux" , target_os = "freebsd" , target_os = "openbsd" , target_vendor = "apple" ,))] const backlog : core :: ffi :: c_int = - 1 ; #[cfg (not (any (target_os = "windows" , target_os = "redox" , target_os = "espidf" , target_os = "horizon" , target_os = "linux" , target_os = "freebsd" , target_os = "openbsd" , target_vendor = "apple" ,)))] const backlog : libc :: c_int = libc :: SOMAXCONN ; cvt (libc :: bind (inner . as_inner () . as_raw_fd () , (& raw const addr) as * const _ , len as _)) ? ; cvt (libc :: listen (inner . as_inner () . as_raw_fd () , backlog)) ? ; Ok (UnixListener (inner)) } } #[doc = " Creates a new `UnixListener` bound to the specified [`socket address`]."] #[doc = ""] #[doc = " [`socket address`]: crate::os::unix::net::SocketAddr"] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```no_run"] #[doc = " use std::os::unix::net::{UnixListener};"] #[doc = ""] #[doc = " fn main() -> std::io::Result<()> {"] #[doc = "     let listener1 = UnixListener::bind(\"path/to/socket\")?;"] #[doc = "     let addr = listener1.local_addr()?;"] #[doc = ""] #[doc = "     let listener2 = match UnixListener::bind_addr(&addr) {"] #[doc = "         Ok(sock) => sock,"] #[doc = "         Err(err) => {"] #[doc = "             println!(\"Couldn't bind: {err:?}\");"] #[doc = "             return Err(err);"] #[doc = "         }"] #[doc = "     };"] #[doc = "     Ok(())"] #[doc = " }"] #[doc = " ```"] #[stable (feature = "unix_socket_abstract" , since = "1.70.0")] pub fn bind_addr (socket_addr : & SocketAddr) -> io :: Result < UnixListener > { unsafe { let inner = Socket :: new_raw (libc :: AF_UNIX , libc :: SOCK_STREAM) ? ; #[cfg (target_os = "linux")] const backlog : core :: ffi :: c_int = - 1 ; #[cfg (not (target_os = "linux"))] const backlog : core :: ffi :: c_int = 128 ; cvt (libc :: bind (inner . as_raw_fd () , (& raw const socket_addr . addr) as * const _ , socket_addr . len as _ ,)) ? ; cvt (libc :: listen (inner . as_raw_fd () , backlog)) ? ; Ok (UnixListener (inner)) } } #[doc = " Accepts a new incoming connection to this listener."] #[doc = ""] #[doc = " This function will block the calling thread until a new Unix connection"] #[doc = " is established. When established, the corresponding [`UnixStream`] and"] #[doc = " the remote peer's address will be returned."] #[doc = ""] #[doc = " [`UnixStream`]: crate::os::unix::net::UnixStream"] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```no_run"] #[doc = " use std::os::unix::net::UnixListener;"] #[doc = ""] #[doc = " fn main() -> std::io::Result<()> {"] #[doc = "     let listener = UnixListener::bind(\"/path/to/the/socket\")?;"] #[doc = ""] #[doc = "     match listener.accept() {"] #[doc = "         Ok((socket, addr)) => println!(\"Got a client: {addr:?}\"),"] #[doc = "         Err(e) => println!(\"accept function failed: {e:?}\"),"] #[doc = "     }"] #[doc = "     Ok(())"] #[doc = " }"] #[doc = " ```"] #[stable (feature = "unix_socket" , since = "1.10.0")] pub fn accept (& self) -> io :: Result < (UnixStream , SocketAddr) > { let mut storage : libc :: sockaddr_un = unsafe { mem :: zeroed () } ; let mut len = size_of_val (& storage) as libc :: socklen_t ; let sock = self . 0 . accept ((& raw mut storage) as * mut _ , & mut len) ? ; let addr = SocketAddr :: from_parts (storage , len) ? ; Ok ((UnixStream (sock) , addr)) } #[doc = " Creates a new independently owned handle to the underlying socket."] #[doc = ""] #[doc = " The returned `UnixListener` is a reference to the same socket that this"] #[doc = " object references. Both handles can be used to accept incoming"] #[doc = " connections and options set on one listener will affect the other."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```no_run"] #[doc = " use std::os::unix::net::UnixListener;"] #[doc = ""] #[doc = " fn main() -> std::io::Result<()> {"] #[doc = "     let listener = UnixListener::bind(\"/path/to/the/socket\")?;"] #[doc = "     let listener_copy = listener.try_clone().expect(\"try_clone failed\");"] #[doc = "     Ok(())"] #[doc = " }"] #[doc = " ```"] #[stable (feature = "unix_socket" , since = "1.10.0")] pub fn try_clone (& self) -> io :: Result < UnixListener > { self . 0 . duplicate () . map (UnixListener) } #[doc = " Returns the local socket address of this listener."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```no_run"] #[doc = " use std::os::unix::net::UnixListener;"] #[doc = ""] #[doc = " fn main() -> std::io::Result<()> {"] #[doc = "     let listener = UnixListener::bind(\"/path/to/the/socket\")?;"] #[doc = "     let addr = listener.local_addr().expect(\"Couldn't get local address\");"] #[doc = "     Ok(())"] #[doc = " }"] #[doc = " ```"] #[stable (feature = "unix_socket" , since = "1.10.0")] pub fn local_addr (& self) -> io :: Result < SocketAddr > { SocketAddr :: new (| addr , len | unsafe { libc :: getsockname (self . as_raw_fd () , addr , len) }) } #[doc = " Moves the socket into or out of nonblocking mode."] #[doc = ""] #[doc = " This will result in the `accept` operation becoming nonblocking,"] #[doc = " i.e., immediately returning from their calls. If the IO operation is"] #[doc = " successful, `Ok` is returned and no further action is required. If the"] #[doc = " IO operation could not be completed and needs to be retried, an error"] #[doc = " with kind [`io::ErrorKind::WouldBlock`] is returned."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```no_run"] #[doc = " use std::os::unix::net::UnixListener;"] #[doc = ""] #[doc = " fn main() -> std::io::Result<()> {"] #[doc = "     let listener = UnixListener::bind(\"/path/to/the/socket\")?;"] #[doc = "     listener.set_nonblocking(true).expect(\"Couldn't set non blocking\");"] #[doc = "     Ok(())"] #[doc = " }"] #[doc = " ```"] #[stable (feature = "unix_socket" , since = "1.10.0")] pub fn set_nonblocking (& self , nonblocking : bool) -> io :: Result < () > { self . 0 . set_nonblocking (nonblocking) } #[doc = " Returns the value of the `SO_ERROR` option."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```no_run"] #[doc = " use std::os::unix::net::UnixListener;"] #[doc = ""] #[doc = " fn main() -> std::io::Result<()> {"] #[doc = "     let listener = UnixListener::bind(\"/tmp/sock\")?;"] #[doc = ""] #[doc = "     if let Ok(Some(err)) = listener.take_error() {"] #[doc = "         println!(\"Got error: {err:?}\");"] #[doc = "     }"] #[doc = "     Ok(())"] #[doc = " }"] #[doc = " ```"] #[doc = ""] #[doc = " # Platform specific"] #[doc = " On Redox this always returns `None`."] #[stable (feature = "unix_socket" , since = "1.10.0")] pub fn take_error (& self) -> io :: Result < Option < io :: Error > > { self . 0 . take_error () } #[doc = " Returns an iterator over incoming connections."] #[doc = ""] #[doc = " The iterator will never return [`None`] and will also not yield the"] #[doc = " peer's [`SocketAddr`] structure."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```no_run"] #[doc = " use std::thread;"] #[doc = " use std::os::unix::net::{UnixStream, UnixListener};"] #[doc = ""] #[doc = " fn handle_client(stream: UnixStream) {"] #[doc = "     // ..."] #[doc = " }"] #[doc = ""] #[doc = " fn main() -> std::io::Result<()> {"] #[doc = "     let listener = UnixListener::bind(\"/path/to/the/socket\")?;"] #[doc = ""] #[doc = "     for stream in listener.incoming() {"] #[doc = "         match stream {"] #[doc = "             Ok(stream) => {"] #[doc = "                 thread::spawn(|| handle_client(stream));"] #[doc = "             }"] #[doc = "             Err(err) => {"] #[doc = "                 break;"] #[doc = "             }"] #[doc = "         }"] #[doc = "     }"] #[doc = "     Ok(())"] #[doc = " }"] #[doc = " ```"] #[stable (feature = "unix_socket" , since = "1.10.0")] pub fn incoming (& self) -> Incoming < '_ > { Incoming { listener : self } } }}}
mkitem!{mkimpl!{#[stable (feature = "unix_socket" , since = "1.10.0")] impl AsRawFd for UnixListener { #[inline] fn as_raw_fd (& self) -> RawFd { self . 0 . as_inner () . as_raw_fd () } }}}
mkitem!{mkimpl!{#[stable (feature = "unix_socket" , since = "1.10.0")] impl FromRawFd for UnixListener { #[inline] unsafe fn from_raw_fd (fd : RawFd) -> UnixListener { UnixListener (Socket :: from_inner (FromInner :: from_inner (OwnedFd :: from_raw_fd (fd)))) } }}}
mkitem!{mkimpl!{#[stable (feature = "unix_socket" , since = "1.10.0")] impl IntoRawFd for UnixListener { #[inline] fn into_raw_fd (self) -> RawFd { self . 0 . into_inner () . into_inner () . into_raw_fd () } }}}
mkitem!{mkimpl!{#[stable (feature = "io_safety" , since = "1.63.0")] impl AsFd for UnixListener { #[inline] fn as_fd (& self) -> BorrowedFd < '_ > { self . 0 . as_inner () . as_fd () } }}}
mkitem!{mkimpl!{#[stable (feature = "io_safety" , since = "1.63.0")] impl From < OwnedFd > for UnixListener { #[inline] fn from (fd : OwnedFd) -> UnixListener { UnixListener (Socket :: from_inner (FromInner :: from_inner (fd))) } }}}
mkitem!{mkimpl!{#[stable (feature = "io_safety" , since = "1.63.0")] impl From < UnixListener > for OwnedFd { #[doc = " Takes ownership of a [`UnixListener`]'s socket file descriptor."] #[inline] fn from (listener : UnixListener) -> OwnedFd { listener . 0 . into_inner () . into_inner () } }}}
mkitem!{mkimpl!{#[stable (feature = "unix_socket" , since = "1.10.0")] impl < 'a > IntoIterator for & 'a UnixListener { type Item = io :: Result < UnixStream > ; type IntoIter = Incoming < 'a > ; fn into_iter (self) -> Incoming < 'a > { self . incoming () } }}}
mkitem!{mkstruct!{#[doc = " An iterator over incoming connections to a [`UnixListener`]."] #[doc = ""] #[doc = " It will never return [`None`]."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```no_run"] #[doc = " use std::thread;"] #[doc = " use std::os::unix::net::{UnixStream, UnixListener};"] #[doc = ""] #[doc = " fn handle_client(stream: UnixStream) {"] #[doc = "     // ..."] #[doc = " }"] #[doc = ""] #[doc = " fn main() -> std::io::Result<()> {"] #[doc = "     let listener = UnixListener::bind(\"/path/to/the/socket\")?;"] #[doc = ""] #[doc = "     for stream in listener.incoming() {"] #[doc = "         match stream {"] #[doc = "             Ok(stream) => {"] #[doc = "                 thread::spawn(|| handle_client(stream));"] #[doc = "             }"] #[doc = "             Err(err) => {"] #[doc = "                 break;"] #[doc = "             }"] #[doc = "         }"] #[doc = "     }"] #[doc = "     Ok(())"] #[doc = " }"] #[doc = " ```"] #[derive (Debug)] #[must_use = "iterators are lazy and do nothing unless consumed"] #[stable (feature = "unix_socket" , since = "1.10.0")] pub struct Incoming < 'a > { listener : & 'a UnixListener , }}}
mkitem!{mkimpl!{#[stable (feature = "unix_socket" , since = "1.10.0")] impl < 'a > Iterator for Incoming < 'a > { type Item = io :: Result < UnixStream > ; fn next (& mut self) -> Option < io :: Result < UnixStream > > { Some (self . listener . accept () . map (| s | s . 0)) } fn size_hint (& self) -> (usize , Option < usize >) { (usize :: MAX , None) } }}}