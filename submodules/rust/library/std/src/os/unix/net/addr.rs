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
mkuse!{use crate :: bstr :: ByteStr ;}
mkuse!{use crate :: ffi :: OsStr ;}
mkuse!{#[cfg (any (doc , target_os = "android" , target_os = "linux" , target_os = "cygwin"))] use crate :: os :: net :: linux_ext ;}
mkuse!{use crate :: os :: unix :: ffi :: OsStrExt ;}
mkuse!{use crate :: path :: Path ;}
mkuse!{use crate :: sealed :: Sealed ;}
mkuse!{use crate :: sys :: cvt ;}
mkuse!{use crate :: { fmt , io , mem , ptr } ;}
mkmod!{libc, { 
                getname!(libc);
                getsrc!(libc);
                getpath!(libc);
                get_deps!(libc);
                get_crates!(libc);
                mkinclude!(libc);
                mkuse!{pub use core :: ffi :: c_int ;}
mkitem!{pub type socklen_t = u32 ;}
mkitem!{mkstruct!{pub struct sockaddr ;}}
mkitem!{mkstruct!{#[derive (Clone)] pub struct sockaddr_un { pub sun_path : [u8 ; 1] , }}} 
            }}
mkitem!{const SUN_PATH_OFFSET : usize = mem :: offset_of ! (libc :: sockaddr_un , sun_path) ;}

macro_rules! sockaddr_un_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sockaddr_un in module {}", module_path!());
    };
}

mkfn!{
    sockaddr_un_introspect!();
    pub (super) fn sockaddr_un (path : & Path) -> io :: Result < (libc :: sockaddr_un , libc :: socklen_t) > { let mut addr : libc :: sockaddr_un = unsafe { mem :: zeroed () } ; addr . sun_family = libc :: AF_UNIX as libc :: sa_family_t ; let bytes = path . as_os_str () . as_bytes () ; if bytes . contains (& 0) { return Err (io :: const_error ! (io :: ErrorKind :: InvalidInput , "paths must not contain interior null bytes" ,)) ; } if bytes . len () >= addr . sun_path . len () { return Err (io :: const_error ! (io :: ErrorKind :: InvalidInput , "path must be shorter than SUN_LEN" ,)) ; } unsafe { ptr :: copy_nonoverlapping (bytes . as_ptr () , addr . sun_path . as_mut_ptr () . cast () , bytes . len ()) } ; let mut len = SUN_PATH_OFFSET + bytes . len () ; match bytes . get (0) { Some (& 0) | None => { } Some (_) => len += 1 , } Ok ((addr , len as libc :: socklen_t)) }
}
mkitem!{mkenum!{enum AddressKind < 'a > { Unnamed , Pathname (& 'a Path) , Abstract (& 'a ByteStr) , }}}
mkitem!{mkstruct!{#[doc = " An address associated with a Unix socket."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::os::unix::net::UnixListener;"] #[doc = ""] #[doc = " let socket = match UnixListener::bind(\"/tmp/sock\") {"] #[doc = "     Ok(sock) => sock,"] #[doc = "     Err(e) => {"] #[doc = "         println!(\"Couldn't bind: {e:?}\");"] #[doc = "         return"] #[doc = "     }"] #[doc = " };"] #[doc = " let addr = socket.local_addr().expect(\"Couldn't get local address\");"] #[doc = " ```"] #[derive (Clone)] #[stable (feature = "unix_socket" , since = "1.10.0")] pub struct SocketAddr { pub (super) addr : libc :: sockaddr_un , pub (super) len : libc :: socklen_t , }}}
mkitem!{mkimpl!{impl SocketAddr { pub (super) fn new < F > (f : F) -> io :: Result < SocketAddr > where F : FnOnce (* mut libc :: sockaddr , * mut libc :: socklen_t) -> libc :: c_int , { unsafe { let mut addr : libc :: sockaddr_un = mem :: zeroed () ; let mut len = size_of :: < libc :: sockaddr_un > () as libc :: socklen_t ; cvt (f ((& raw mut addr) as * mut _ , & mut len)) ? ; SocketAddr :: from_parts (addr , len) } } pub (super) fn from_parts (addr : libc :: sockaddr_un , mut len : libc :: socklen_t ,) -> io :: Result < SocketAddr > { if cfg ! (target_os = "openbsd") { let sun_path : & [u8] = unsafe { mem :: transmute :: < & [libc :: c_char] , & [u8] > (& addr . sun_path) } ; len = core :: slice :: memchr :: memchr (0 , sun_path) . map_or (len , | new_len | (new_len + SUN_PATH_OFFSET) as libc :: socklen_t) ; } if len == 0 { len = SUN_PATH_OFFSET as libc :: socklen_t ; } else if addr . sun_family != libc :: AF_UNIX as libc :: sa_family_t { return Err (io :: const_error ! (io :: ErrorKind :: InvalidInput , "file descriptor did not correspond to a Unix socket" ,)) ; } Ok (SocketAddr { addr , len }) } #[doc = " Constructs a `SockAddr` with the family `AF_UNIX` and the provided path."] #[doc = ""] #[doc = " # Errors"] #[doc = ""] #[doc = " Returns an error if the path is longer than `SUN_LEN` or if it contains"] #[doc = " NULL bytes."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::os::unix::net::SocketAddr;"] #[doc = " use std::path::Path;"] #[doc = ""] #[doc = " # fn main() -> std::io::Result<()> {"] #[doc = " let address = SocketAddr::from_pathname(\"/path/to/socket\")?;"] #[doc = " assert_eq!(address.as_pathname(), Some(Path::new(\"/path/to/socket\")));"] #[doc = " # Ok(())"] #[doc = " # }"] #[doc = " ```"] #[doc = ""] #[doc = " Creating a `SocketAddr` with a NULL byte results in an error."] #[doc = ""] #[doc = " ```"] #[doc = " use std::os::unix::net::SocketAddr;"] #[doc = ""] #[doc = " assert!(SocketAddr::from_pathname(\"/path/with/\\0/bytes\").is_err());"] #[doc = " ```"] #[stable (feature = "unix_socket_creation" , since = "1.61.0")] pub fn from_pathname < P > (path : P) -> io :: Result < SocketAddr > where P : AsRef < Path > , { sockaddr_un (path . as_ref ()) . map (| (addr , len) | SocketAddr { addr , len }) } #[doc = " Returns `true` if the address is unnamed."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " A named address:"] #[doc = ""] #[doc = " ```no_run"] #[doc = " use std::os::unix::net::UnixListener;"] #[doc = ""] #[doc = " fn main() -> std::io::Result<()> {"] #[doc = "     let socket = UnixListener::bind(\"/tmp/sock\")?;"] #[doc = "     let addr = socket.local_addr().expect(\"Couldn't get local address\");"] #[doc = "     assert_eq!(addr.is_unnamed(), false);"] #[doc = "     Ok(())"] #[doc = " }"] #[doc = " ```"] #[doc = ""] #[doc = " An unnamed address:"] #[doc = ""] #[doc = " ```"] #[doc = " use std::os::unix::net::UnixDatagram;"] #[doc = ""] #[doc = " fn main() -> std::io::Result<()> {"] #[doc = "     let socket = UnixDatagram::unbound()?;"] #[doc = "     let addr = socket.local_addr().expect(\"Couldn't get local address\");"] #[doc = "     assert_eq!(addr.is_unnamed(), true);"] #[doc = "     Ok(())"] #[doc = " }"] #[doc = " ```"] #[must_use] #[stable (feature = "unix_socket" , since = "1.10.0")] pub fn is_unnamed (& self) -> bool { matches ! (self . address () , AddressKind :: Unnamed) } #[doc = " Returns the contents of this address if it is a `pathname` address."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " With a pathname:"] #[doc = ""] #[doc = " ```no_run"] #[doc = " use std::os::unix::net::UnixListener;"] #[doc = " use std::path::Path;"] #[doc = ""] #[doc = " fn main() -> std::io::Result<()> {"] #[doc = "     let socket = UnixListener::bind(\"/tmp/sock\")?;"] #[doc = "     let addr = socket.local_addr().expect(\"Couldn't get local address\");"] #[doc = "     assert_eq!(addr.as_pathname(), Some(Path::new(\"/tmp/sock\")));"] #[doc = "     Ok(())"] #[doc = " }"] #[doc = " ```"] #[doc = ""] #[doc = " Without a pathname:"] #[doc = ""] #[doc = " ```"] #[doc = " use std::os::unix::net::UnixDatagram;"] #[doc = ""] #[doc = " fn main() -> std::io::Result<()> {"] #[doc = "     let socket = UnixDatagram::unbound()?;"] #[doc = "     let addr = socket.local_addr().expect(\"Couldn't get local address\");"] #[doc = "     assert_eq!(addr.as_pathname(), None);"] #[doc = "     Ok(())"] #[doc = " }"] #[doc = " ```"] #[stable (feature = "unix_socket" , since = "1.10.0")] #[must_use] pub fn as_pathname (& self) -> Option < & Path > { if let AddressKind :: Pathname (path) = self . address () { Some (path) } else { None } } fn address (& self) -> AddressKind < '_ > { let len = self . len as usize - SUN_PATH_OFFSET ; let path = unsafe { mem :: transmute :: < & [libc :: c_char] , & [u8] > (& self . addr . sun_path) } ; if len == 0 || (cfg ! (not (any (target_os = "linux" , target_os = "android" , target_os = "cygwin"))) && self . addr . sun_path [0] == 0) { AddressKind :: Unnamed } else if self . addr . sun_path [0] == 0 { AddressKind :: Abstract (ByteStr :: from_bytes (& path [1 .. len])) } else { AddressKind :: Pathname (OsStr :: from_bytes (& path [.. len - 1]) . as_ref ()) } } }}}
mkitem!{mkimpl!{#[stable (feature = "unix_socket_abstract" , since = "1.70.0")] impl Sealed for SocketAddr { }}}
mkitem!{mkimpl!{#[doc (cfg (any (target_os = "android" , target_os = "linux" , target_os = "cygwin")))] #[cfg (any (doc , target_os = "android" , target_os = "linux" , target_os = "cygwin"))] #[stable (feature = "unix_socket_abstract" , since = "1.70.0")] impl linux_ext :: addr :: SocketAddrExt for SocketAddr { fn as_abstract_name (& self) -> Option < & [u8] > { if let AddressKind :: Abstract (name) = self . address () { Some (name . as_bytes ()) } else { None } } fn from_abstract_name < N > (name : N) -> crate :: io :: Result < Self > where N : AsRef < [u8] > , { let name = name . as_ref () ; unsafe { let mut addr : libc :: sockaddr_un = mem :: zeroed () ; addr . sun_family = libc :: AF_UNIX as libc :: sa_family_t ; if name . len () + 1 > addr . sun_path . len () { return Err (io :: const_error ! (io :: ErrorKind :: InvalidInput , "abstract socket name must be shorter than SUN_LEN" ,)) ; } crate :: ptr :: copy_nonoverlapping (name . as_ptr () , addr . sun_path . as_mut_ptr () . add (1) as * mut u8 , name . len () ,) ; let len = (SUN_PATH_OFFSET + 1 + name . len ()) as libc :: socklen_t ; SocketAddr :: from_parts (addr , len) } } }}}
mkitem!{mkimpl!{#[stable (feature = "unix_socket" , since = "1.10.0")] impl fmt :: Debug for SocketAddr { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self . address () { AddressKind :: Unnamed => write ! (fmt , "(unnamed)") , AddressKind :: Abstract (name) => write ! (fmt , "{name:?} (abstract)") , AddressKind :: Pathname (path) => write ! (fmt , "{path:?} (pathname)") , } } }}}