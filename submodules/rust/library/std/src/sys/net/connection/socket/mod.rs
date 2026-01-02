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
mkuse!{use crate :: ffi :: { c_int , c_void } ;}
mkuse!{use crate :: io :: { self , BorrowedCursor , ErrorKind , IoSlice , IoSliceMut } ;}
mkuse!{use crate :: net :: { Ipv4Addr , Ipv6Addr , Shutdown , SocketAddr , SocketAddrV4 , SocketAddrV6 , ToSocketAddrs , } ;}
mkuse!{use crate :: sys :: common :: small_c_string :: run_with_cstr ;}
mkuse!{use crate :: sys :: net :: connection :: each_addr ;}
mkuse!{use crate :: sys_common :: { AsInner , FromInner } ;}
mkuse!{use crate :: time :: Duration ;}
mkuse!{use crate :: { cmp , fmt , mem , ptr } ;}
mkitem!{cfg_select ! { target_os = "hermit" => { mod hermit ; pub use hermit ::*; } target_os = "solid_asp3" => { mod solid ; pub use solid ::*; } target_family = "unix" => { mod unix ; pub use unix ::*; } all (target_os = "wasi" , target_env = "p2") => { mod wasip2 ; pub use wasip2 ::*; } target_os = "windows" => { mod windows ; pub use windows ::*; } _ => { } }}
mkuse!{use netc as c ;}
mkitem!{cfg_select ! { any (target_os = "dragonfly" , target_os = "freebsd" , target_os = "openbsd" , target_os = "netbsd" , target_os = "illumos" , target_os = "solaris" , target_os = "haiku" , target_os = "l4re" , target_os = "nto" , target_os = "nuttx" , target_vendor = "apple" ,) => { use c :: IPV6_JOIN_GROUP as IPV6_ADD_MEMBERSHIP ; use c :: IPV6_LEAVE_GROUP as IPV6_DROP_MEMBERSHIP ; } _ => { use c :: IPV6_ADD_MEMBERSHIP ; use c :: IPV6_DROP_MEMBERSHIP ; } }}
mkitem!{cfg_select ! { any (target_os = "linux" , target_os = "android" , target_os = "hurd" , target_os = "dragonfly" , target_os = "freebsd" , target_os = "openbsd" , target_os = "netbsd" , target_os = "solaris" , target_os = "illumos" , target_os = "haiku" , target_os = "nto" , target_os = "cygwin" ,) => { use libc :: MSG_NOSIGNAL ; } _ => { const MSG_NOSIGNAL : c_int = 0x0 ; } }}
mkitem!{cfg_select ! { any (target_os = "dragonfly" , target_os = "freebsd" , target_os = "openbsd" , target_os = "netbsd" , target_os = "solaris" , target_os = "illumos" , target_os = "nto" ,) => { use crate :: ffi :: c_uchar ; type IpV4MultiCastType = c_uchar ; } _ => { type IpV4MultiCastType = c_int ; } }}

macro_rules! ip_v4_addr_to_c_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function ip_v4_addr_to_c in module {}", module_path!());
    };
}

mkfn!{
    ip_v4_addr_to_c_introspect!();
    fn ip_v4_addr_to_c (addr : & Ipv4Addr) -> c :: in_addr { c :: in_addr { s_addr : u32 :: from_ne_bytes (addr . octets ()) } }
}

macro_rules! ip_v6_addr_to_c_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function ip_v6_addr_to_c in module {}", module_path!());
    };
}

mkfn!{
    ip_v6_addr_to_c_introspect!();
    fn ip_v6_addr_to_c (addr : & Ipv6Addr) -> c :: in6_addr { c :: in6_addr { s6_addr : addr . octets () } }
}

macro_rules! ip_v4_addr_from_c_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function ip_v4_addr_from_c in module {}", module_path!());
    };
}

