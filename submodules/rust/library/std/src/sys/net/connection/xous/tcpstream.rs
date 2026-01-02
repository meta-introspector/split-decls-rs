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
mkuse!{use core :: sync :: atomic :: { Atomic , AtomicBool , AtomicU32 , AtomicUsize , Ordering } ;}
mkuse!{use super :: * ;}
mkuse!{use crate :: fmt ;}
mkuse!{use crate :: io :: { self , BorrowedCursor , IoSlice , IoSliceMut } ;}
mkuse!{use crate :: net :: { IpAddr , Ipv4Addr , Shutdown , SocketAddr , SocketAddrV4 , SocketAddrV6 , ToSocketAddrs , } ;}
mkuse!{use crate :: os :: xous :: services ;}
mkuse!{use crate :: sync :: Arc ;}
mkuse!{use crate :: sys :: net :: connection :: each_addr ;}
mkuse!{use crate :: time :: Duration ;}
mkitem!{macro_rules ! unimpl { () => { return Err (io :: const_error ! (io :: ErrorKind :: Unsupported , "this function is not yet implemented" ,)) ; } ; }}
mkitem!{mkenum!{enum ReadOrPeek { Read , Peek , }}}
mkitem!{mkstruct!{# [derive (Clone)] pub struct TcpStream { fd : u16 , local_port : u16 , remote_port : u16 , peer_addr : SocketAddr , read_timeout : Arc < Atomic < u32 > > , write_timeout : Arc < Atomic < u32 > > , handle_count : Arc < Atomic < usize > > , nonblocking : Arc < Atomic < bool > > , }}}

macro_rules! sockaddr_to_buf_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sockaddr_to_buf in module {}", module_path!());
    };
}

