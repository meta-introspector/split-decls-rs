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
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                 
            }}
mkuse!{#[stable (feature = "rust1" , since = "1.0.0")] pub use core :: net :: { SocketAddr , SocketAddrV4 , SocketAddrV6 } ;}
mkuse!{use crate :: net :: { IpAddr , Ipv4Addr , Ipv6Addr } ;}
mkuse!{use crate :: sys :: net :: LookupHost ;}
mkuse!{use crate :: { io , iter , option , slice , vec } ;}
mkitem!{mktrait!{#[doc = " A trait for objects which can be converted or resolved to one or more"] #[doc = " [`SocketAddr`] values."] #[doc = ""] #[doc = " This trait is used for generic address resolution when constructing network"] #[doc = " objects. By default it is implemented for the following types:"] #[doc = ""] #[doc = "  * [`SocketAddr`]: [`to_socket_addrs`] is the identity function."] #[doc = ""] #[doc = "  * [`SocketAddrV4`], [`SocketAddrV6`], <code>([IpAddr], [u16])</code>,"] #[doc = "    <code>([Ipv4Addr], [u16])</code>, <code>([Ipv6Addr], [u16])</code>:"] #[doc = "    [`to_socket_addrs`] constructs a [`SocketAddr`] trivially."] #[doc = ""] #[doc = "  * <code>(&[str], [u16])</code>: <code>&[str]</code> should be either a string representation"] #[doc = "    of an [`IpAddr`] address as expected by [`FromStr`] implementation or a host"] #[doc = "    name. [`u16`] is the port number."] #[doc = ""] #[doc = "  * <code>&[str]</code>: the string should be either a string representation of a"] #[doc = "    [`SocketAddr`] as expected by its [`FromStr`] implementation or a string like"] #[doc = "    `<host_name>:<port>` pair where `<port>` is a [`u16`] value."] #[doc = ""] #[doc = " This trait allows constructing network objects like [`TcpStream`] or"] #[doc = " [`UdpSocket`] easily with values of various types for the bind/connection"] #[doc = " address. It is needed because sometimes one type is more appropriate than"] #[doc = " the other: for simple uses a string like `\"localhost:12345\"` is much nicer"] #[doc = " than manual construction of the corresponding [`SocketAddr`], but sometimes"] #[doc = " [`SocketAddr`] value is *the* main source of the address, and converting it to"] #[doc = " some other type (e.g., a string) just for it to be converted back to"] #[doc = " [`SocketAddr`] in constructor methods is pointless."] #[doc = ""] #[doc = " Addresses returned by the operating system that are not IP addresses are"] #[doc = " silently ignored."] #[doc = ""] #[doc = " [`FromStr`]: crate::str::FromStr \"std::str::FromStr\""] #[doc = " [`TcpStream`]: crate::net::TcpStream \"net::TcpStream\""] #[doc = " [`to_socket_addrs`]: ToSocketAddrs::to_socket_addrs"] #[doc = " [`UdpSocket`]: crate::net::UdpSocket \"net::UdpSocket\""] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " Creating a [`SocketAddr`] iterator that yields one item:"] #[doc = ""] #[doc = " ```"] #[doc = " use std::net::{ToSocketAddrs, SocketAddr};"] #[doc = ""] #[doc = " let addr = SocketAddr::from(([127, 0, 0, 1], 443));"] #[doc = " let mut addrs_iter = addr.to_socket_addrs().unwrap();"] #[doc = ""] #[doc = " assert_eq!(Some(addr), addrs_iter.next());"] #[doc = " assert!(addrs_iter.next().is_none());"] #[doc = " ```"] #[doc = ""] #[doc = " Creating a [`SocketAddr`] iterator from a hostname:"] #[doc = ""] #[doc = " ```no_run"] #[doc = " use std::net::{SocketAddr, ToSocketAddrs};"] #[doc = ""] #[doc = " // assuming 'localhost' resolves to 127.0.0.1"] #[doc = " let mut addrs_iter = \"localhost:443\".to_socket_addrs().unwrap();"] #[doc = " assert_eq!(addrs_iter.next(), Some(SocketAddr::from(([127, 0, 0, 1], 443))));"] #[doc = " assert!(addrs_iter.next().is_none());"] #[doc = ""] #[doc = " // assuming 'foo' does not resolve"] #[doc = " assert!(\"foo:443\".to_socket_addrs().is_err());"] #[doc = " ```"] #[doc = ""] #[doc = " Creating a [`SocketAddr`] iterator that yields multiple items:"] #[doc = ""] #[doc = " ```"] #[doc = " use std::net::{SocketAddr, ToSocketAddrs};"] #[doc = ""] #[doc = " let addr1 = SocketAddr::from(([0, 0, 0, 0], 80));"] #[doc = " let addr2 = SocketAddr::from(([127, 0, 0, 1], 443));"] #[doc = " let addrs = vec![addr1, addr2];"] #[doc = ""] #[doc = " let mut addrs_iter = (&addrs[..]).to_socket_addrs().unwrap();"] #[doc = ""] #[doc = " assert_eq!(Some(addr1), addrs_iter.next());"] #[doc = " assert_eq!(Some(addr2), addrs_iter.next());"] #[doc = " assert!(addrs_iter.next().is_none());"] #[doc = " ```"] #[doc = ""] #[doc = " Attempting to create a [`SocketAddr`] iterator from an improperly formatted"] #[doc = " socket address `&str` (missing the port):"] #[doc = ""] #[doc = " ```"] #[doc = " use std::io;"] #[doc = " use std::net::ToSocketAddrs;"] #[doc = ""] #[doc = " let err = \"127.0.0.1\".to_socket_addrs().unwrap_err();"] #[doc = " assert_eq!(err.kind(), io::ErrorKind::InvalidInput);"] #[doc = " ```"] #[doc = ""] #[doc = " [`TcpStream::connect`] is an example of a function that utilizes"] #[doc = " `ToSocketAddrs` as a trait bound on its parameter in order to accept"] #[doc = " different types:"] #[doc = ""] #[doc = " ```no_run"] #[doc = " use std::net::{TcpStream, Ipv4Addr};"] #[doc = ""] #[doc = " let stream = TcpStream::connect((\"127.0.0.1\", 443));"] #[doc = " // or"] #[doc = " let stream = TcpStream::connect(\"127.0.0.1:443\");"] #[doc = " // or"] #[doc = " let stream = TcpStream::connect((Ipv4Addr::new(127, 0, 0, 1), 443));"] #[doc = " ```"] #[doc = ""] #[doc = " [`TcpStream::connect`]: crate::net::TcpStream::connect"] #[stable (feature = "rust1" , since = "1.0.0")] pub trait ToSocketAddrs { #[doc = " Returned iterator over socket addresses which this type may correspond"] #[doc = " to."] #[stable (feature = "rust1" , since = "1.0.0")] type Iter : Iterator < Item = SocketAddr > ; #[doc = " Converts this object to an iterator of resolved [`SocketAddr`]s."] #[doc = ""] #[doc = " The returned iterator might not actually yield any values depending on the"] #[doc = " outcome of any resolution performed."] #[doc = ""] #[doc = " Note that this function may block the current thread while resolution is"] #[doc = " performed."] #[stable (feature = "rust1" , since = "1.0.0")] fn to_socket_addrs (& self) -> io :: Result < Self :: Iter > ; }}}
mkitem!{mkimpl!{#[stable (feature = "rust1" , since = "1.0.0")] impl ToSocketAddrs for SocketAddr { type Iter = option :: IntoIter < SocketAddr > ; fn to_socket_addrs (& self) -> io :: Result < option :: IntoIter < SocketAddr > > { Ok (Some (* self) . into_iter ()) } }}}
mkitem!{mkimpl!{#[stable (feature = "rust1" , since = "1.0.0")] impl ToSocketAddrs for SocketAddrV4 { type Iter = option :: IntoIter < SocketAddr > ; fn to_socket_addrs (& self) -> io :: Result < option :: IntoIter < SocketAddr > > { SocketAddr :: V4 (* self) . to_socket_addrs () } }}}
mkitem!{mkimpl!{#[stable (feature = "rust1" , since = "1.0.0")] impl ToSocketAddrs for SocketAddrV6 { type Iter = option :: IntoIter < SocketAddr > ; fn to_socket_addrs (& self) -> io :: Result < option :: IntoIter < SocketAddr > > { SocketAddr :: V6 (* self) . to_socket_addrs () } }}}
mkitem!{mkimpl!{#[stable (feature = "rust1" , since = "1.0.0")] impl ToSocketAddrs for (IpAddr , u16) { type Iter = option :: IntoIter < SocketAddr > ; fn to_socket_addrs (& self) -> io :: Result < option :: IntoIter < SocketAddr > > { let (ip , port) = * self ; match ip { IpAddr :: V4 (ref a) => (* a , port) . to_socket_addrs () , IpAddr :: V6 (ref a) => (* a , port) . to_socket_addrs () , } } }}}
mkitem!{mkimpl!{#[stable (feature = "rust1" , since = "1.0.0")] impl ToSocketAddrs for (Ipv4Addr , u16) { type Iter = option :: IntoIter < SocketAddr > ; fn to_socket_addrs (& self) -> io :: Result < option :: IntoIter < SocketAddr > > { let (ip , port) = * self ; SocketAddrV4 :: new (ip , port) . to_socket_addrs () } }}}
mkitem!{mkimpl!{#[stable (feature = "rust1" , since = "1.0.0")] impl ToSocketAddrs for (Ipv6Addr , u16) { type Iter = option :: IntoIter < SocketAddr > ; fn to_socket_addrs (& self) -> io :: Result < option :: IntoIter < SocketAddr > > { let (ip , port) = * self ; SocketAddrV6 :: new (ip , port , 0 , 0) . to_socket_addrs () } }}}

macro_rules! resolve_socket_addr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function resolve_socket_addr in module {}", module_path!());
    };
}

mkfn!{
    resolve_socket_addr_introspect!();
    fn resolve_socket_addr (lh : LookupHost) -> io :: Result < vec :: IntoIter < SocketAddr > > { let p = lh . port () ; let v : Vec < _ > = lh . map (| mut a | { a . set_port (p) ; a }) . collect () ; Ok (v . into_iter ()) }
}
mkitem!{mkimpl!{#[stable (feature = "rust1" , since = "1.0.0")] impl ToSocketAddrs for (& str , u16) { type Iter = vec :: IntoIter < SocketAddr > ; fn to_socket_addrs (& self) -> io :: Result < vec :: IntoIter < SocketAddr > > { let (host , port) = * self ; if let Ok (addr) = host . parse :: < Ipv4Addr > () { let addr = SocketAddrV4 :: new (addr , port) ; return Ok (vec ! [SocketAddr :: V4 (addr)] . into_iter ()) ; } if let Ok (addr) = host . parse :: < Ipv6Addr > () { let addr = SocketAddrV6 :: new (addr , port , 0 , 0) ; return Ok (vec ! [SocketAddr :: V6 (addr)] . into_iter ()) ; } resolve_socket_addr ((host , port) . try_into () ?) } }}}
mkitem!{mkimpl!{#[stable (feature = "string_u16_to_socket_addrs" , since = "1.46.0")] impl ToSocketAddrs for (String , u16) { type Iter = vec :: IntoIter < SocketAddr > ; fn to_socket_addrs (& self) -> io :: Result < vec :: IntoIter < SocketAddr > > { (& * self . 0 , self . 1) . to_socket_addrs () } }}}
mkitem!{mkimpl!{#[stable (feature = "rust1" , since = "1.0.0")] impl ToSocketAddrs for str { type Iter = vec :: IntoIter < SocketAddr > ; fn to_socket_addrs (& self) -> io :: Result < vec :: IntoIter < SocketAddr > > { if let Ok (addr) = self . parse () { return Ok (vec ! [addr] . into_iter ()) ; } resolve_socket_addr (self . try_into () ?) } }}}
mkitem!{mkimpl!{#[stable (feature = "slice_to_socket_addrs" , since = "1.8.0")] impl < 'a > ToSocketAddrs for & 'a [SocketAddr] { type Iter = iter :: Cloned < slice :: Iter < 'a , SocketAddr > > ; fn to_socket_addrs (& self) -> io :: Result < Self :: Iter > { Ok (self . iter () . cloned ()) } }}}
mkitem!{mkimpl!{#[stable (feature = "rust1" , since = "1.0.0")] impl < T : ToSocketAddrs + ? Sized > ToSocketAddrs for & T { type Iter = T :: Iter ; fn to_socket_addrs (& self) -> io :: Result < T :: Iter > { (* * self) . to_socket_addrs () } }}}
mkitem!{mkimpl!{#[stable (feature = "string_to_socket_addrs" , since = "1.16.0")] impl ToSocketAddrs for String { type Iter = vec :: IntoIter < SocketAddr > ; fn to_socket_addrs (& self) -> io :: Result < vec :: IntoIter < SocketAddr > > { (& * * self) . to_socket_addrs () } }}}