mkfn!{
    ip_v4_addr_from_c_introspect!();
    fn ip_v4_addr_from_c (addr : c :: in_addr) -> Ipv4Addr { Ipv4Addr :: from (addr . s_addr . to_ne_bytes ()) }
}

macro_rules! ip_v6_addr_from_c_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function ip_v6_addr_from_c in module {}", module_path!());
    };
}

mkfn!{
    ip_v6_addr_from_c_introspect!();
    fn ip_v6_addr_from_c (addr : c :: in6_addr) -> Ipv6Addr { Ipv6Addr :: from (addr . s6_addr) }
}

macro_rules! socket_addr_v4_to_c_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function socket_addr_v4_to_c in module {}", module_path!());
    };
}

mkfn!{
    socket_addr_v4_to_c_introspect!();
    fn socket_addr_v4_to_c (addr : & SocketAddrV4) -> c :: sockaddr_in { c :: sockaddr_in { sin_family : c :: AF_INET as c :: sa_family_t , sin_port : addr . port () . to_be () , sin_addr : ip_v4_addr_to_c (addr . ip ()) , .. unsafe { mem :: zeroed () } } }
}

macro_rules! socket_addr_v6_to_c_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function socket_addr_v6_to_c in module {}", module_path!());
    };
}

mkfn!{
    socket_addr_v6_to_c_introspect!();
    fn socket_addr_v6_to_c (addr : & SocketAddrV6) -> c :: sockaddr_in6 { c :: sockaddr_in6 { sin6_family : c :: AF_INET6 as c :: sa_family_t , sin6_port : addr . port () . to_be () , sin6_addr : ip_v6_addr_to_c (addr . ip ()) , sin6_flowinfo : addr . flowinfo () , sin6_scope_id : addr . scope_id () , .. unsafe { mem :: zeroed () } } }
}

macro_rules! socket_addr_v4_from_c_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function socket_addr_v4_from_c in module {}", module_path!());
    };
}

mkfn!{
    socket_addr_v4_from_c_introspect!();
    fn socket_addr_v4_from_c (addr : c :: sockaddr_in) -> SocketAddrV4 { SocketAddrV4 :: new (ip_v4_addr_from_c (addr . sin_addr) , u16 :: from_be (addr . sin_port)) }
}

macro_rules! socket_addr_v6_from_c_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function socket_addr_v6_from_c in module {}", module_path!());
    };
}

mkfn!{
    socket_addr_v6_from_c_introspect!();
    fn socket_addr_v6_from_c (addr : c :: sockaddr_in6) -> SocketAddrV6 { SocketAddrV6 :: new (ip_v6_addr_from_c (addr . sin6_addr) , u16 :: from_be (addr . sin6_port) , addr . sin6_flowinfo , addr . sin6_scope_id ,) }
}
mkitem!{#[doc = " A type with the same memory layout as `c::sockaddr`. Used in converting Rust level"] #[doc = " SocketAddr* types into their system representation. The benefit of this specific"] #[doc = " type over using `c::sockaddr_storage` is that this type is exactly as large as it"] #[doc = " needs to be and not a lot larger. And it can be initialized more cleanly from Rust."] #[repr (C)] union SocketAddrCRepr { v4 : c :: sockaddr_in , v6 : c :: sockaddr_in6 , }}
mkitem!{mkimpl!{impl SocketAddrCRepr { fn as_ptr (& self) -> * const c :: sockaddr { self as * const _ as * const c :: sockaddr } }}}

macro_rules! socket_addr_to_c_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function socket_addr_to_c in module {}", module_path!());
    };
}

mkfn!{
    socket_addr_to_c_introspect!();
    fn socket_addr_to_c (addr : & SocketAddr) -> (SocketAddrCRepr , c :: socklen_t) { match addr { SocketAddr :: V4 (a) => { let sockaddr = SocketAddrCRepr { v4 : socket_addr_v4_to_c (a) } ; (sockaddr , size_of :: < c :: sockaddr_in > () as c :: socklen_t) } SocketAddr :: V6 (a) => { let sockaddr = SocketAddrCRepr { v6 : socket_addr_v6_to_c (a) } ; (sockaddr , size_of :: < c :: sockaddr_in6 > () as c :: socklen_t) } } }
}

