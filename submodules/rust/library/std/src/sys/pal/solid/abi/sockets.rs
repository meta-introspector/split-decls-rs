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
mkuse!{pub use libc :: { c_int , c_long , size_t , ssize_t , timeval } ;}
mkuse!{use crate :: os :: raw :: { c_char , c_uint , c_void } ;}
mkitem!{pub const SOLID_NET_ERR_BASE : c_int = - 2000 ;}
mkitem!{pub const EINPROGRESS : c_int = SOLID_NET_ERR_BASE - libc :: EINPROGRESS ;}
mkitem!{pub const AF_INET6 : i32 = 10 ;}
mkitem!{pub const AF_INET : i32 = 2 ;}
mkitem!{pub const IPPROTO_IP : i32 = 0 ;}
mkitem!{pub const IPPROTO_IPV6 : i32 = 41 ;}
mkitem!{pub const IPPROTO_TCP : i32 = 6 ;}
mkitem!{pub const IPV6_ADD_MEMBERSHIP : i32 = 12 ;}
mkitem!{pub const IPV6_DROP_MEMBERSHIP : i32 = 13 ;}
mkitem!{pub const IPV6_MULTICAST_LOOP : i32 = 19 ;}
mkitem!{pub const IPV6_V6ONLY : i32 = 27 ;}
mkitem!{pub const IP_TTL : i32 = 2 ;}
mkitem!{pub const IP_MULTICAST_TTL : i32 = 5 ;}
mkitem!{pub const IP_MULTICAST_LOOP : i32 = 7 ;}
mkitem!{pub const IP_ADD_MEMBERSHIP : i32 = 3 ;}
mkitem!{pub const IP_DROP_MEMBERSHIP : i32 = 4 ;}
mkitem!{pub const SHUT_RD : i32 = 0 ;}
mkitem!{pub const SHUT_RDWR : i32 = 2 ;}
mkitem!{pub const SHUT_WR : i32 = 1 ;}
mkitem!{pub const SOCK_DGRAM : i32 = 2 ;}
mkitem!{pub const SOCK_STREAM : i32 = 1 ;}
mkitem!{pub const SOL_SOCKET : i32 = 4095 ;}
mkitem!{pub const SO_BROADCAST : i32 = 32 ;}
mkitem!{pub const SO_ERROR : i32 = 4103 ;}
mkitem!{pub const SO_RCVTIMEO : i32 = 4102 ;}
mkitem!{pub const SO_REUSEADDR : i32 = 4 ;}
mkitem!{pub const SO_SNDTIMEO : i32 = 4101 ;}
mkitem!{pub const SO_LINGER : i32 = 128 ;}
mkitem!{pub const TCP_NODELAY : i32 = 1 ;}
mkitem!{pub const MSG_PEEK : c_int = 1 ;}
mkitem!{pub const FIONBIO : c_long = 0x8008667eu32 as c_long ;}
mkitem!{pub const EAI_NONAME : i32 = - 2200 ;}
mkitem!{pub const EAI_SERVICE : i32 = - 2201 ;}
mkitem!{pub const EAI_FAIL : i32 = - 2202 ;}
mkitem!{pub const EAI_MEMORY : i32 = - 2203 ;}
mkitem!{pub const EAI_FAMILY : i32 = - 2204 ;}
mkitem!{pub type sa_family_t = u8 ;}
mkitem!{pub type socklen_t = u32 ;}
mkitem!{pub type in_addr_t = u32 ;}
mkitem!{pub type in_port_t = u16 ;}
mkitem!{mkstruct!{# [repr (C)] # [derive (Debug , Copy , Clone)] pub struct in_addr { pub s_addr : in_addr_t , }}}
mkitem!{mkstruct!{# [repr (C)] # [derive (Debug , Copy , Clone)] pub struct in6_addr { pub s6_addr : [u8 ; 16] , }}}
mkitem!{mkstruct!{# [repr (C)] # [derive (Debug , Copy , Clone)] pub struct ip_mreq { pub imr_multiaddr : in_addr , pub imr_interface : in_addr , }}}
mkitem!{mkstruct!{# [repr (C)] # [derive (Debug , Copy , Clone)] pub struct ipv6_mreq { pub ipv6mr_multiaddr : in6_addr , pub ipv6mr_interface : c_uint , }}}
mkitem!{mkstruct!{# [repr (C)] # [derive (Debug , Copy , Clone)] pub struct msghdr { pub msg_name : * mut c_void , pub msg_namelen : socklen_t , pub msg_iov : * mut iovec , pub msg_iovlen : c_int , pub msg_control : * mut c_void , pub msg_controllen : socklen_t , pub msg_flags : c_int , }}}
mkitem!{mkstruct!{# [repr (C)] # [derive (Debug , Copy , Clone)] pub struct sockaddr { pub sa_len : u8 , pub sa_family : sa_family_t , pub sa_data : [c_char ; 14usize] , }}}
mkitem!{mkstruct!{# [repr (C)] # [derive (Debug , Copy , Clone)] pub struct sockaddr_in { pub sin_len : u8 , pub sin_family : sa_family_t , pub sin_port : in_port_t , pub sin_addr : in_addr , pub sin_zero : [c_char ; 8usize] , }}}
mkitem!{mkstruct!{# [repr (C)] # [derive (Copy , Clone)] pub struct sockaddr_in6 { pub sin6_len : u8 , pub sin6_family : sa_family_t , pub sin6_port : in_port_t , pub sin6_flowinfo : u32 , pub sin6_addr : in6_addr , pub sin6_scope_id : u32 , }}}
mkitem!{mkstruct!{# [repr (C)] # [derive (Debug , Copy , Clone)] pub struct sockaddr_storage { pub s2_len : u8 , pub ss_family : sa_family_t , pub s2_data1 : [c_char ; 2usize] , pub s2_data2 : [u32 ; 3usize] , }}}
mkitem!{mkstruct!{# [repr (C)] # [derive (Debug , Copy , Clone)] pub struct addrinfo { pub ai_flags : c_int , pub ai_family : c_int , pub ai_socktype : c_int , pub ai_protocol : c_int , pub ai_addrlen : socklen_t , pub ai_addr : * mut sockaddr , pub ai_canonname : * mut c_char , pub ai_next : * mut addrinfo , }}}
mkitem!{mkstruct!{# [repr (C)] # [derive (Debug , Copy , Clone)] pub struct linger { pub l_onoff : c_int , pub l_linger : c_int , }}}
mkitem!{mkstruct!{# [repr (C)] # [derive (Debug , Copy , Clone)] pub struct iovec { pub iov_base : * mut c_void , pub iov_len : usize , }}}
mkitem!{# [doc = " This value can be chosen by an application"] pub const SOLID_NET_FD_SETSIZE : usize = 1 ;}
mkitem!{mkstruct!{# [repr (C)] # [derive (Debug , Copy , Clone)] pub struct fd_set { pub num_fds : usize , pub fds : [c_int ; SOLID_NET_FD_SETSIZE] , }}}
mkitem!{unsafe extern "C" { # [link_name = "SOLID_NET_StrError"] pub fn strerror (errnum : c_int) -> * const c_char ; pub fn SOLID_NET_GetLastError () -> c_int ; # [link_name = "SOLID_NET_Accept"] pub fn accept (s : c_int , addr : * mut sockaddr , addrlen : * mut socklen_t) -> c_int ; # [link_name = "SOLID_NET_Bind"] pub fn bind (s : c_int , name : * const sockaddr , namelen : socklen_t) -> c_int ; # [link_name = "SOLID_NET_Connect"] pub fn connect (s : c_int , name : * const sockaddr , namelen : socklen_t) -> c_int ; # [link_name = "SOLID_NET_Close"] pub fn close (s : c_int) -> c_int ; # [link_name = "SOLID_NET_Dup"] pub fn dup (s : c_int) -> c_int ; # [link_name = "SOLID_NET_GetPeerName"] pub fn getpeername (s : c_int , name : * mut sockaddr , namelen : * mut socklen_t) -> c_int ; # [link_name = "SOLID_NET_GetSockName"] pub fn getsockname (s : c_int , name : * mut sockaddr , namelen : * mut socklen_t) -> c_int ; # [link_name = "SOLID_NET_GetSockOpt"] pub fn getsockopt (s : c_int , level : c_int , optname : c_int , optval : * mut c_void , optlen : * mut socklen_t ,) -> c_int ; # [link_name = "SOLID_NET_SetSockOpt"] pub fn setsockopt (s : c_int , level : c_int , optname : c_int , optval : * const c_void , optlen : socklen_t ,) -> c_int ; # [link_name = "SOLID_NET_Ioctl"] pub fn ioctl (s : c_int , cmd : c_long , argp : * mut c_void) -> c_int ; # [link_name = "SOLID_NET_Listen"] pub fn listen (s : c_int , backlog : c_int) -> c_int ; # [link_name = "SOLID_NET_Recv"] pub fn recv (s : c_int , mem : * mut c_void , len : size_t , flags : c_int) -> ssize_t ; # [link_name = "SOLID_NET_Read"] pub fn read (s : c_int , mem : * mut c_void , len : size_t) -> ssize_t ; # [link_name = "SOLID_NET_Readv"] pub fn readv (s : c_int , bufs : * const iovec , bufcnt : c_int) -> ssize_t ; # [link_name = "SOLID_NET_RecvFrom"] pub fn recvfrom (s : c_int , mem : * mut c_void , len : size_t , flags : c_int , from : * mut sockaddr , fromlen : * mut socklen_t ,) -> ssize_t ; # [link_name = "SOLID_NET_Send"] pub fn send (s : c_int , mem : * const c_void , len : size_t , flags : c_int) -> ssize_t ; # [link_name = "SOLID_NET_SendMsg"] pub fn sendmsg (s : c_int , message : * const msghdr , flags : c_int) -> ssize_t ; # [link_name = "SOLID_NET_SendTo"] pub fn sendto (s : c_int , mem : * const c_void , len : size_t , flags : c_int , to : * const sockaddr , tolen : socklen_t ,) -> ssize_t ; # [link_name = "SOLID_NET_Shutdown"] pub fn shutdown (s : c_int , how : c_int) -> c_int ; # [link_name = "SOLID_NET_Socket"] pub fn socket (domain : c_int , type_ : c_int , protocol : c_int) -> c_int ; # [link_name = "SOLID_NET_Write"] pub fn write (s : c_int , mem : * const c_void , len : size_t) -> ssize_t ; # [link_name = "SOLID_NET_Writev"] pub fn writev (s : c_int , bufs : * const iovec , bufcnt : c_int) -> ssize_t ; # [link_name = "SOLID_NET_FreeAddrInfo"] pub fn freeaddrinfo (ai : * mut addrinfo) ; # [link_name = "SOLID_NET_GetAddrInfo"] pub fn getaddrinfo (nodename : * const c_char , servname : * const c_char , hints : * const addrinfo , res : * mut * mut addrinfo ,) -> c_int ; # [link_name = "SOLID_NET_Select"] pub fn select (maxfdp1 : c_int , readset : * mut fd_set , writeset : * mut fd_set , exceptset : * mut fd_set , timeout : * mut timeval ,) -> c_int ; }}