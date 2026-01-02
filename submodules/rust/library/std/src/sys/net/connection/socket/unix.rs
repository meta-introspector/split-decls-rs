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
mkuse!{use libc :: { MSG_PEEK , c_int , c_void , size_t , sockaddr , socklen_t } ;}
mkuse!{#[cfg (not (any (target_os = "espidf" , target_os = "nuttx")))] use crate :: ffi :: CStr ;}
mkuse!{use crate :: io :: { self , BorrowedBuf , BorrowedCursor , IoSlice , IoSliceMut } ;}
mkuse!{use crate :: net :: { Shutdown , SocketAddr } ;}
mkuse!{use crate :: os :: unix :: io :: { AsFd , AsRawFd , BorrowedFd , FromRawFd , IntoRawFd , RawFd } ;}
mkuse!{use crate :: sys :: fd :: FileDesc ;}
mkuse!{use crate :: sys :: net :: { getsockopt , setsockopt } ;}
mkuse!{use crate :: sys :: pal :: IsMinusOne ;}
mkuse!{use crate :: sys_common :: { AsInner , FromInner , IntoInner } ;}
mkuse!{use crate :: time :: { Duration , Instant } ;}
mkuse!{use crate :: { cmp , mem } ;}
mkitem!{cfg_select ! { target_vendor = "apple" => { use libc :: SO_LINGER_SEC as SO_LINGER ; } _ => { use libc :: SO_LINGER ; } }}
mkuse!{pub (super) use libc as netc ;}
mkuse!{use super :: { socket_addr_from_c , socket_addr_to_c } ;}
mkuse!{pub use crate :: sys :: { cvt , cvt_r } ;}
mkitem!{#[expect (non_camel_case_types)] pub type wrlen_t = size_t ;}
mkitem!{mkstruct!{pub struct Socket (FileDesc) ;}}

macro_rules! init_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function init in module {}", module_path!());
    };
}

mkfn!{
    init_introspect!();
    pub fn init () { }
}

macro_rules! cvt_gai_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function cvt_gai in module {}", module_path!());
    };
}