macro_rules! socket_addr_from_c_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function socket_addr_from_c in module {}", module_path!());
    };
}

mkfn!{
    socket_addr_from_c_introspect!();
    unsafe fn socket_addr_from_c (storage : * const c :: sockaddr_storage , len : usize ,) -> io :: Result < SocketAddr > { match (* storage) . ss_family as c_int { c :: AF_INET => { assert ! (len >= size_of ::< c :: sockaddr_in > ()) ; Ok (SocketAddr :: V4 (socket_addr_v4_from_c (unsafe { * (storage as * const _ as * const c :: sockaddr_in) }))) } c :: AF_INET6 => { assert ! (len >= size_of ::< c :: sockaddr_in6 > ()) ; Ok (SocketAddr :: V6 (socket_addr_v6_from_c (unsafe { * (storage as * const _ as * const c :: sockaddr_in6) }))) } _ => Err (io :: const_error ! (ErrorKind :: InvalidInput , "invalid argument")) , } }
}

macro_rules! setsockopt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function setsockopt in module {}", module_path!());
    };
}

mkfn!{
    setsockopt_introspect!();
    pub fn setsockopt < T > (sock : & Socket , level : c_int , option_name : c_int , option_value : T ,) -> io :: Result < () > { unsafe { cvt (c :: setsockopt (sock . as_raw () , level , option_name , (& raw const option_value) as * const _ , size_of :: < T > () as c :: socklen_t ,)) ? ; Ok (()) } }
}

macro_rules! getsockopt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function getsockopt in module {}", module_path!());
    };
}

mkfn!{
    getsockopt_introspect!();
    pub fn getsockopt < T : Copy > (sock : & Socket , level : c_int , option_name : c_int) -> io :: Result < T > { unsafe { let mut option_value : T = mem :: zeroed () ; let mut option_len = size_of :: < T > () as c :: socklen_t ; cvt (c :: getsockopt (sock . as_raw () , level , option_name , (& raw mut option_value) as * mut _ , & mut option_len ,)) ? ; Ok (option_value) } }
}

macro_rules! sockname_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sockname in module {}", module_path!());
    };
}

mkfn!{
    sockname_introspect!();
    fn sockname < F > (f : F) -> io :: Result < SocketAddr > where F : FnOnce (* mut c :: sockaddr , * mut c :: socklen_t) -> c_int , { unsafe { let mut storage : c :: sockaddr_storage = mem :: zeroed () ; let mut len = size_of_val (& storage) as c :: socklen_t ; cvt (f ((& raw mut storage) as * mut _ , & mut len)) ? ; socket_addr_from_c (& storage , len as usize) } }
}

macro_rules! to_ipv6mr_interface_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function to_ipv6mr_interface in module {}", module_path!());
    };
}

mkfn!{
    to_ipv6mr_interface_introspect!();
    #[cfg (target_os = "android")] fn to_ipv6mr_interface (value : u32) -> c_int { value as c_int }
}

macro_rules! to_ipv6mr_interface_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function to_ipv6mr_interface in module {}", module_path!());
    };
}

