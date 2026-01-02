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
mkuse!{use super :: display_buffer :: DisplayBuffer ;}
mkuse!{use crate :: fmt :: { self , Write } ;}
mkuse!{use crate :: net :: { IpAddr , Ipv4Addr , Ipv6Addr } ;}
mkitem!{mkenum!{#[doc = " An internet socket address, either IPv4 or IPv6."] #[doc = ""] #[doc = " Internet socket addresses consist of an [IP address], a 16-bit port number, as well"] #[doc = " as possibly some version-dependent additional information. See [`SocketAddrV4`]'s and"] #[doc = " [`SocketAddrV6`]'s respective documentation for more details."] #[doc = ""] #[doc = " [IP address]: IpAddr"] #[doc = ""] #[doc = " # Portability"] #[doc = ""] #[doc = " `SocketAddr` is intended to be a portable representation of socket addresses and is likely not"] #[doc = " the same as the internal socket address type used by the target operating system's API. Like all"] #[doc = " `repr(Rust)` structs, however, its exact layout remains undefined and should not be relied upon"] #[doc = " between builds."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::net::{IpAddr, Ipv4Addr, SocketAddr};"] #[doc = ""] #[doc = " let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);"] #[doc = ""] #[doc = " assert_eq!(\"127.0.0.1:8080\".parse(), Ok(socket));"] #[doc = " assert_eq!(socket.port(), 8080);"] #[doc = " assert_eq!(socket.is_ipv4(), true);"] #[doc = " ```"] #[derive (Copy , Clone , PartialEq , Eq , Hash , PartialOrd , Ord)] #[stable (feature = "rust1" , since = "1.0.0")] pub enum SocketAddr { #[doc = " An IPv4 socket address."] #[stable (feature = "rust1" , since = "1.0.0")] V4 (#[stable (feature = "rust1" , since = "1.0.0")] SocketAddrV4) , #[doc = " An IPv6 socket address."] #[stable (feature = "rust1" , since = "1.0.0")] V6 (#[stable (feature = "rust1" , since = "1.0.0")] SocketAddrV6) , }}}
mkitem!{mkstruct!{#[doc = " An IPv4 socket address."] #[doc = ""] #[doc = " IPv4 socket addresses consist of an [`IPv4` address] and a 16-bit port number, as"] #[doc = " stated in [IETF RFC 793]."] #[doc = ""] #[doc = " See [`SocketAddr`] for a type encompassing both IPv4 and IPv6 socket addresses."] #[doc = ""] #[doc = " [IETF RFC 793]: https://tools.ietf.org/html/rfc793"] #[doc = " [`IPv4` address]: Ipv4Addr"] #[doc = ""] #[doc = " # Portability"] #[doc = ""] #[doc = " `SocketAddrV4` is intended to be a portable representation of socket addresses and is likely not"] #[doc = " the same as the internal socket address type used by the target operating system's API. Like all"] #[doc = " `repr(Rust)` structs, however, its exact layout remains undefined and should not be relied upon"] #[doc = " between builds."] #[doc = ""] #[doc = " # Textual representation"] #[doc = ""] #[doc = " `SocketAddrV4` provides a [`FromStr`](crate::str::FromStr) implementation."] #[doc = " It accepts an IPv4 address in its [textual representation], followed by a"] #[doc = " single `:`, followed by the port encoded as a decimal integer.  Other"] #[doc = " formats are not accepted."] #[doc = ""] #[doc = " [textual representation]: Ipv4Addr#textual-representation"] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::net::{Ipv4Addr, SocketAddrV4};"] #[doc = ""] #[doc = " let socket = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080);"] #[doc = ""] #[doc = " assert_eq!(\"127.0.0.1:8080\".parse(), Ok(socket));"] #[doc = " assert_eq!(socket.ip(), &Ipv4Addr::new(127, 0, 0, 1));"] #[doc = " assert_eq!(socket.port(), 8080);"] #[doc = " ```"] #[derive (Copy , Clone , Eq , PartialEq , Ord , PartialOrd , Hash)] #[stable (feature = "rust1" , since = "1.0.0")] pub struct SocketAddrV4 { ip : Ipv4Addr , port : u16 , }}}
mkitem!{mkstruct!{#[doc = " An IPv6 socket address."] #[doc = ""] #[doc = " IPv6 socket addresses consist of an [`IPv6` address], a 16-bit port number, as well"] #[doc = " as fields containing the traffic class, the flow label, and a scope identifier"] #[doc = " (see [IETF RFC 2553, Section 3.3] for more details)."] #[doc = ""] #[doc = " See [`SocketAddr`] for a type encompassing both IPv4 and IPv6 socket addresses."] #[doc = ""] #[doc = " [IETF RFC 2553, Section 3.3]: https://tools.ietf.org/html/rfc2553#section-3.3"] #[doc = " [`IPv6` address]: Ipv6Addr"] #[doc = ""] #[doc = " # Portability"] #[doc = ""] #[doc = " `SocketAddrV6` is intended to be a portable representation of socket addresses and is likely not"] #[doc = " the same as the internal socket address type used by the target operating system's API. Like all"] #[doc = " `repr(Rust)` structs, however, its exact layout remains undefined and should not be relied upon"] #[doc = " between builds."] #[doc = ""] #[doc = " # Textual representation"] #[doc = ""] #[doc = " `SocketAddrV6` provides a [`FromStr`](crate::str::FromStr) implementation,"] #[doc = " based on the bracketed format recommended by [IETF RFC 5952],"] #[doc = " with scope identifiers based on those specified in [IETF RFC 4007]."] #[doc = ""] #[doc = " It accepts addresses consisting of the following elements, in order:"] #[doc = "   - A left square bracket (`[`)"] #[doc = "   - The [textual representation] of an IPv6 address"] #[doc = "   - _Optionally_, a percent sign (`%`) followed by the scope identifier"] #[doc = "     encoded as a decimal integer"] #[doc = "   - A right square bracket (`]`)"] #[doc = "   - A colon (`:`)"] #[doc = "   - The port, encoded as a decimal integer."] #[doc = ""] #[doc = " For example, the string `[2001:db8::413]:443` represents a `SocketAddrV6`"] #[doc = " with the address `2001:db8::413` and port `443`.  The string"] #[doc = " `[2001:db8::413%612]:443` represents the same address and port, with a"] #[doc = " scope identifier of `612`."] #[doc = ""] #[doc = " Other formats are not accepted."] #[doc = ""] #[doc = " [IETF RFC 5952]: https://tools.ietf.org/html/rfc5952#section-6"] #[doc = " [IETF RFC 4007]: https://tools.ietf.org/html/rfc4007#section-11"] #[doc = " [textual representation]: Ipv6Addr#textual-representation"] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::net::{Ipv6Addr, SocketAddrV6};"] #[doc = ""] #[doc = " let socket = SocketAddrV6::new(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1), 8080, 0, 0);"] #[doc = ""] #[doc = " assert_eq!(\"[2001:db8::1]:8080\".parse(), Ok(socket));"] #[doc = " assert_eq!(socket.ip(), &Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1));"] #[doc = " assert_eq!(socket.port(), 8080);"] #[doc = ""] #[doc = " let mut with_scope = socket.clone();"] #[doc = " with_scope.set_scope_id(3);"] #[doc = " assert_eq!(\"[2001:db8::1%3]:8080\".parse(), Ok(with_scope));"] #[doc = " ```"] #[derive (Copy , Clone , Eq , PartialEq , Ord , PartialOrd , Hash)] #[stable (feature = "rust1" , since = "1.0.0")] pub struct SocketAddrV6 { ip : Ipv6Addr , port : u16 , flowinfo : u32 , scope_id : u32 , }}}
mkitem!{mkimpl!{impl SocketAddr { #[doc = " Creates a new socket address from an [IP address] and a port number."] #[doc = ""] #[doc = " [IP address]: IpAddr"] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::net::{IpAddr, Ipv4Addr, SocketAddr};"] #[doc = ""] #[doc = " let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);"] #[doc = " assert_eq!(socket.ip(), IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));"] #[doc = " assert_eq!(socket.port(), 8080);"] #[doc = " ```"] #[stable (feature = "ip_addr" , since = "1.7.0")] #[must_use] #[rustc_const_stable (feature = "const_socketaddr" , since = "1.69.0")] #[inline] pub const fn new (ip : IpAddr , port : u16) -> SocketAddr { match ip { IpAddr :: V4 (a) => SocketAddr :: V4 (SocketAddrV4 :: new (a , port)) , IpAddr :: V6 (a) => SocketAddr :: V6 (SocketAddrV6 :: new (a , port , 0 , 0)) , } } #[doc = " Returns the IP address associated with this socket address."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::net::{IpAddr, Ipv4Addr, SocketAddr};"] #[doc = ""] #[doc = " let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);"] #[doc = " assert_eq!(socket.ip(), IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));"] #[doc = " ```"] #[must_use] #[stable (feature = "ip_addr" , since = "1.7.0")] #[rustc_const_stable (feature = "const_socketaddr" , since = "1.69.0")] #[inline] pub const fn ip (& self) -> IpAddr { match * self { SocketAddr :: V4 (ref a) => IpAddr :: V4 (* a . ip ()) , SocketAddr :: V6 (ref a) => IpAddr :: V6 (* a . ip ()) , } } #[doc = " Changes the IP address associated with this socket address."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::net::{IpAddr, Ipv4Addr, SocketAddr};"] #[doc = ""] #[doc = " let mut socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);"] #[doc = " socket.set_ip(IpAddr::V4(Ipv4Addr::new(10, 10, 0, 1)));"] #[doc = " assert_eq!(socket.ip(), IpAddr::V4(Ipv4Addr::new(10, 10, 0, 1)));"] #[doc = " ```"] #[inline] #[stable (feature = "sockaddr_setters" , since = "1.9.0")] #[rustc_const_stable (feature = "const_sockaddr_setters" , since = "1.87.0")] pub const fn set_ip (& mut self , new_ip : IpAddr) { match (self , new_ip) { (& mut SocketAddr :: V4 (ref mut a) , IpAddr :: V4 (new_ip)) => a . set_ip (new_ip) , (& mut SocketAddr :: V6 (ref mut a) , IpAddr :: V6 (new_ip)) => a . set_ip (new_ip) , (self_ , new_ip) => * self_ = Self :: new (new_ip , self_ . port ()) , } } #[doc = " Returns the port number associated with this socket address."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::net::{IpAddr, Ipv4Addr, SocketAddr};"] #[doc = ""] #[doc = " let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);"] #[doc = " assert_eq!(socket.port(), 8080);"] #[doc = " ```"] #[must_use] #[stable (feature = "rust1" , since = "1.0.0")] #[rustc_const_stable (feature = "const_socketaddr" , since = "1.69.0")] #[inline] pub const fn port (& self) -> u16 { match * self { SocketAddr :: V4 (ref a) => a . port () , SocketAddr :: V6 (ref a) => a . port () , } } #[doc = " Changes the port number associated with this socket address."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::net::{IpAddr, Ipv4Addr, SocketAddr};"] #[doc = ""] #[doc = " let mut socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);"] #[doc = " socket.set_port(1025);"] #[doc = " assert_eq!(socket.port(), 1025);"] #[doc = " ```"] #[inline] #[stable (feature = "sockaddr_setters" , since = "1.9.0")] #[rustc_const_stable (feature = "const_sockaddr_setters" , since = "1.87.0")] pub const fn set_port (& mut self , new_port : u16) { match * self { SocketAddr :: V4 (ref mut a) => a . set_port (new_port) , SocketAddr :: V6 (ref mut a) => a . set_port (new_port) , } } #[doc = " Returns [`true`] if the [IP address] in this `SocketAddr` is an"] #[doc = " [`IPv4` address], and [`false`] otherwise."] #[doc = ""] #[doc = " [IP address]: IpAddr"] #[doc = " [`IPv4` address]: IpAddr::V4"] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::net::{IpAddr, Ipv4Addr, SocketAddr};"] #[doc = ""] #[doc = " let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);"] #[doc = " assert_eq!(socket.is_ipv4(), true);"] #[doc = " assert_eq!(socket.is_ipv6(), false);"] #[doc = " ```"] #[must_use] #[stable (feature = "sockaddr_checker" , since = "1.16.0")] #[rustc_const_stable (feature = "const_socketaddr" , since = "1.69.0")] #[inline] pub const fn is_ipv4 (& self) -> bool { matches ! (* self , SocketAddr :: V4 (_)) } #[doc = " Returns [`true`] if the [IP address] in this `SocketAddr` is an"] #[doc = " [`IPv6` address], and [`false`] otherwise."] #[doc = ""] #[doc = " [IP address]: IpAddr"] #[doc = " [`IPv6` address]: IpAddr::V6"] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::net::{IpAddr, Ipv6Addr, SocketAddr};"] #[doc = ""] #[doc = " let socket = SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 65535, 0, 1)), 8080);"] #[doc = " assert_eq!(socket.is_ipv4(), false);"] #[doc = " assert_eq!(socket.is_ipv6(), true);"] #[doc = " ```"] #[must_use] #[stable (feature = "sockaddr_checker" , since = "1.16.0")] #[rustc_const_stable (feature = "const_socketaddr" , since = "1.69.0")] #[inline] pub const fn is_ipv6 (& self) -> bool { matches ! (* self , SocketAddr :: V6 (_)) } }}}
mkitem!{mkimpl!{impl SocketAddrV4 { #[doc = " Creates a new socket address from an [`IPv4` address] and a port number."] #[doc = ""] #[doc = " [`IPv4` address]: Ipv4Addr"] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::net::{SocketAddrV4, Ipv4Addr};"] #[doc = ""] #[doc = " let socket = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080);"] #[doc = " ```"] #[stable (feature = "rust1" , since = "1.0.0")] #[must_use] #[rustc_const_stable (feature = "const_socketaddr" , since = "1.69.0")] #[inline] pub const fn new (ip : Ipv4Addr , port : u16) -> SocketAddrV4 { SocketAddrV4 { ip , port } } #[doc = " Returns the IP address associated with this socket address."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::net::{SocketAddrV4, Ipv4Addr};"] #[doc = ""] #[doc = " let socket = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080);"] #[doc = " assert_eq!(socket.ip(), &Ipv4Addr::new(127, 0, 0, 1));"] #[doc = " ```"] #[must_use] #[stable (feature = "rust1" , since = "1.0.0")] #[rustc_const_stable (feature = "const_socketaddr" , since = "1.69.0")] #[inline] pub const fn ip (& self) -> & Ipv4Addr { & self . ip } #[doc = " Changes the IP address associated with this socket address."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::net::{SocketAddrV4, Ipv4Addr};"] #[doc = ""] #[doc = " let mut socket = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080);"] #[doc = " socket.set_ip(Ipv4Addr::new(192, 168, 0, 1));"] #[doc = " assert_eq!(socket.ip(), &Ipv4Addr::new(192, 168, 0, 1));"] #[doc = " ```"] #[inline] #[stable (feature = "sockaddr_setters" , since = "1.9.0")] #[rustc_const_stable (feature = "const_sockaddr_setters" , since = "1.87.0")] pub const fn set_ip (& mut self , new_ip : Ipv4Addr) { self . ip = new_ip ; } #[doc = " Returns the port number associated with this socket address."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::net::{SocketAddrV4, Ipv4Addr};"] #[doc = ""] #[doc = " let socket = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080);"] #[doc = " assert_eq!(socket.port(), 8080);"] #[doc = " ```"] #[must_use] #[stable (feature = "rust1" , since = "1.0.0")] #[rustc_const_stable (feature = "const_socketaddr" , since = "1.69.0")] #[inline] pub const fn port (& self) -> u16 { self . port } #[doc = " Changes the port number associated with this socket address."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::net::{SocketAddrV4, Ipv4Addr};"] #[doc = ""] #[doc = " let mut socket = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080);"] #[doc = " socket.set_port(4242);"] #[doc = " assert_eq!(socket.port(), 4242);"] #[doc = " ```"] #[inline] #[stable (feature = "sockaddr_setters" , since = "1.9.0")] #[rustc_const_stable (feature = "const_sockaddr_setters" , since = "1.87.0")] pub const fn set_port (& mut self , new_port : u16) { self . port = new_port ; } }}}
mkitem!{mkimpl!{impl SocketAddrV6 { #[doc = " Creates a new socket address from an [`IPv6` address], a 16-bit port number,"] #[doc = " and the `flowinfo` and `scope_id` fields."] #[doc = ""] #[doc = " For more information on the meaning and layout of the `flowinfo` and `scope_id`"] #[doc = " parameters, see [IETF RFC 2553, Section 3.3]."] #[doc = ""] #[doc = " [IETF RFC 2553, Section 3.3]: https://tools.ietf.org/html/rfc2553#section-3.3"] #[doc = " [`IPv6` address]: Ipv6Addr"] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::net::{SocketAddrV6, Ipv6Addr};"] #[doc = ""] #[doc = " let socket = SocketAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 8080, 0, 0);"] #[doc = " ```"] #[stable (feature = "rust1" , since = "1.0.0")] #[must_use] #[rustc_const_stable (feature = "const_socketaddr" , since = "1.69.0")] #[inline] pub const fn new (ip : Ipv6Addr , port : u16 , flowinfo : u32 , scope_id : u32) -> SocketAddrV6 { SocketAddrV6 { ip , port , flowinfo , scope_id } } #[doc = " Returns the IP address associated with this socket address."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::net::{SocketAddrV6, Ipv6Addr};"] #[doc = ""] #[doc = " let socket = SocketAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 8080, 0, 0);"] #[doc = " assert_eq!(socket.ip(), &Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));"] #[doc = " ```"] #[must_use] #[stable (feature = "rust1" , since = "1.0.0")] #[rustc_const_stable (feature = "const_socketaddr" , since = "1.69.0")] #[inline] pub const fn ip (& self) -> & Ipv6Addr { & self . ip } #[doc = " Changes the IP address associated with this socket address."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::net::{SocketAddrV6, Ipv6Addr};"] #[doc = ""] #[doc = " let mut socket = SocketAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 8080, 0, 0);"] #[doc = " socket.set_ip(Ipv6Addr::new(76, 45, 0, 0, 0, 0, 0, 0));"] #[doc = " assert_eq!(socket.ip(), &Ipv6Addr::new(76, 45, 0, 0, 0, 0, 0, 0));"] #[doc = " ```"] #[inline] #[stable (feature = "sockaddr_setters" , since = "1.9.0")] #[rustc_const_stable (feature = "const_sockaddr_setters" , since = "1.87.0")] pub const fn set_ip (& mut self , new_ip : Ipv6Addr) { self . ip = new_ip ; } #[doc = " Returns the port number associated with this socket address."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::net::{SocketAddrV6, Ipv6Addr};"] #[doc = ""] #[doc = " let socket = SocketAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 8080, 0, 0);"] #[doc = " assert_eq!(socket.port(), 8080);"] #[doc = " ```"] #[must_use] #[stable (feature = "rust1" , since = "1.0.0")] #[rustc_const_stable (feature = "const_socketaddr" , since = "1.69.0")] #[inline] pub const fn port (& self) -> u16 { self . port } #[doc = " Changes the port number associated with this socket address."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::net::{SocketAddrV6, Ipv6Addr};"] #[doc = ""] #[doc = " let mut socket = SocketAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 8080, 0, 0);"] #[doc = " socket.set_port(4242);"] #[doc = " assert_eq!(socket.port(), 4242);"] #[doc = " ```"] #[inline] #[stable (feature = "sockaddr_setters" , since = "1.9.0")] #[rustc_const_stable (feature = "const_sockaddr_setters" , since = "1.87.0")] pub const fn set_port (& mut self , new_port : u16) { self . port = new_port ; } #[doc = " Returns the flow information associated with this address."] #[doc = ""] #[doc = " This information corresponds to the `sin6_flowinfo` field in C's `netinet/in.h`,"] #[doc = " as specified in [IETF RFC 2553, Section 3.3]."] #[doc = " It combines information about the flow label and the traffic class as specified"] #[doc = " in [IETF RFC 2460], respectively [Section 6] and [Section 7]."] #[doc = ""] #[doc = " [IETF RFC 2553, Section 3.3]: https://tools.ietf.org/html/rfc2553#section-3.3"] #[doc = " [IETF RFC 2460]: https://tools.ietf.org/html/rfc2460"] #[doc = " [Section 6]: https://tools.ietf.org/html/rfc2460#section-6"] #[doc = " [Section 7]: https://tools.ietf.org/html/rfc2460#section-7"] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::net::{SocketAddrV6, Ipv6Addr};"] #[doc = ""] #[doc = " let socket = SocketAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 8080, 10, 0);"] #[doc = " assert_eq!(socket.flowinfo(), 10);"] #[doc = " ```"] #[must_use] #[stable (feature = "rust1" , since = "1.0.0")] #[rustc_const_stable (feature = "const_socketaddr" , since = "1.69.0")] #[inline] pub const fn flowinfo (& self) -> u32 { self . flowinfo } #[doc = " Changes the flow information associated with this socket address."] #[doc = ""] #[doc = " See [`SocketAddrV6::flowinfo`]'s documentation for more details."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::net::{SocketAddrV6, Ipv6Addr};"] #[doc = ""] #[doc = " let mut socket = SocketAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 8080, 10, 0);"] #[doc = " socket.set_flowinfo(56);"] #[doc = " assert_eq!(socket.flowinfo(), 56);"] #[doc = " ```"] #[inline] #[stable (feature = "sockaddr_setters" , since = "1.9.0")] #[rustc_const_stable (feature = "const_sockaddr_setters" , since = "1.87.0")] pub const fn set_flowinfo (& mut self , new_flowinfo : u32) { self . flowinfo = new_flowinfo ; } #[doc = " Returns the scope ID associated with this address."] #[doc = ""] #[doc = " This information corresponds to the `sin6_scope_id` field in C's `netinet/in.h`,"] #[doc = " as specified in [IETF RFC 2553, Section 3.3]."] #[doc = ""] #[doc = " [IETF RFC 2553, Section 3.3]: https://tools.ietf.org/html/rfc2553#section-3.3"] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::net::{SocketAddrV6, Ipv6Addr};"] #[doc = ""] #[doc = " let socket = SocketAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 8080, 0, 78);"] #[doc = " assert_eq!(socket.scope_id(), 78);"] #[doc = " ```"] #[must_use] #[stable (feature = "rust1" , since = "1.0.0")] #[rustc_const_stable (feature = "const_socketaddr" , since = "1.69.0")] #[inline] pub const fn scope_id (& self) -> u32 { self . scope_id } #[doc = " Changes the scope ID associated with this socket address."] #[doc = ""] #[doc = " See [`SocketAddrV6::scope_id`]'s documentation for more details."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use std::net::{SocketAddrV6, Ipv6Addr};"] #[doc = ""] #[doc = " let mut socket = SocketAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 8080, 0, 78);"] #[doc = " socket.set_scope_id(42);"] #[doc = " assert_eq!(socket.scope_id(), 42);"] #[doc = " ```"] #[inline] #[stable (feature = "sockaddr_setters" , since = "1.9.0")] #[rustc_const_stable (feature = "const_sockaddr_setters" , since = "1.87.0")] pub const fn set_scope_id (& mut self , new_scope_id : u32) { self . scope_id = new_scope_id ; } }}}
mkitem!{#[stable (feature = "ip_from_ip" , since = "1.16.0")] #[rustc_const_unstable (feature = "const_convert" , issue = "143773")] impl const From < SocketAddrV4 > for SocketAddr { #[doc = " Converts a [`SocketAddrV4`] into a [`SocketAddr::V4`]."] #[inline] fn from (sock4 : SocketAddrV4) -> SocketAddr { SocketAddr :: V4 (sock4) } }}
mkitem!{#[stable (feature = "ip_from_ip" , since = "1.16.0")] #[rustc_const_unstable (feature = "const_convert" , issue = "143773")] impl const From < SocketAddrV6 > for SocketAddr { #[doc = " Converts a [`SocketAddrV6`] into a [`SocketAddr::V6`]."] #[inline] fn from (sock6 : SocketAddrV6) -> SocketAddr { SocketAddr :: V6 (sock6) } }}
mkitem!{#[stable (feature = "addr_from_into_ip" , since = "1.17.0")] #[rustc_const_unstable (feature = "const_convert" , issue = "143773")] impl < I : [const] Into < IpAddr >> const From < (I , u16) > for SocketAddr { #[doc = " Converts a tuple struct (Into<[`IpAddr`]>, `u16`) into a [`SocketAddr`]."] #[doc = ""] #[doc = " This conversion creates a [`SocketAddr::V4`] for an [`IpAddr::V4`]"] #[doc = " and creates a [`SocketAddr::V6`] for an [`IpAddr::V6`]."] #[doc = ""] #[doc = " `u16` is treated as port of the newly created [`SocketAddr`]."] fn from (pieces : (I , u16)) -> SocketAddr { SocketAddr :: new (pieces . 0 . into () , pieces . 1) } }}
mkitem!{mkimpl!{#[stable (feature = "rust1" , since = "1.0.0")] impl fmt :: Display for SocketAddr { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { SocketAddr :: V4 (ref a) => a . fmt (f) , SocketAddr :: V6 (ref a) => a . fmt (f) , } } }}}
mkitem!{mkimpl!{#[stable (feature = "rust1" , since = "1.0.0")] impl fmt :: Debug for SocketAddr { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (self , fmt) } }}}
mkitem!{mkimpl!{#[stable (feature = "rust1" , since = "1.0.0")] impl fmt :: Display for SocketAddrV4 { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if f . precision () . is_none () && f . width () . is_none () { write ! (f , "{}:{}" , self . ip () , self . port ()) } else { const LONGEST_IPV4_SOCKET_ADDR : & str = "255.255.255.255:65535" ; let mut buf = DisplayBuffer :: < { LONGEST_IPV4_SOCKET_ADDR . len () } > :: new () ; write ! (buf , "{}:{}" , self . ip () , self . port ()) . unwrap () ; f . pad (buf . as_str ()) } } }}}
mkitem!{mkimpl!{#[stable (feature = "rust1" , since = "1.0.0")] impl fmt :: Debug for SocketAddrV4 { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (self , fmt) } }}}
mkitem!{mkimpl!{#[stable (feature = "rust1" , since = "1.0.0")] impl fmt :: Display for SocketAddrV6 { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if f . precision () . is_none () && f . width () . is_none () { match self . scope_id () { 0 => write ! (f , "[{}]:{}" , self . ip () , self . port ()) , scope_id => write ! (f , "[{}%{}]:{}" , self . ip () , scope_id , self . port ()) , } } else { const LONGEST_IPV6_SOCKET_ADDR : & str = "[ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff%4294967295]:65535" ; let mut buf = DisplayBuffer :: < { LONGEST_IPV6_SOCKET_ADDR . len () } > :: new () ; match self . scope_id () { 0 => write ! (buf , "[{}]:{}" , self . ip () , self . port ()) , scope_id => write ! (buf , "[{}%{}]:{}" , self . ip () , scope_id , self . port ()) , } . unwrap () ; f . pad (buf . as_str ()) } } }}}
mkitem!{mkimpl!{#[stable (feature = "rust1" , since = "1.0.0")] impl fmt :: Debug for SocketAddrV6 { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (self , fmt) } }}}