mkfn!{
    sockaddr_to_buf_introspect!();
    fn sockaddr_to_buf (duration : Duration , addr : & SocketAddr , buf : & mut [u8]) { let port_bytes = addr . port () . to_le_bytes () ; buf [0] = port_bytes [0] ; buf [1] = port_bytes [1] ; for (dest , src) in buf [2 ..] . iter_mut () . zip ((duration . as_millis () as u64) . to_le_bytes ()) { * dest = src ; } match addr . ip () { IpAddr :: V4 (addr) => { buf [10] = 4 ; for (dest , src) in buf [11 ..] . iter_mut () . zip (addr . octets ()) { * dest = src ; } } IpAddr :: V6 (addr) => { buf [10] = 6 ; for (dest , src) in buf [11 ..] . iter_mut () . zip (addr . octets ()) { * dest = src ; } } } }
}
mkitem!{mkimpl!{impl TcpStream { pub (crate) fn from_listener (fd : u16 , local_port : u16 , remote_port : u16 , peer_addr : SocketAddr ,) -> TcpStream { TcpStream { fd , local_port , remote_port , peer_addr , read_timeout : Arc :: new (AtomicU32 :: new (0)) , write_timeout : Arc :: new (AtomicU32 :: new (0)) , handle_count : Arc :: new (AtomicUsize :: new (1)) , nonblocking : Arc :: new (AtomicBool :: new (false)) , } } pub fn connect < A : ToSocketAddrs > (addr : A) -> io :: Result < TcpStream > { each_addr (addr , | addr | Self :: connect_timeout (addr , Duration :: ZERO)) } pub fn connect_timeout (addr : & SocketAddr , duration : Duration) -> io :: Result < TcpStream > { let mut connect_request = ConnectRequest { raw : [0u8 ; 4096] } ; sockaddr_to_buf (duration , & addr , & mut connect_request . raw) ; let Ok ((_ , valid)) = crate :: os :: xous :: ffi :: lend_mut (services :: net_server () , services :: NetLendMut :: StdTcpConnect . into () , & mut connect_request . raw , 0 , 4096 ,) else { return Err (io :: const_error ! (io :: ErrorKind :: InvalidInput , "invalid response")) ; } ; let response = connect_request . raw ; if response [0] != 0 || valid == 0 { let errcode = response [0] ; if errcode == NetError :: SocketInUse as u8 { return Err (io :: const_error ! (io :: ErrorKind :: ResourceBusy , "socket in use")) ; } else if errcode == NetError :: Unaddressable as u8 { return Err (io :: const_error ! (io :: ErrorKind :: AddrNotAvailable , "invalid address")) ; } else { return Err (io :: const_error ! (io :: ErrorKind :: InvalidInput , "unable to connect or internal error" ,)) ; } } let fd = u16 :: from_le_bytes ([response [2] , response [3]]) ; let local_port = u16 :: from_le_bytes ([response [4] , response [5]]) ; let remote_port = u16 :: from_le_bytes ([response [6] , response [7]]) ; Ok (TcpStream { fd , local_port , remote_port , peer_addr : * addr , read_timeout : Arc :: new (AtomicU32 :: new (0)) , write_timeout : Arc :: new (AtomicU32 :: new (0)) , handle_count : Arc :: new (AtomicUsize :: new (1)) , nonblocking : Arc :: new (AtomicBool :: new (false)) , }) } pub fn set_read_timeout (& self , timeout : Option < Duration >) -> io :: Result < () > { if let Some (to) = timeout { if to . is_zero () { return Err (io :: Error :: ZERO_TIMEOUT) ; } } self . read_timeout . store (timeout . map (| t | t . as_millis () . min (u32 :: MAX as u128) as u32) . unwrap_or_default () , Ordering :: Relaxed ,) ; Ok (()) } pub fn set_write_timeout (& self , timeout : Option < Duration >) -> io :: Result < () > { if let Some (to) = timeout { if to . is_zero () { return Err (io :: Error :: ZERO_TIMEOUT) ; } } self . write_timeout . store (timeout . map (| t | t . as_millis () . min (u32 :: MAX as u128) as u32) . unwrap_or_default () , Ordering :: Relaxed ,) ; Ok (()) } pub fn read_timeout (& self) -> io :: Result < Option < Duration > > { match self . read_timeout . load (Ordering :: Relaxed) { 0 => Ok (None) , t => Ok (Some (Duration :: from_millis (t as u64))) , } } pub fn write_timeout (& self) -> io :: Result < Option < Duration > > { match self . write_timeout . load (Ordering :: Relaxed) { 0 => Ok (None) , t => Ok (Some (Duration :: from_millis (t as u64))) , } } fn read_or_peek (& self , buf : & mut [u8] , op : ReadOrPeek) -> io :: Result < usize > { let mut receive_request = ReceiveData { raw : [0u8 ; 4096] } ; let data_to_read = buf . len () . min (receive_request . raw . len ()) ; let opcode = match op { ReadOrPeek :: Read => { services :: NetLendMut :: StdTcpRx (self . fd , self . nonblocking . load (Ordering :: Relaxed)) } ReadOrPeek :: Peek => { services :: NetLendMut :: StdTcpPeek (self . fd , self . nonblocking . load (Ordering :: Relaxed)) } } ; let Ok ((offset , length)) = crate :: os :: xous :: ffi :: lend_mut (services :: net_server () , opcode . into () , & mut receive_request . raw , self . read_timeout . load (Ordering :: Relaxed) as usize , data_to_read ,) else { return Err (io :: const_error ! (io :: ErrorKind :: InvalidInput , "library failure: wrong message type or messaging error" ,)) ; } ; if offset != 0 { for (dest , src) in buf . iter_mut () . zip (receive_request . raw [.. length] . iter ()) { * dest = * src ; } Ok (length) } else { let result = receive_request . raw ; if result [0] != 0 { if result [1] == 8 { return Err (io :: const_error ! (io :: ErrorKind :: TimedOut , "timeout")) ; } if result [1] == 9 { return Err (io :: const_error ! (io :: ErrorKind :: WouldBlock , "would block")) ; } } Err (io :: const_error ! (io :: ErrorKind :: Other , "recv_slice failure")) } } pub fn peek (& self , buf : & mut [u8]) -> io :: Result < usize > { self . read_or_peek (buf , ReadOrPeek :: Peek) } pub fn read (& self , buf : & mut [u8]) -> io :: Result < usize > { self . read_or_peek (buf , ReadOrPeek :: Read) } pub fn read_vectored (& self , bufs : & mut [IoSliceMut < '_ >]) -> io :: Result < usize > { crate :: io :: default_read_vectored (| b | self . read (b) , bufs) } pub fn read_buf (& self , cursor : BorrowedCursor < '_ >) -> io :: Result < () > { crate :: io :: default_read_buf (| buf | self . read (buf) , cursor) } pub fn is_read_vectored (& self) -> bool { false } pub fn write (& self , buf : & [u8]) -> io :: Result < usize > { let mut send_request = SendData { raw : [0u8 ; 4096] } ; for (dest , src) in send_request . raw . iter_mut () . zip (buf) { * dest = * src ; } let buf_len = send_request . raw . len () . min (buf . len ()) ; let (_offset , _valid) = crate :: os :: xous :: ffi :: lend_mut (services :: net_server () , services :: NetLendMut :: StdTcpTx (self . fd) . into () , & mut send_request . raw , self . write_timeout . load (Ordering :: Relaxed) as usize , buf_len ,) . or (Err (io :: const_error ! (io :: ErrorKind :: InvalidInput , "internal error"))) ? ; if send_request . raw [0] != 0 { if send_request . raw [4] == 8 { return Err (io :: const_error ! (io :: ErrorKind :: BrokenPipe , "timeout or connection closed" ,)) ; } else if send_request . raw [4] == 9 { return Err (io :: const_error ! (io :: ErrorKind :: WouldBlock , "would block")) ; } else { return Err (io :: const_error ! (io :: ErrorKind :: InvalidInput , "error when sending")) ; } } Ok (u32 :: from_le_bytes ([send_request . raw [4] , send_request . raw [5] , send_request . raw [6] , send_request . raw [7] ,]) as usize) } pub fn write_vectored (& self , bufs : & [IoSlice < '_ >]) -> io :: Result < usize > { crate :: io :: default_write_vectored (| b | self . write (b) , bufs) } pub fn is_write_vectored (& self) -> bool { false } pub fn peer_addr (& self) -> io :: Result < SocketAddr > { Ok (self . peer_addr) } pub fn socket_addr (& self) -> io :: Result < SocketAddr > { let mut get_addr = GetAddress { raw : [0u8 ; 4096] } ; let Ok ((_offset , _valid)) = crate :: os :: xous :: ffi :: lend_mut (services :: net_server () , services :: NetLendMut :: StdGetAddress (self . fd) . into () , & mut get_addr . raw , 0 , 0 ,) else { return Err (io :: const_error ! (io :: ErrorKind :: InvalidInput , "internal error")) ; } ; let mut i = get_addr . raw . iter () ; match * i . next () . unwrap () { 4 => Ok (SocketAddr :: V4 (SocketAddrV4 :: new (Ipv4Addr :: new (* i . next () . unwrap () , * i . next () . unwrap () , * i . next () . unwrap () , * i . next () . unwrap () ,) , self . local_port ,))) , 6 => { let mut new_addr = [0u8 ; 16] ; for (src , octet) in i . zip (new_addr . iter_mut ()) { * octet = * src ; } Ok (SocketAddr :: V6 (SocketAddrV6 :: new (new_addr . into () , self . local_port , 0 , 0))) } _ => Err (io :: const_error ! (io :: ErrorKind :: InvalidInput , "internal error")) , } } pub fn shutdown (& self , how : Shutdown) -> io :: Result < () > { crate :: os :: xous :: ffi :: blocking_scalar (services :: net_server () , services :: NetBlockingScalar :: StdTcpStreamShutdown (self . fd , how) . into () ,) . or (Err (io :: const_error ! (io :: ErrorKind :: InvalidInput , "unexpected return value"))) . map (| _ | ()) } pub fn duplicate (& self) -> io :: Result < TcpStream > { self . handle_count . fetch_add (1 , Ordering :: Relaxed) ; Ok (self . clone ()) } pub fn set_linger (& self , _ : Option < Duration >) -> io :: Result < () > { unimpl ! () ; } pub fn linger (& self) -> io :: Result < Option < Duration > > { unimpl ! () ; } pub fn set_nodelay (& self , enabled : bool) -> io :: Result < () > { crate :: os :: xous :: ffi :: blocking_scalar (services :: net_server () , services :: NetBlockingScalar :: StdSetNodelay (self . fd , enabled) . into () ,) . or (Err (io :: const_error ! (io :: ErrorKind :: InvalidInput , "unexpected return value"))) . map (| _ | ()) } pub fn nodelay (& self) -> io :: Result < bool > { Ok (crate :: os :: xous :: ffi :: blocking_scalar (services :: net_server () , services :: NetBlockingScalar :: StdGetNodelay (self . fd) . into () ,) . or (Err (io :: const_error ! (io :: ErrorKind :: InvalidInput , "unexpected return value"))) . map (| res | res [0] != 0) ?) } pub fn set_ttl (& self , ttl : u32) -> io :: Result < () > { if ttl > 255 { return Err (io :: const_error ! (io :: ErrorKind :: InvalidInput , "TTL must be less than 256")) ; } crate :: os :: xous :: ffi :: blocking_scalar (services :: net_server () , services :: NetBlockingScalar :: StdSetTtlTcp (self . fd , ttl) . into () ,) . or (Err (io :: const_error ! (io :: ErrorKind :: InvalidInput , "unexpected return value"))) . map (| _ | ()) } pub fn ttl (& self) -> io :: Result < u32 > { Ok (crate :: os :: xous :: ffi :: blocking_scalar (services :: net_server () , services :: NetBlockingScalar :: StdGetTtlTcp (self . fd) . into () ,) . or (Err (io :: const_error ! (io :: ErrorKind :: InvalidInput , "unexpected return value"))) . map (| res | res [0] as _) ?) } pub fn take_error (& self) -> io :: Result < Option < io :: Error > > { Ok (None) } pub fn set_nonblocking (& self , nonblocking : bool) -> io :: Result < () > { self . nonblocking . store (nonblocking , Ordering :: Relaxed) ; Ok (()) } }}}
mkitem!{mkimpl!{impl fmt :: Debug for TcpStream { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "TCP connection to {:?} port {} to local port {}" , self . peer_addr , self . remote_port , self . local_port) } }}}
mkitem!{mkimpl!{impl Drop for TcpStream { fn drop (& mut self) { if self . handle_count . fetch_sub (1 , Ordering :: Relaxed) == 1 { crate :: os :: xous :: ffi :: blocking_scalar (services :: net_server () , services :: NetBlockingScalar :: StdTcpClose (self . fd) . into () ,) . unwrap () ; } } }}}