mkfn!{
    to_ipv6mr_interface_introspect!();
    #[cfg (not (target_os = "android"))] fn to_ipv6mr_interface (value : u32) -> crate :: ffi :: c_uint { value as crate :: ffi :: c_uint }
}
mkitem!{mkstruct!{pub struct LookupHost { original : * mut c :: addrinfo , cur : * mut c :: addrinfo , port : u16 , }}}
mkitem!{mkimpl!{impl LookupHost { pub fn port (& self) -> u16 { self . port } }}}
mkitem!{mkimpl!{impl Iterator for LookupHost { type Item = SocketAddr ; fn next (& mut self) -> Option < SocketAddr > { loop { unsafe { let cur = self . cur . as_ref () ? ; self . cur = cur . ai_next ; match socket_addr_from_c (cur . ai_addr . cast () , cur . ai_addrlen as usize) { Ok (addr) => return Some (addr) , Err (_) => continue , } } } } }}}
mkitem!{mkimpl!{unsafe impl Sync for LookupHost { }}}
mkitem!{mkimpl!{unsafe impl Send for LookupHost { }}}
mkitem!{mkimpl!{impl Drop for LookupHost { fn drop (& mut self) { unsafe { c :: freeaddrinfo (self . original) } } }}}
mkitem!{mkimpl!{impl TryFrom < & str > for LookupHost { type Error = io :: Error ; fn try_from (s : & str) -> io :: Result < LookupHost > { macro_rules ! try_opt { ($ e : expr , $ msg : expr) => { match $ e { Some (r) => r , None => return Err (io :: const_error ! (io :: ErrorKind :: InvalidInput , $ msg)) , } } ; } let (host , port_str) = try_opt ! (s . rsplit_once (':') , "invalid socket address") ; let port : u16 = try_opt ! (port_str . parse () . ok () , "invalid port value") ; (host , port) . try_into () } }}}
mkitem!{mkimpl!{impl < 'a > TryFrom < (& 'a str , u16) > for LookupHost { type Error = io :: Error ; fn try_from ((host , port) : (& 'a str , u16)) -> io :: Result < LookupHost > { init () ; run_with_cstr (host . as_bytes () , & | c_host | { let mut hints : c :: addrinfo = unsafe { mem :: zeroed () } ; hints . ai_socktype = c :: SOCK_STREAM ; let mut res = ptr :: null_mut () ; unsafe { cvt_gai (c :: getaddrinfo (c_host . as_ptr () , ptr :: null () , & hints , & mut res)) . map (| _ | LookupHost { original : res , cur : res , port }) } }) } }}}
mkitem!{mkstruct!{pub struct TcpStream { inner : Socket , }}}
mkitem!{mkimpl!{impl TcpStream { pub fn connect < A : ToSocketAddrs > (addr : A) -> io :: Result < TcpStream > { init () ; return each_addr (addr , inner) ; fn inner (addr : & SocketAddr) -> io :: Result < TcpStream > { let sock = Socket :: new (addr , c :: SOCK_STREAM) ? ; sock . connect (addr) ? ; Ok (TcpStream { inner : sock }) } } pub fn connect_timeout (addr : & SocketAddr , timeout : Duration) -> io :: Result < TcpStream > { init () ; let sock = Socket :: new (addr , c :: SOCK_STREAM) ? ; sock . connect_timeout (addr , timeout) ? ; Ok (TcpStream { inner : sock }) } #[inline] pub fn socket (& self) -> & Socket { & self . inner } pub fn into_socket (self) -> Socket { self . inner } pub fn set_read_timeout (& self , dur : Option < Duration >) -> io :: Result < () > { self . inner . set_timeout (dur , c :: SO_RCVTIMEO) } pub fn set_write_timeout (& self , dur : Option < Duration >) -> io :: Result < () > { self . inner . set_timeout (dur , c :: SO_SNDTIMEO) } pub fn read_timeout (& self) -> io :: Result < Option < Duration > > { self . inner . timeout (c :: SO_RCVTIMEO) } pub fn write_timeout (& self) -> io :: Result < Option < Duration > > { self . inner . timeout (c :: SO_SNDTIMEO) } pub fn peek (& self , buf : & mut [u8]) -> io :: Result < usize > { self . inner . peek (buf) } pub fn read (& self , buf : & mut [u8]) -> io :: Result < usize > { self . inner . read (buf) } pub fn read_buf (& self , buf : BorrowedCursor < '_ >) -> io :: Result < () > { self . inner . read_buf (buf) } pub fn read_vectored (& self , bufs : & mut [IoSliceMut < '_ >]) -> io :: Result < usize > { self . inner . read_vectored (bufs) } #[inline] pub fn is_read_vectored (& self) -> bool { self . inner . is_read_vectored () } pub fn write (& self , buf : & [u8]) -> io :: Result < usize > { let len = cmp :: min (buf . len () , < wrlen_t > :: MAX as usize) as wrlen_t ; let ret = cvt (unsafe { c :: send (self . inner . as_raw () , buf . as_ptr () as * const c_void , len , MSG_NOSIGNAL) }) ? ; Ok (ret as usize) } pub fn write_vectored (& self , bufs : & [IoSlice < '_ >]) -> io :: Result < usize > { self . inner . write_vectored (bufs) } #[inline] pub fn is_write_vectored (& self) -> bool { self . inner . is_write_vectored () } pub fn peer_addr (& self) -> io :: Result < SocketAddr > { sockname (| buf , len | unsafe { c :: getpeername (self . inner . as_raw () , buf , len) }) } pub fn socket_addr (& self) -> io :: Result < SocketAddr > { sockname (| buf , len | unsafe { c :: getsockname (self . inner . as_raw () , buf , len) }) } pub fn shutdown (& self , how : Shutdown) -> io :: Result < () > { self . inner . shutdown (how) } pub fn duplicate (& self) -> io :: Result < TcpStream > { self . inner . duplicate () . map (| s | TcpStream { inner : s }) } pub fn set_linger (& self , linger : Option < Duration >) -> io :: Result < () > { self . inner . set_linger (linger) } pub fn linger (& self) -> io :: Result < Option < Duration > > { self . inner . linger () } pub fn set_nodelay (& self , nodelay : bool) -> io :: Result < () > { self . inner . set_nodelay (nodelay) } pub fn nodelay (& self) -> io :: Result < bool > { self . inner . nodelay () } pub fn set_ttl (& self , ttl : u32) -> io :: Result < () > { setsockopt (& self . inner , c :: IPPROTO_IP , c :: IP_TTL , ttl as c_int) } pub fn ttl (& self) -> io :: Result < u32 > { let raw : c_int = getsockopt (& self . inner , c :: IPPROTO_IP , c :: IP_TTL) ? ; Ok (raw as u32) } pub fn take_error (& self) -> io :: Result < Option < io :: Error > > { self . inner . take_error () } pub fn set_nonblocking (& self , nonblocking : bool) -> io :: Result < () > { self . inner . set_nonblocking (nonblocking) } }}}
mkitem!{mkimpl!{impl AsInner < Socket > for TcpStream { #[inline] fn as_inner (& self) -> & Socket { & self . inner } }}}
mkitem!{mkimpl!{impl FromInner < Socket > for TcpStream { fn from_inner (socket : Socket) -> TcpStream { TcpStream { inner : socket } } }}}
mkitem!{mkimpl!{impl fmt :: Debug for TcpStream { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut res = f . debug_struct ("TcpStream") ; if let Ok (addr) = self . socket_addr () { res . field ("addr" , & addr) ; } if let Ok (peer) = self . peer_addr () { res . field ("peer" , & peer) ; } let name = if cfg ! (windows) { "socket" } else { "fd" } ; res . field (name , & self . inner . as_raw ()) . finish () } }}}
mkitem!{mkstruct!{pub struct TcpListener { inner : Socket , }}}
mkitem!{mkimpl!{impl TcpListener { pub fn bind < A : ToSocketAddrs > (addr : A) -> io :: Result < TcpListener > { init () ; return each_addr (addr , inner) ; fn inner (addr : & SocketAddr) -> io :: Result < TcpListener > { let sock = Socket :: new (addr , c :: SOCK_STREAM) ? ; #[cfg (not (windows))] setsockopt (& sock , c :: SOL_SOCKET , c :: SO_REUSEADDR , 1 as c_int) ? ; let (addr , len) = socket_addr_to_c (addr) ; cvt (unsafe { c :: bind (sock . as_raw () , addr . as_ptr () , len as _) }) ? ; let backlog = if cfg ! (target_os = "horizon") { 20 } else if cfg ! (target_os = "haiku") { 32 } else { 128 } ; cvt (unsafe { c :: listen (sock . as_raw () , backlog) }) ? ; Ok (TcpListener { inner : sock }) } } #[inline] pub fn socket (& self) -> & Socket { & self . inner } pub fn into_socket (self) -> Socket { self . inner } pub fn socket_addr (& self) -> io :: Result < SocketAddr > { sockname (| buf , len | unsafe { c :: getsockname (self . inner . as_raw () , buf , len) }) } pub fn accept (& self) -> io :: Result < (TcpStream , SocketAddr) > { let mut storage : mem :: MaybeUninit < c :: sockaddr_storage > = mem :: MaybeUninit :: uninit () ; let mut len = size_of_val (& storage) as c :: socklen_t ; let sock = self . inner . accept (storage . as_mut_ptr () as * mut _ , & mut len) ? ; let addr = unsafe { socket_addr_from_c (storage . as_ptr () , len as usize) ? } ; Ok ((TcpStream { inner : sock } , addr)) } pub fn duplicate (& self) -> io :: Result < TcpListener > { self . inner . duplicate () . map (| s | TcpListener { inner : s }) } pub fn set_ttl (& self , ttl : u32) -> io :: Result < () > { setsockopt (& self . inner , c :: IPPROTO_IP , c :: IP_TTL , ttl as c_int) } pub fn ttl (& self) -> io :: Result < u32 > { let raw : c_int = getsockopt (& self . inner , c :: IPPROTO_IP , c :: IP_TTL) ? ; Ok (raw as u32) } pub fn set_only_v6 (& self , only_v6 : bool) -> io :: Result < () > { setsockopt (& self . inner , c :: IPPROTO_IPV6 , c :: IPV6_V6ONLY , only_v6 as c_int) } pub fn only_v6 (& self) -> io :: Result < bool > { let raw : c_int = getsockopt (& self . inner , c :: IPPROTO_IPV6 , c :: IPV6_V6ONLY) ? ; Ok (raw != 0) } pub fn take_error (& self) -> io :: Result < Option < io :: Error > > { self . inner . take_error () } pub fn set_nonblocking (& self , nonblocking : bool) -> io :: Result < () > { self . inner . set_nonblocking (nonblocking) } }}}
mkitem!{mkimpl!{impl FromInner < Socket > for TcpListener { fn from_inner (socket : Socket) -> TcpListener { TcpListener { inner : socket } } }}}
mkitem!{mkimpl!{impl fmt :: Debug for TcpListener { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut res = f . debug_struct ("TcpListener") ; if let Ok (addr) = self . socket_addr () { res . field ("addr" , & addr) ; } let name = if cfg ! (windows) { "socket" } else { "fd" } ; res . field (name , & self . inner . as_raw ()) . finish () } }}}
mkitem!{mkstruct!{pub struct UdpSocket { inner : Socket , }}}
mkitem!{mkimpl!{impl UdpSocket { pub fn bind < A : ToSocketAddrs > (addr : A) -> io :: Result < UdpSocket > { init () ; return each_addr (addr , inner) ; fn inner (addr : & SocketAddr) -> io :: Result < UdpSocket > { let sock = Socket :: new (addr , c :: SOCK_DGRAM) ? ; let (addr , len) = socket_addr_to_c (addr) ; cvt (unsafe { c :: bind (sock . as_raw () , addr . as_ptr () , len as _) }) ? ; Ok (UdpSocket { inner : sock }) } } #[inline] pub fn socket (& self) -> & Socket { & self . inner } pub fn into_socket (self) -> Socket { self . inner } pub fn peer_addr (& self) -> io :: Result < SocketAddr > { sockname (| buf , len | unsafe { c :: getpeername (self . inner . as_raw () , buf , len) }) } pub fn socket_addr (& self) -> io :: Result < SocketAddr > { sockname (| buf , len | unsafe { c :: getsockname (self . inner . as_raw () , buf , len) }) } pub fn recv_from (& self , buf : & mut [u8]) -> io :: Result < (usize , SocketAddr) > { self . inner . recv_from (buf) } pub fn peek_from (& self , buf : & mut [u8]) -> io :: Result < (usize , SocketAddr) > { self . inner . peek_from (buf) } pub fn send_to (& self , buf : & [u8] , dst : & SocketAddr) -> io :: Result < usize > { let len = cmp :: min (buf . len () , < wrlen_t > :: MAX as usize) as wrlen_t ; let (dst , dstlen) = socket_addr_to_c (dst) ; let ret = cvt (unsafe { c :: sendto (self . inner . as_raw () , buf . as_ptr () as * const c_void , len , MSG_NOSIGNAL , dst . as_ptr () , dstlen ,) }) ? ; Ok (ret as usize) } pub fn duplicate (& self) -> io :: Result < UdpSocket > { self . inner . duplicate () . map (| s | UdpSocket { inner : s }) } pub fn set_read_timeout (& self , dur : Option < Duration >) -> io :: Result < () > { self . inner . set_timeout (dur , c :: SO_RCVTIMEO) } pub fn set_write_timeout (& self , dur : Option < Duration >) -> io :: Result < () > { self . inner . set_timeout (dur , c :: SO_SNDTIMEO) } pub fn read_timeout (& self) -> io :: Result < Option < Duration > > { self . inner . timeout (c :: SO_RCVTIMEO) } pub fn write_timeout (& self) -> io :: Result < Option < Duration > > { self . inner . timeout (c :: SO_SNDTIMEO) } pub fn set_broadcast (& self , broadcast : bool) -> io :: Result < () > { setsockopt (& self . inner , c :: SOL_SOCKET , c :: SO_BROADCAST , broadcast as c_int) } pub fn broadcast (& self) -> io :: Result < bool > { let raw : c_int = getsockopt (& self . inner , c :: SOL_SOCKET , c :: SO_BROADCAST) ? ; Ok (raw != 0) } pub fn set_multicast_loop_v4 (& self , multicast_loop_v4 : bool) -> io :: Result < () > { setsockopt (& self . inner , c :: IPPROTO_IP , c :: IP_MULTICAST_LOOP , multicast_loop_v4 as IpV4MultiCastType ,) } pub fn multicast_loop_v4 (& self) -> io :: Result < bool > { let raw : IpV4MultiCastType = getsockopt (& self . inner , c :: IPPROTO_IP , c :: IP_MULTICAST_LOOP) ? ; Ok (raw != 0) } pub fn set_multicast_ttl_v4 (& self , multicast_ttl_v4 : u32) -> io :: Result < () > { setsockopt (& self . inner , c :: IPPROTO_IP , c :: IP_MULTICAST_TTL , multicast_ttl_v4 as IpV4MultiCastType ,) } pub fn multicast_ttl_v4 (& self) -> io :: Result < u32 > { let raw : IpV4MultiCastType = getsockopt (& self . inner , c :: IPPROTO_IP , c :: IP_MULTICAST_TTL) ? ; Ok (raw as u32) } pub fn set_multicast_loop_v6 (& self , multicast_loop_v6 : bool) -> io :: Result < () > { setsockopt (& self . inner , c :: IPPROTO_IPV6 , c :: IPV6_MULTICAST_LOOP , multicast_loop_v6 as c_int) } pub fn multicast_loop_v6 (& self) -> io :: Result < bool > { let raw : c_int = getsockopt (& self . inner , c :: IPPROTO_IPV6 , c :: IPV6_MULTICAST_LOOP) ? ; Ok (raw != 0) } pub fn join_multicast_v4 (& self , multiaddr : & Ipv4Addr , interface : & Ipv4Addr) -> io :: Result < () > { let mreq = c :: ip_mreq { imr_multiaddr : ip_v4_addr_to_c (multiaddr) , imr_interface : ip_v4_addr_to_c (interface) , } ; setsockopt (& self . inner , c :: IPPROTO_IP , c :: IP_ADD_MEMBERSHIP , mreq) } pub fn join_multicast_v6 (& self , multiaddr : & Ipv6Addr , interface : u32) -> io :: Result < () > { let mreq = c :: ipv6_mreq { ipv6mr_multiaddr : ip_v6_addr_to_c (multiaddr) , ipv6mr_interface : to_ipv6mr_interface (interface) , } ; setsockopt (& self . inner , c :: IPPROTO_IPV6 , IPV6_ADD_MEMBERSHIP , mreq) } pub fn leave_multicast_v4 (& self , multiaddr : & Ipv4Addr , interface : & Ipv4Addr) -> io :: Result < () > { let mreq = c :: ip_mreq { imr_multiaddr : ip_v4_addr_to_c (multiaddr) , imr_interface : ip_v4_addr_to_c (interface) , } ; setsockopt (& self . inner , c :: IPPROTO_IP , c :: IP_DROP_MEMBERSHIP , mreq) } pub fn leave_multicast_v6 (& self , multiaddr : & Ipv6Addr , interface : u32) -> io :: Result < () > { let mreq = c :: ipv6_mreq { ipv6mr_multiaddr : ip_v6_addr_to_c (multiaddr) , ipv6mr_interface : to_ipv6mr_interface (interface) , } ; setsockopt (& self . inner , c :: IPPROTO_IPV6 , IPV6_DROP_MEMBERSHIP , mreq) } pub fn set_ttl (& self , ttl : u32) -> io :: Result < () > { setsockopt (& self . inner , c :: IPPROTO_IP , c :: IP_TTL , ttl as c_int) } pub fn ttl (& self) -> io :: Result < u32 > { let raw : c_int = getsockopt (& self . inner , c :: IPPROTO_IP , c :: IP_TTL) ? ; Ok (raw as u32) } pub fn take_error (& self) -> io :: Result < Option < io :: Error > > { self . inner . take_error () } pub fn set_nonblocking (& self , nonblocking : bool) -> io :: Result < () > { self . inner . set_nonblocking (nonblocking) } pub fn recv (& self , buf : & mut [u8]) -> io :: Result < usize > { self . inner . read (buf) } pub fn peek (& self , buf : & mut [u8]) -> io :: Result < usize > { self . inner . peek (buf) } pub fn send (& self , buf : & [u8]) -> io :: Result < usize > { let len = cmp :: min (buf . len () , < wrlen_t > :: MAX as usize) as wrlen_t ; let ret = cvt (unsafe { c :: send (self . inner . as_raw () , buf . as_ptr () as * const c_void , len , MSG_NOSIGNAL) }) ? ; Ok (ret as usize) } pub fn connect < A : ToSocketAddrs > (& self , addr : A) -> io :: Result < () > { return each_addr (addr , | addr | inner (self , addr)) ; fn inner (this : & UdpSocket , addr : & SocketAddr) -> io :: Result < () > { let (addr , len) = socket_addr_to_c (addr) ; cvt_r (| | unsafe { c :: connect (this . inner . as_raw () , addr . as_ptr () , len) }) . map (drop) } } }}}
mkitem!{mkimpl!{impl FromInner < Socket > for UdpSocket { fn from_inner (socket : Socket) -> UdpSocket { UdpSocket { inner : socket } } }}}
mkitem!{mkimpl!{impl fmt :: Debug for UdpSocket { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut res = f . debug_struct ("UdpSocket") ; if let Ok (addr) = self . socket_addr () { res . field ("addr" , & addr) ; } let name = if cfg ! (windows) { "socket" } else { "fd" } ; res . field (name , & self . inner . as_raw ()) . finish () } }}}