mkfn!{
    cvt_gai_introspect!();
    pub fn cvt_gai (err : c_int) -> io :: Result < () > { if err == 0 { return Ok (()) ; } on_resolver_failure () ; #[cfg (not (any (target_os = "espidf" , target_os = "nuttx")))] if err == libc :: EAI_SYSTEM { return Err (io :: Error :: last_os_error ()) ; } #[cfg (not (any (target_os = "espidf" , target_os = "nuttx")))] let detail = unsafe { CStr :: from_ptr (libc :: gai_strerror (err)) . to_string_lossy () } ; #[cfg (any (target_os = "espidf" , target_os = "nuttx"))] let detail = "" ; Err (io :: Error :: new (io :: ErrorKind :: Uncategorized , & format ! ("failed to lookup address information: {detail}") [..] ,)) }
}
mkitem!{mkimpl!{impl Socket { pub fn new (addr : & SocketAddr , ty : c_int) -> io :: Result < Socket > { let fam = match * addr { SocketAddr :: V4 (..) => libc :: AF_INET , SocketAddr :: V6 (..) => libc :: AF_INET6 , } ; Socket :: new_raw (fam , ty) } pub fn new_raw (fam : c_int , ty : c_int) -> io :: Result < Socket > { unsafe { cfg_select ! { any (target_os = "android" , target_os = "dragonfly" , target_os = "freebsd" , target_os = "illumos" , target_os = "hurd" , target_os = "linux" , target_os = "netbsd" , target_os = "openbsd" , target_os = "cygwin" , target_os = "nto" , target_os = "solaris" ,) => { let fd = cvt (libc :: socket (fam , ty | libc :: SOCK_CLOEXEC , 0)) ?; let socket = Socket (FileDesc :: from_raw_fd (fd)) ; #[cfg (any (target_os = "freebsd" , target_os = "netbsd" , target_os = "dragonfly"))] setsockopt (& socket , libc :: SOL_SOCKET , libc :: SO_NOSIGPIPE , 1) ?; Ok (socket) } _ => { let fd = cvt (libc :: socket (fam , ty , 0)) ?; let fd = FileDesc :: from_raw_fd (fd) ; fd . set_cloexec () ?; let socket = Socket (fd) ; #[cfg (target_vendor = "apple")] setsockopt (& socket , libc :: SOL_SOCKET , libc :: SO_NOSIGPIPE , 1) ?; Ok (socket) } } } } #[cfg (not (target_os = "vxworks"))] pub fn new_pair (fam : c_int , ty : c_int) -> io :: Result < (Socket , Socket) > { unsafe { let mut fds = [0 , 0] ; cfg_select ! { any (target_os = "android" , target_os = "dragonfly" , target_os = "freebsd" , target_os = "illumos" , target_os = "linux" , target_os = "hurd" , target_os = "netbsd" , target_os = "openbsd" , target_os = "cygwin" , target_os = "nto" ,) => { cvt (libc :: socketpair (fam , ty | libc :: SOCK_CLOEXEC , 0 , fds . as_mut_ptr ())) ?; Ok ((Socket (FileDesc :: from_raw_fd (fds [0])) , Socket (FileDesc :: from_raw_fd (fds [1])))) } _ => { cvt (libc :: socketpair (fam , ty , 0 , fds . as_mut_ptr ())) ?; let a = FileDesc :: from_raw_fd (fds [0]) ; let b = FileDesc :: from_raw_fd (fds [1]) ; a . set_cloexec () ?; b . set_cloexec () ?; Ok ((Socket (a) , Socket (b))) } } } } #[cfg (target_os = "vxworks")] pub fn new_pair (_fam : c_int , _ty : c_int) -> io :: Result < (Socket , Socket) > { unimplemented ! () } pub fn connect (& self , addr : & SocketAddr) -> io :: Result < () > { let (addr , len) = socket_addr_to_c (addr) ; loop { let result = unsafe { libc :: connect (self . as_raw_fd () , addr . as_ptr () , len) } ; if result . is_minus_one () { let err = crate :: sys :: os :: errno () ; match err { libc :: EINTR => continue , libc :: EISCONN => return Ok (()) , _ => return Err (io :: Error :: from_raw_os_error (err)) , } } return Ok (()) ; } } pub fn connect_timeout (& self , addr : & SocketAddr , timeout : Duration) -> io :: Result < () > { self . set_nonblocking (true) ? ; let r = unsafe { let (addr , len) = socket_addr_to_c (addr) ; cvt (libc :: connect (self . as_raw_fd () , addr . as_ptr () , len)) } ; self . set_nonblocking (false) ? ; match r { Ok (_) => return Ok (()) , Err (ref e) if e . raw_os_error () == Some (libc :: EINPROGRESS) => { } Err (e) => return Err (e) , } let mut pollfd = libc :: pollfd { fd : self . as_raw_fd () , events : libc :: POLLOUT , revents : 0 } ; if timeout . as_secs () == 0 && timeout . subsec_nanos () == 0 { return Err (io :: Error :: ZERO_TIMEOUT) ; } let start = Instant :: now () ; loop { let elapsed = start . elapsed () ; if elapsed >= timeout { return Err (io :: const_error ! (io :: ErrorKind :: TimedOut , "connection timed out")) ; } let timeout = timeout - elapsed ; let mut timeout = timeout . as_secs () . saturating_mul (1_000) . saturating_add (timeout . subsec_nanos () as u64 / 1_000_000) ; if timeout == 0 { timeout = 1 ; } let timeout = cmp :: min (timeout , c_int :: MAX as u64) as c_int ; match unsafe { libc :: poll (& mut pollfd , 1 , timeout) } { - 1 => { let err = io :: Error :: last_os_error () ; if ! err . is_interrupted () { return Err (err) ; } } 0 => { } _ => { if cfg ! (target_os = "vxworks") { if let Some (e) = self . take_error () ? { return Err (e) ; } } else { if pollfd . revents & (libc :: POLLHUP | libc :: POLLERR) != 0 { let e = self . take_error () ? . unwrap_or_else (| | { io :: const_error ! (io :: ErrorKind :: Uncategorized , "no error set after POLLHUP" ,) }) ; return Err (e) ; } } return Ok (()) ; } } } } pub fn accept (& self , storage : * mut sockaddr , len : * mut socklen_t) -> io :: Result < Socket > { cfg_select ! { any (target_os = "android" , target_os = "dragonfly" , target_os = "freebsd" , target_os = "illumos" , target_os = "linux" , target_os = "hurd" , target_os = "netbsd" , target_os = "openbsd" , target_os = "cygwin" ,) => { unsafe { let fd = cvt_r (|| libc :: accept4 (self . as_raw_fd () , storage , len , libc :: SOCK_CLOEXEC)) ?; Ok (Socket (FileDesc :: from_raw_fd (fd))) } } _ => { unsafe { let fd = cvt_r (|| libc :: accept (self . as_raw_fd () , storage , len)) ?; let fd = FileDesc :: from_raw_fd (fd) ; fd . set_cloexec () ?; Ok (Socket (fd)) } } } } pub fn duplicate (& self) -> io :: Result < Socket > { self . 0 . duplicate () . map (Socket) } pub fn send_with_flags (& self , buf : & [u8] , flags : c_int) -> io :: Result < usize > { let len = cmp :: min (buf . len () , < wrlen_t > :: MAX as usize) as wrlen_t ; let ret = cvt (unsafe { libc :: send (self . as_raw_fd () , buf . as_ptr () as * const c_void , len , flags) }) ? ; Ok (ret as usize) } fn recv_with_flags (& self , mut buf : BorrowedCursor < '_ > , flags : c_int) -> io :: Result < () > { let ret = cvt (unsafe { libc :: recv (self . as_raw_fd () , buf . as_mut () . as_mut_ptr () as * mut c_void , buf . capacity () , flags ,) }) ? ; unsafe { buf . advance_unchecked (ret as usize) ; } Ok (()) } pub fn read (& self , buf : & mut [u8]) -> io :: Result < usize > { let mut buf = BorrowedBuf :: from (buf) ; self . recv_with_flags (buf . unfilled () , 0) ? ; Ok (buf . len ()) } pub fn peek (& self , buf : & mut [u8]) -> io :: Result < usize > { let mut buf = BorrowedBuf :: from (buf) ; self . recv_with_flags (buf . unfilled () , MSG_PEEK) ? ; Ok (buf . len ()) } pub fn read_buf (& self , buf : BorrowedCursor < '_ >) -> io :: Result < () > { self . recv_with_flags (buf , 0) } pub fn read_vectored (& self , bufs : & mut [IoSliceMut < '_ >]) -> io :: Result < usize > { self . 0 . read_vectored (bufs) } #[inline] pub fn is_read_vectored (& self) -> bool { self . 0 . is_read_vectored () } fn recv_from_with_flags (& self , buf : & mut [u8] , flags : c_int ,) -> io :: Result < (usize , SocketAddr) > { let mut storage : mem :: MaybeUninit < libc :: sockaddr_storage > = mem :: MaybeUninit :: uninit () ; let mut addrlen = size_of_val (& storage) as libc :: socklen_t ; let n = cvt (unsafe { libc :: recvfrom (self . as_raw_fd () , buf . as_mut_ptr () as * mut c_void , buf . len () , flags , (& raw mut storage) as * mut _ , & mut addrlen ,) }) ? ; Ok ((n as usize , unsafe { socket_addr_from_c (storage . as_ptr () , addrlen as usize) ? })) } pub fn recv_from (& self , buf : & mut [u8]) -> io :: Result < (usize , SocketAddr) > { self . recv_from_with_flags (buf , 0) } #[cfg (any (target_os = "android" , target_os = "linux" , target_os = "cygwin"))] pub fn recv_msg (& self , msg : & mut libc :: msghdr) -> io :: Result < usize > { let n = cvt (unsafe { libc :: recvmsg (self . as_raw_fd () , msg , libc :: MSG_CMSG_CLOEXEC) }) ? ; Ok (n as usize) } pub fn peek_from (& self , buf : & mut [u8]) -> io :: Result < (usize , SocketAddr) > { self . recv_from_with_flags (buf , MSG_PEEK) } pub fn write (& self , buf : & [u8]) -> io :: Result < usize > { self . 0 . write (buf) } pub fn write_vectored (& self , bufs : & [IoSlice < '_ >]) -> io :: Result < usize > { self . 0 . write_vectored (bufs) } #[inline] pub fn is_write_vectored (& self) -> bool { self . 0 . is_write_vectored () } #[cfg (any (target_os = "android" , target_os = "linux" , target_os = "cygwin"))] pub fn send_msg (& self , msg : & mut libc :: msghdr) -> io :: Result < usize > { let n = cvt (unsafe { libc :: sendmsg (self . as_raw_fd () , msg , 0) }) ? ; Ok (n as usize) } pub fn set_timeout (& self , dur : Option < Duration > , kind : libc :: c_int) -> io :: Result < () > { let timeout = match dur { Some (dur) => { if dur . as_secs () == 0 && dur . subsec_nanos () == 0 { return Err (io :: Error :: ZERO_TIMEOUT) ; } let secs = if dur . as_secs () > libc :: time_t :: MAX as u64 { libc :: time_t :: MAX } else { dur . as_secs () as libc :: time_t } ; let mut timeout = libc :: timeval { tv_sec : secs , tv_usec : dur . subsec_micros () as libc :: suseconds_t , } ; if timeout . tv_sec == 0 && timeout . tv_usec == 0 { timeout . tv_usec = 1 ; } timeout } None => libc :: timeval { tv_sec : 0 , tv_usec : 0 } , } ; setsockopt (self , libc :: SOL_SOCKET , kind , timeout) } pub fn timeout (& self , kind : libc :: c_int) -> io :: Result < Option < Duration > > { let raw : libc :: timeval = getsockopt (self , libc :: SOL_SOCKET , kind) ? ; if raw . tv_sec == 0 && raw . tv_usec == 0 { Ok (None) } else { let sec = raw . tv_sec as u64 ; let nsec = (raw . tv_usec as u32) * 1000 ; Ok (Some (Duration :: new (sec , nsec))) } } pub fn shutdown (& self , how : Shutdown) -> io :: Result < () > { let how = match how { Shutdown :: Write => libc :: SHUT_WR , Shutdown :: Read => libc :: SHUT_RD , Shutdown :: Both => libc :: SHUT_RDWR , } ; cvt (unsafe { libc :: shutdown (self . as_raw_fd () , how) }) ? ; Ok (()) } #[cfg (not (target_os = "cygwin"))] pub fn set_linger (& self , linger : Option < Duration >) -> io :: Result < () > { let linger = libc :: linger { l_onoff : linger . is_some () as libc :: c_int , l_linger : linger . unwrap_or_default () . as_secs () as libc :: c_int , } ; setsockopt (self , libc :: SOL_SOCKET , SO_LINGER , linger) } #[cfg (target_os = "cygwin")] pub fn set_linger (& self , linger : Option < Duration >) -> io :: Result < () > { let linger = libc :: linger { l_onoff : linger . is_some () as libc :: c_ushort , l_linger : linger . unwrap_or_default () . as_secs () as libc :: c_ushort , } ; setsockopt (self , libc :: SOL_SOCKET , SO_LINGER , linger) } pub fn linger (& self) -> io :: Result < Option < Duration > > { let val : libc :: linger = getsockopt (self , libc :: SOL_SOCKET , SO_LINGER) ? ; Ok ((val . l_onoff != 0) . then (| | Duration :: from_secs (val . l_linger as u64))) } pub fn set_nodelay (& self , nodelay : bool) -> io :: Result < () > { setsockopt (self , libc :: IPPROTO_TCP , libc :: TCP_NODELAY , nodelay as c_int) } pub fn nodelay (& self) -> io :: Result < bool > { let raw : c_int = getsockopt (self , libc :: IPPROTO_TCP , libc :: TCP_NODELAY) ? ; Ok (raw != 0) } #[cfg (any (target_os = "android" , target_os = "linux" , target_os = "cygwin"))] pub fn set_quickack (& self , quickack : bool) -> io :: Result < () > { setsockopt (self , libc :: IPPROTO_TCP , libc :: TCP_QUICKACK , quickack as c_int) } #[cfg (any (target_os = "android" , target_os = "linux" , target_os = "cygwin"))] pub fn quickack (& self) -> io :: Result < bool > { let raw : c_int = getsockopt (self , libc :: IPPROTO_TCP , libc :: TCP_QUICKACK) ? ; Ok (raw != 0) } #[cfg (target_os = "linux")] pub fn set_deferaccept (& self , accept : u32) -> io :: Result < () > { setsockopt (self , libc :: IPPROTO_TCP , libc :: TCP_DEFER_ACCEPT , accept as c_int) } #[cfg (target_os = "linux")] pub fn deferaccept (& self) -> io :: Result < u32 > { let raw : c_int = getsockopt (self , libc :: IPPROTO_TCP , libc :: TCP_DEFER_ACCEPT) ? ; Ok (raw as u32) } #[cfg (any (target_os = "freebsd" , target_os = "netbsd"))] pub fn set_acceptfilter (& self , name : & CStr) -> io :: Result < () > { if ! name . to_bytes () . is_empty () { const AF_NAME_MAX : usize = 16 ; let mut buf = [0 ; AF_NAME_MAX] ; for (src , dst) in name . to_bytes () . iter () . zip (& mut buf [.. AF_NAME_MAX - 1]) { * dst = * src as libc :: c_char ; } let mut arg : libc :: accept_filter_arg = unsafe { mem :: zeroed () } ; arg . af_name = buf ; setsockopt (self , libc :: SOL_SOCKET , libc :: SO_ACCEPTFILTER , & mut arg) } else { setsockopt (self , libc :: SOL_SOCKET , libc :: SO_ACCEPTFILTER , core :: ptr :: null_mut () as * mut c_void ,) } } #[cfg (any (target_os = "freebsd" , target_os = "netbsd"))] pub fn acceptfilter (& self) -> io :: Result < & CStr > { let arg : libc :: accept_filter_arg = getsockopt (self , libc :: SOL_SOCKET , libc :: SO_ACCEPTFILTER) ? ; let s : & [u8] = unsafe { core :: slice :: from_raw_parts (arg . af_name . as_ptr () as * const u8 , 16) } ; let name = CStr :: from_bytes_with_nul (s) . unwrap () ; Ok (name) } #[cfg (any (target_os = "solaris" , target_os = "illumos"))] pub fn set_exclbind (& self , excl : bool) -> io :: Result < () > { const SO_EXCLBIND : i32 = 0x1015 ; setsockopt (self , libc :: SOL_SOCKET , SO_EXCLBIND , excl) } #[cfg (any (target_os = "solaris" , target_os = "illumos"))] pub fn exclbind (& self) -> io :: Result < bool > { const SO_EXCLBIND : i32 = 0x1015 ; let raw : c_int = getsockopt (self , libc :: SOL_SOCKET , SO_EXCLBIND) ? ; Ok (raw != 0) } #[cfg (any (target_os = "android" , target_os = "linux" , target_os = "cygwin"))] pub fn set_passcred (& self , passcred : bool) -> io :: Result < () > { setsockopt (self , libc :: SOL_SOCKET , libc :: SO_PASSCRED , passcred as libc :: c_int) } #[cfg (any (target_os = "android" , target_os = "linux" , target_os = "cygwin"))] pub fn passcred (& self) -> io :: Result < bool > { let passcred : libc :: c_int = getsockopt (self , libc :: SOL_SOCKET , libc :: SO_PASSCRED) ? ; Ok (passcred != 0) } #[cfg (target_os = "netbsd")] pub fn set_local_creds (& self , local_creds : bool) -> io :: Result < () > { setsockopt (self , 0 as libc :: c_int , libc :: LOCAL_CREDS , local_creds as libc :: c_int) } #[cfg (target_os = "netbsd")] pub fn local_creds (& self) -> io :: Result < bool > { let local_creds : libc :: c_int = getsockopt (self , 0 as libc :: c_int , libc :: LOCAL_CREDS) ? ; Ok (local_creds != 0) } #[cfg (target_os = "freebsd")] pub fn set_local_creds_persistent (& self , local_creds_persistent : bool) -> io :: Result < () > { setsockopt (self , libc :: AF_LOCAL , libc :: LOCAL_CREDS_PERSISTENT , local_creds_persistent as libc :: c_int ,) } #[cfg (target_os = "freebsd")] pub fn local_creds_persistent (& self) -> io :: Result < bool > { let local_creds_persistent : libc :: c_int = getsockopt (self , libc :: AF_LOCAL , libc :: LOCAL_CREDS_PERSISTENT) ? ; Ok (local_creds_persistent != 0) } #[cfg (not (any (target_os = "solaris" , target_os = "illumos" , target_os = "vita")))] pub fn set_nonblocking (& self , nonblocking : bool) -> io :: Result < () > { let mut nonblocking = nonblocking as libc :: c_int ; cvt (unsafe { libc :: ioctl (self . as_raw_fd () , libc :: FIONBIO , & mut nonblocking) }) . map (drop) } #[cfg (target_os = "vita")] pub fn set_nonblocking (& self , nonblocking : bool) -> io :: Result < () > { let option = nonblocking as libc :: c_int ; setsockopt (self , libc :: SOL_SOCKET , libc :: SO_NONBLOCK , option) } #[cfg (any (target_os = "solaris" , target_os = "illumos"))] pub fn set_nonblocking (& self , nonblocking : bool) -> io :: Result < () > { self . 0 . set_nonblocking (nonblocking) } #[cfg (any (target_os = "linux" , target_os = "freebsd" , target_os = "openbsd"))] pub fn set_mark (& self , mark : u32) -> io :: Result < () > { #[cfg (target_os = "linux")] let option = libc :: SO_MARK ; #[cfg (target_os = "freebsd")] let option = libc :: SO_USER_COOKIE ; #[cfg (target_os = "openbsd")] let option = libc :: SO_RTABLE ; setsockopt (self , libc :: SOL_SOCKET , option , mark as libc :: c_int) } pub fn take_error (& self) -> io :: Result < Option < io :: Error > > { let raw : c_int = getsockopt (self , libc :: SOL_SOCKET , libc :: SO_ERROR) ? ; if raw == 0 { Ok (None) } else { Ok (Some (io :: Error :: from_raw_os_error (raw as i32))) } } pub fn as_raw (& self) -> RawFd { self . as_raw_fd () } }}}
mkitem!{mkimpl!{impl AsInner < FileDesc > for Socket { #[inline] fn as_inner (& self) -> & FileDesc { & self . 0 } }}}
mkitem!{mkimpl!{impl IntoInner < FileDesc > for Socket { fn into_inner (self) -> FileDesc { self . 0 } }}}
mkitem!{mkimpl!{impl FromInner < FileDesc > for Socket { fn from_inner (file_desc : FileDesc) -> Self { Self (file_desc) } }}}
mkitem!{mkimpl!{impl AsFd for Socket { fn as_fd (& self) -> BorrowedFd < '_ > { self . 0 . as_fd () } }}}
mkitem!{mkimpl!{impl AsRawFd for Socket { #[inline] fn as_raw_fd (& self) -> RawFd { self . 0 . as_raw_fd () } }}}
mkitem!{mkimpl!{impl IntoRawFd for Socket { fn into_raw_fd (self) -> RawFd { self . 0 . into_raw_fd () } }}}
mkitem!{mkimpl!{impl FromRawFd for Socket { unsafe fn from_raw_fd (raw_fd : RawFd) -> Self { Self (FromRawFd :: from_raw_fd (raw_fd)) } }}}

macro_rules! on_resolver_failure_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function on_resolver_failure in module {}", module_path!());
    };
}

mkfn!{
    on_resolver_failure_introspect!();
    #[cfg (all (target_os = "linux" , target_env = "gnu"))] fn on_resolver_failure () { use crate :: sys ; if let Some (version) = sys :: os :: glibc_version () { if version < (2 , 26) { unsafe { libc :: res_init () } ; } } }
}

macro_rules! on_resolver_failure_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function on_resolver_failure in module {}", module_path!());
    };
}

mkfn!{
    on_resolver_failure_introspect!();
    #[cfg (not (all (target_os = "linux" , target_env = "gnu")))] fn on_resolver_failure () { }
}