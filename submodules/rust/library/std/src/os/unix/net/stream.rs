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
mkitem!{cfg_select ! { any (target_os = "linux" , target_os = "android" , target_os = "hurd" , target_os = "dragonfly" , target_os = "freebsd" , target_os = "openbsd" , target_os = "netbsd" , target_os = "solaris" , target_os = "illumos" , target_os = "haiku" , target_os = "nto" , target_os = "cygwin" ,) => { use libc :: MSG_NOSIGNAL ; } _ => { const MSG_NOSIGNAL : core :: ffi :: c_int = 0x0 ; } }}
mkuse!{use super :: { SocketAddr , sockaddr_un } ;}
mkuse!{# [cfg (any (doc , target_os = "android" , target_os = "linux" , target_os = "cygwin"))] use super :: { SocketAncillary , recv_vectored_with_ancillary_from , send_vectored_with_ancillary_to } ;}
mkuse!{# [cfg (any (target_os = "android" , target_os = "linux" , target_os = "dragonfly" , target_os = "freebsd" , target_os = "netbsd" , target_os = "openbsd" , target_os = "nto" , target_vendor = "apple" , target_os = "cygwin"))] use super :: { UCred , peer_cred } ;}
mkuse!{use crate :: fmt ;}
mkuse!{use crate :: io :: { self , IoSlice , IoSliceMut } ;}
mkuse!{use crate :: net :: Shutdown ;}
mkuse!{use crate :: os :: unix :: io :: { AsFd , AsRawFd , BorrowedFd , FromRawFd , IntoRawFd , OwnedFd , RawFd } ;}
mkuse!{use crate :: path :: Path ;}
mkuse!{use crate :: sealed :: Sealed ;}
mkuse!{use crate :: sys :: cvt ;}
mkuse!{use crate :: sys :: net :: Socket ;}
mkuse!{use crate :: sys_common :: { AsInner , FromInner } ;}
mkuse!{use crate :: time :: Duration ;}
mkitem!{mkstruct!{# [doc = " A Unix stream socket."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::os::unix::net::UnixStream;"] # [doc = " use std::io::prelude::*;"] # [doc = ""] # [doc = " fn main() -> std::io::Result<()> {"] # [doc = "     let mut stream = UnixStream::connect(\"/path/to/my/socket\")?;"] # [doc = "     stream.write_all(b\"hello world\")?;"] # [doc = "     let mut response = String::new();"] # [doc = "     stream.read_to_string(&mut response)?;"] # [doc = "     println!(\"{response}\");"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " # `SIGPIPE`"] # [doc = ""] # [doc = " Writes to the underlying socket in `SOCK_STREAM` mode are made with `MSG_NOSIGNAL` flag."] # [doc = " This suppresses the emission of the  `SIGPIPE` signal when writing to disconnected socket."] # [doc = " In some cases getting a `SIGPIPE` would trigger process termination."] # [stable (feature = "unix_socket" , since = "1.10.0")] pub struct UnixStream (pub (super) Socket) ;}}
mkitem!{mkimpl!{# [doc = " Allows extension traits within `std`."] # [unstable (feature = "sealed" , issue = "none")] impl Sealed for UnixStream { }}}
mkitem!{mkimpl!{# [stable (feature = "unix_socket" , since = "1.10.0")] impl fmt :: Debug for UnixStream { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut builder = fmt . debug_struct ("UnixStream") ; builder . field ("fd" , self . 0 . as_inner ()) ; if let Ok (addr) = self . local_addr () { builder . field ("local" , & addr) ; } if let Ok (addr) = self . peer_addr () { builder . field ("peer" , & addr) ; } builder . finish () } }}}
mkitem!{mkimpl!{impl UnixStream { # [doc = " Connects to the socket named by `path`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::os::unix::net::UnixStream;"] # [doc = ""] # [doc = " let socket = match UnixStream::connect(\"/tmp/sock\") {"] # [doc = "     Ok(sock) => sock,"] # [doc = "     Err(e) => {"] # [doc = "         println!(\"Couldn't connect: {e:?}\");"] # [doc = "         return"] # [doc = "     }"] # [doc = " };"] # [doc = " ```"] # [stable (feature = "unix_socket" , since = "1.10.0")] pub fn connect < P : AsRef < Path > > (path : P) -> io :: Result < UnixStream > { unsafe { let inner = Socket :: new_raw (libc :: AF_UNIX , libc :: SOCK_STREAM) ? ; let (addr , len) = sockaddr_un (path . as_ref ()) ? ; cvt (libc :: connect (inner . as_raw_fd () , (& raw const addr) as * const _ , len)) ? ; Ok (UnixStream (inner)) } } # [doc = " Connects to the socket specified by [`address`]."] # [doc = ""] # [doc = " [`address`]: crate::os::unix::net::SocketAddr"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::os::unix::net::{UnixListener, UnixStream};"] # [doc = ""] # [doc = " fn main() -> std::io::Result<()> {"] # [doc = "     let listener = UnixListener::bind(\"/path/to/the/socket\")?;"] # [doc = "     let addr = listener.local_addr()?;"] # [doc = ""] # [doc = "     let sock = match UnixStream::connect_addr(&addr) {"] # [doc = "         Ok(sock) => sock,"] # [doc = "         Err(e) => {"] # [doc = "             println!(\"Couldn't connect: {e:?}\");"] # [doc = "             return Err(e)"] # [doc = "         }"] # [doc = "     };"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ````"] # [stable (feature = "unix_socket_abstract" , since = "1.70.0")] pub fn connect_addr (socket_addr : & SocketAddr) -> io :: Result < UnixStream > { unsafe { let inner = Socket :: new_raw (libc :: AF_UNIX , libc :: SOCK_STREAM) ? ; cvt (libc :: connect (inner . as_raw_fd () , (& raw const socket_addr . addr) as * const _ , socket_addr . len ,)) ? ; Ok (UnixStream (inner)) } } # [doc = " Creates an unnamed pair of connected sockets."] # [doc = ""] # [doc = " Returns two `UnixStream`s which are connected to each other."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::os::unix::net::UnixStream;"] # [doc = ""] # [doc = " let (sock1, sock2) = match UnixStream::pair() {"] # [doc = "     Ok((sock1, sock2)) => (sock1, sock2),"] # [doc = "     Err(e) => {"] # [doc = "         println!(\"Couldn't create a pair of sockets: {e:?}\");"] # [doc = "         return"] # [doc = "     }"] # [doc = " };"] # [doc = " ```"] # [stable (feature = "unix_socket" , since = "1.10.0")] pub fn pair () -> io :: Result < (UnixStream , UnixStream) > { let (i1 , i2) = Socket :: new_pair (libc :: AF_UNIX , libc :: SOCK_STREAM) ? ; Ok ((UnixStream (i1) , UnixStream (i2))) } # [doc = " Creates a new independently owned handle to the underlying socket."] # [doc = ""] # [doc = " The returned `UnixStream` is a reference to the same stream that this"] # [doc = " object references. Both handles will read and write the same stream of"] # [doc = " data, and options set on one stream will be propagated to the other"] # [doc = " stream."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::os::unix::net::UnixStream;"] # [doc = ""] # [doc = " fn main() -> std::io::Result<()> {"] # [doc = "     let socket = UnixStream::connect(\"/tmp/sock\")?;"] # [doc = "     let sock_copy = socket.try_clone().expect(\"Couldn't clone socket\");"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [stable (feature = "unix_socket" , since = "1.10.0")] pub fn try_clone (& self) -> io :: Result < UnixStream > { self . 0 . duplicate () . map (UnixStream) } # [doc = " Returns the socket address of the local half of this connection."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::os::unix::net::UnixStream;"] # [doc = ""] # [doc = " fn main() -> std::io::Result<()> {"] # [doc = "     let socket = UnixStream::connect(\"/tmp/sock\")?;"] # [doc = "     let addr = socket.local_addr().expect(\"Couldn't get local address\");"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [stable (feature = "unix_socket" , since = "1.10.0")] pub fn local_addr (& self) -> io :: Result < SocketAddr > { SocketAddr :: new (| addr , len | unsafe { libc :: getsockname (self . as_raw_fd () , addr , len) }) } # [doc = " Returns the socket address of the remote half of this connection."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::os::unix::net::UnixStream;"] # [doc = ""] # [doc = " fn main() -> std::io::Result<()> {"] # [doc = "     let socket = UnixStream::connect(\"/tmp/sock\")?;"] # [doc = "     let addr = socket.peer_addr().expect(\"Couldn't get peer address\");"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [stable (feature = "unix_socket" , since = "1.10.0")] pub fn peer_addr (& self) -> io :: Result < SocketAddr > { SocketAddr :: new (| addr , len | unsafe { libc :: getpeername (self . as_raw_fd () , addr , len) }) } # [doc = " Gets the peer credentials for this Unix domain socket."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " #![feature(peer_credentials_unix_socket)]"] # [doc = " use std::os::unix::net::UnixStream;"] # [doc = ""] # [doc = " fn main() -> std::io::Result<()> {"] # [doc = "     let socket = UnixStream::connect(\"/tmp/sock\")?;"] # [doc = "     let peer_cred = socket.peer_cred().expect(\"Couldn't get peer credentials\");"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [unstable (feature = "peer_credentials_unix_socket" , issue = "42839" , reason = "unstable")] # [cfg (any (target_os = "android" , target_os = "linux" , target_os = "dragonfly" , target_os = "freebsd" , target_os = "netbsd" , target_os = "openbsd" , target_os = "nto" , target_vendor = "apple" , target_os = "cygwin"))] pub fn peer_cred (& self) -> io :: Result < UCred > { peer_cred (self) } # [doc = " Sets the read timeout for the socket."] # [doc = ""] # [doc = " If the provided value is [`None`], then [`read`] calls will block"] # [doc = " indefinitely. An [`Err`] is returned if the zero [`Duration`] is passed to this"] # [doc = " method."] # [doc = ""] # [doc = " [`read`]: io::Read::read"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::os::unix::net::UnixStream;"] # [doc = " use std::time::Duration;"] # [doc = ""] # [doc = " fn main() -> std::io::Result<()> {"] # [doc = "     let socket = UnixStream::connect(\"/tmp/sock\")?;"] # [doc = "     socket.set_read_timeout(Some(Duration::new(1, 0))).expect(\"Couldn't set read timeout\");"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " An [`Err`] is returned if the zero [`Duration`] is passed to this"] # [doc = " method:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::io;"] # [doc = " use std::os::unix::net::UnixStream;"] # [doc = " use std::time::Duration;"] # [doc = ""] # [doc = " fn main() -> std::io::Result<()> {"] # [doc = "     let socket = UnixStream::connect(\"/tmp/sock\")?;"] # [doc = "     let result = socket.set_read_timeout(Some(Duration::new(0, 0)));"] # [doc = "     let err = result.unwrap_err();"] # [doc = "     assert_eq!(err.kind(), io::ErrorKind::InvalidInput);"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [stable (feature = "unix_socket" , since = "1.10.0")] pub fn set_read_timeout (& self , timeout : Option < Duration >) -> io :: Result < () > { self . 0 . set_timeout (timeout , libc :: SO_RCVTIMEO) } # [doc = " Sets the write timeout for the socket."] # [doc = ""] # [doc = " If the provided value is [`None`], then [`write`] calls will block"] # [doc = " indefinitely. An [`Err`] is returned if the zero [`Duration`] is"] # [doc = " passed to this method."] # [doc = ""] # [doc = " [`read`]: io::Read::read"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::os::unix::net::UnixStream;"] # [doc = " use std::time::Duration;"] # [doc = ""] # [doc = " fn main() -> std::io::Result<()> {"] # [doc = "     let socket = UnixStream::connect(\"/tmp/sock\")?;"] # [doc = "     socket.set_write_timeout(Some(Duration::new(1, 0)))"] # [doc = "         .expect(\"Couldn't set write timeout\");"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " An [`Err`] is returned if the zero [`Duration`] is passed to this"] # [doc = " method:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::io;"] # [doc = " use std::os::unix::net::UnixStream;"] # [doc = " use std::time::Duration;"] # [doc = ""] # [doc = " fn main() -> std::io::Result<()> {"] # [doc = "     let socket = UnixStream::connect(\"/tmp/sock\")?;"] # [doc = "     let result = socket.set_write_timeout(Some(Duration::new(0, 0)));"] # [doc = "     let err = result.unwrap_err();"] # [doc = "     assert_eq!(err.kind(), io::ErrorKind::InvalidInput);"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [stable (feature = "unix_socket" , since = "1.10.0")] pub fn set_write_timeout (& self , timeout : Option < Duration >) -> io :: Result < () > { self . 0 . set_timeout (timeout , libc :: SO_SNDTIMEO) } # [doc = " Returns the read timeout of this socket."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::os::unix::net::UnixStream;"] # [doc = " use std::time::Duration;"] # [doc = ""] # [doc = " fn main() -> std::io::Result<()> {"] # [doc = "     let socket = UnixStream::connect(\"/tmp/sock\")?;"] # [doc = "     socket.set_read_timeout(Some(Duration::new(1, 0))).expect(\"Couldn't set read timeout\");"] # [doc = "     assert_eq!(socket.read_timeout()?, Some(Duration::new(1, 0)));"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [stable (feature = "unix_socket" , since = "1.10.0")] pub fn read_timeout (& self) -> io :: Result < Option < Duration > > { self . 0 . timeout (libc :: SO_RCVTIMEO) } # [doc = " Returns the write timeout of this socket."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::os::unix::net::UnixStream;"] # [doc = " use std::time::Duration;"] # [doc = ""] # [doc = " fn main() -> std::io::Result<()> {"] # [doc = "     let socket = UnixStream::connect(\"/tmp/sock\")?;"] # [doc = "     socket.set_write_timeout(Some(Duration::new(1, 0)))"] # [doc = "         .expect(\"Couldn't set write timeout\");"] # [doc = "     assert_eq!(socket.write_timeout()?, Some(Duration::new(1, 0)));"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [stable (feature = "unix_socket" , since = "1.10.0")] pub fn write_timeout (& self) -> io :: Result < Option < Duration > > { self . 0 . timeout (libc :: SO_SNDTIMEO) } # [doc = " Moves the socket into or out of nonblocking mode."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::os::unix::net::UnixStream;"] # [doc = ""] # [doc = " fn main() -> std::io::Result<()> {"] # [doc = "     let socket = UnixStream::connect(\"/tmp/sock\")?;"] # [doc = "     socket.set_nonblocking(true).expect(\"Couldn't set nonblocking\");"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [stable (feature = "unix_socket" , since = "1.10.0")] pub fn set_nonblocking (& self , nonblocking : bool) -> io :: Result < () > { self . 0 . set_nonblocking (nonblocking) } # [doc = " Set the id of the socket for network filtering purpose"] # [doc = ""] # [cfg_attr (any (target_os = "linux" , target_os = "freebsd" , target_os = "openbsd") , doc = "```no_run")] # [cfg_attr (not (any (target_os = "linux" , target_os = "freebsd" , target_os = "openbsd")) , doc = "```ignore")] # [doc = " #![feature(unix_set_mark)]"] # [doc = " use std::os::unix::net::UnixStream;"] # [doc = ""] # [doc = " fn main() -> std::io::Result<()> {"] # [doc = "     let sock = UnixStream::connect(\"/tmp/sock\")?;"] # [doc = "     sock.set_mark(32)?;"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [cfg (any (doc , target_os = "linux" , target_os = "freebsd" , target_os = "openbsd" ,))] # [unstable (feature = "unix_set_mark" , issue = "96467")] pub fn set_mark (& self , mark : u32) -> io :: Result < () > { self . 0 . set_mark (mark) } # [doc = " Returns the value of the `SO_ERROR` option."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::os::unix::net::UnixStream;"] # [doc = ""] # [doc = " fn main() -> std::io::Result<()> {"] # [doc = "     let socket = UnixStream::connect(\"/tmp/sock\")?;"] # [doc = "     if let Ok(Some(err)) = socket.take_error() {"] # [doc = "         println!(\"Got error: {err:?}\");"] # [doc = "     }"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " # Platform specific"] # [doc = " On Redox this always returns `None`."] # [stable (feature = "unix_socket" , since = "1.10.0")] pub fn take_error (& self) -> io :: Result < Option < io :: Error > > { self . 0 . take_error () } # [doc = " Shuts down the read, write, or both halves of this connection."] # [doc = ""] # [doc = " This function will cause all pending and future I/O calls on the"] # [doc = " specified portions to immediately return with an appropriate value"] # [doc = " (see the documentation of [`Shutdown`])."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::os::unix::net::UnixStream;"] # [doc = " use std::net::Shutdown;"] # [doc = ""] # [doc = " fn main() -> std::io::Result<()> {"] # [doc = "     let socket = UnixStream::connect(\"/tmp/sock\")?;"] # [doc = "     socket.shutdown(Shutdown::Both).expect(\"shutdown function failed\");"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [stable (feature = "unix_socket" , since = "1.10.0")] pub fn shutdown (& self , how : Shutdown) -> io :: Result < () > { self . 0 . shutdown (how) } # [doc = " Receives data on the socket from the remote address to which it is"] # [doc = " connected, without removing that data from the queue. On success,"] # [doc = " returns the number of bytes peeked."] # [doc = ""] # [doc = " Successive calls return the same data. This is accomplished by passing"] # [doc = " `MSG_PEEK` as a flag to the underlying `recv` system call."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " #![feature(unix_socket_peek)]"] # [doc = ""] # [doc = " use std::os::unix::net::UnixStream;"] # [doc = ""] # [doc = " fn main() -> std::io::Result<()> {"] # [doc = "     let socket = UnixStream::connect(\"/tmp/sock\")?;"] # [doc = "     let mut buf = [0; 10];"] # [doc = "     let len = socket.peek(&mut buf).expect(\"peek failed\");"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [unstable (feature = "unix_socket_peek" , issue = "76923")] pub fn peek (& self , buf : & mut [u8]) -> io :: Result < usize > { self . 0 . peek (buf) } # [doc = " Receives data and ancillary data from socket."] # [doc = ""] # [doc = " On success, returns the number of bytes read."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [cfg_attr (any (target_os = "android" , target_os = "linux" , target_os = "cygwin") , doc = "```no_run")] # [cfg_attr (not (any (target_os = "android" , target_os = "linux" , target_os = "cygwin")) , doc = "```ignore")] # [doc = " #![feature(unix_socket_ancillary_data)]"] # [doc = " use std::os::unix::net::{UnixStream, SocketAncillary, AncillaryData};"] # [doc = " use std::io::IoSliceMut;"] # [doc = ""] # [doc = " fn main() -> std::io::Result<()> {"] # [doc = "     let socket = UnixStream::connect(\"/tmp/sock\")?;"] # [doc = "     let mut buf1 = [1; 8];"] # [doc = "     let mut buf2 = [2; 16];"] # [doc = "     let mut buf3 = [3; 8];"] # [doc = "     let mut bufs = &mut ["] # [doc = "         IoSliceMut::new(&mut buf1),"] # [doc = "         IoSliceMut::new(&mut buf2),"] # [doc = "         IoSliceMut::new(&mut buf3),"] # [doc = "     ][..];"] # [doc = "     let mut fds = [0; 8];"] # [doc = "     let mut ancillary_buffer = [0; 128];"] # [doc = "     let mut ancillary = SocketAncillary::new(&mut ancillary_buffer[..]);"] # [doc = "     let size = socket.recv_vectored_with_ancillary(bufs, &mut ancillary)?;"] # [doc = "     println!(\"received {size}\");"] # [doc = "     for ancillary_result in ancillary.messages() {"] # [doc = "         if let AncillaryData::ScmRights(scm_rights) = ancillary_result.unwrap() {"] # [doc = "             for fd in scm_rights {"] # [doc = "                 println!(\"receive file descriptor: {fd}\");"] # [doc = "             }"] # [doc = "         }"] # [doc = "     }"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [cfg (any (doc , target_os = "android" , target_os = "linux" , target_os = "cygwin"))] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] pub fn recv_vectored_with_ancillary (& self , bufs : & mut [IoSliceMut < '_ >] , ancillary : & mut SocketAncillary < '_ > ,) -> io :: Result < usize > { let (count , _ , _) = recv_vectored_with_ancillary_from (& self . 0 , bufs , ancillary) ? ; Ok (count) } # [doc = " Sends data and ancillary data on the socket."] # [doc = ""] # [doc = " On success, returns the number of bytes written."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [cfg_attr (any (target_os = "android" , target_os = "linux" , target_os = "cygwin") , doc = "```no_run")] # [cfg_attr (not (any (target_os = "android" , target_os = "linux" , target_os = "cygwin")) , doc = "```ignore")] # [doc = " #![feature(unix_socket_ancillary_data)]"] # [doc = " use std::os::unix::net::{UnixStream, SocketAncillary};"] # [doc = " use std::io::IoSlice;"] # [doc = ""] # [doc = " fn main() -> std::io::Result<()> {"] # [doc = "     let socket = UnixStream::connect(\"/tmp/sock\")?;"] # [doc = "     let buf1 = [1; 8];"] # [doc = "     let buf2 = [2; 16];"] # [doc = "     let buf3 = [3; 8];"] # [doc = "     let bufs = &["] # [doc = "         IoSlice::new(&buf1),"] # [doc = "         IoSlice::new(&buf2),"] # [doc = "         IoSlice::new(&buf3),"] # [doc = "     ][..];"] # [doc = "     let fds = [0, 1, 2];"] # [doc = "     let mut ancillary_buffer = [0; 128];"] # [doc = "     let mut ancillary = SocketAncillary::new(&mut ancillary_buffer[..]);"] # [doc = "     ancillary.add_fds(&fds[..]);"] # [doc = "     socket.send_vectored_with_ancillary(bufs, &mut ancillary)"] # [doc = "         .expect(\"send_vectored_with_ancillary function failed\");"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [cfg (any (doc , target_os = "android" , target_os = "linux" , target_os = "cygwin"))] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] pub fn send_vectored_with_ancillary (& self , bufs : & [IoSlice < '_ >] , ancillary : & mut SocketAncillary < '_ > ,) -> io :: Result < usize > { send_vectored_with_ancillary_to (& self . 0 , None , bufs , ancillary) } }}}
mkitem!{mkimpl!{# [stable (feature = "unix_socket" , since = "1.10.0")] impl io :: Read for UnixStream { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { io :: Read :: read (& mut & * self , buf) } fn read_buf (& mut self , buf : io :: BorrowedCursor < '_ >) -> io :: Result < () > { io :: Read :: read_buf (& mut & * self , buf) } fn read_vectored (& mut self , bufs : & mut [IoSliceMut < '_ >]) -> io :: Result < usize > { io :: Read :: read_vectored (& mut & * self , bufs) } # [inline] fn is_read_vectored (& self) -> bool { io :: Read :: is_read_vectored (& & * self) } }}}
mkitem!{mkimpl!{# [stable (feature = "unix_socket" , since = "1.10.0")] impl < 'a > io :: Read for & 'a UnixStream { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { self . 0 . read (buf) } fn read_buf (& mut self , buf : io :: BorrowedCursor < '_ >) -> io :: Result < () > { self . 0 . read_buf (buf) } fn read_vectored (& mut self , bufs : & mut [IoSliceMut < '_ >]) -> io :: Result < usize > { self . 0 . read_vectored (bufs) } # [inline] fn is_read_vectored (& self) -> bool { self . 0 . is_read_vectored () } }}}
mkitem!{mkimpl!{# [stable (feature = "unix_socket" , since = "1.10.0")] impl io :: Write for UnixStream { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { io :: Write :: write (& mut & * self , buf) } fn write_vectored (& mut self , bufs : & [IoSlice < '_ >]) -> io :: Result < usize > { io :: Write :: write_vectored (& mut & * self , bufs) } # [inline] fn is_write_vectored (& self) -> bool { io :: Write :: is_write_vectored (& & * self) } fn flush (& mut self) -> io :: Result < () > { io :: Write :: flush (& mut & * self) } }}}
mkitem!{mkimpl!{# [stable (feature = "unix_socket" , since = "1.10.0")] impl < 'a > io :: Write for & 'a UnixStream { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . 0 . send_with_flags (buf , MSG_NOSIGNAL) } fn write_vectored (& mut self , bufs : & [IoSlice < '_ >]) -> io :: Result < usize > { self . 0 . write_vectored (bufs) } # [inline] fn is_write_vectored (& self) -> bool { self . 0 . is_write_vectored () } # [inline] fn flush (& mut self) -> io :: Result < () > { Ok (()) } }}}
mkitem!{mkimpl!{# [stable (feature = "unix_socket" , since = "1.10.0")] impl AsRawFd for UnixStream { # [inline] fn as_raw_fd (& self) -> RawFd { self . 0 . as_raw_fd () } }}}
mkitem!{mkimpl!{# [stable (feature = "unix_socket" , since = "1.10.0")] impl FromRawFd for UnixStream { # [inline] unsafe fn from_raw_fd (fd : RawFd) -> UnixStream { UnixStream (Socket :: from_inner (FromInner :: from_inner (OwnedFd :: from_raw_fd (fd)))) } }}}
mkitem!{mkimpl!{# [stable (feature = "unix_socket" , since = "1.10.0")] impl IntoRawFd for UnixStream { # [inline] fn into_raw_fd (self) -> RawFd { self . 0 . into_raw_fd () } }}}
mkitem!{mkimpl!{# [stable (feature = "io_safety" , since = "1.63.0")] impl AsFd for UnixStream { # [inline] fn as_fd (& self) -> BorrowedFd < '_ > { self . 0 . as_fd () } }}}
mkitem!{mkimpl!{# [stable (feature = "io_safety" , since = "1.63.0")] impl From < UnixStream > for OwnedFd { # [doc = " Takes ownership of a [`UnixStream`]'s socket file descriptor."] # [inline] fn from (unix_stream : UnixStream) -> OwnedFd { unsafe { OwnedFd :: from_raw_fd (unix_stream . into_raw_fd ()) } } }}}
mkitem!{mkimpl!{# [stable (feature = "io_safety" , since = "1.63.0")] impl From < OwnedFd > for UnixStream { # [inline] fn from (owned : OwnedFd) -> Self { unsafe { Self :: from_raw_fd (owned . into_raw_fd ()) } } }}}
mkitem!{mkimpl!{impl AsInner < Socket > for UnixStream { # [inline] fn as_inner (& self) -> & Socket { & self . 0 } }}}