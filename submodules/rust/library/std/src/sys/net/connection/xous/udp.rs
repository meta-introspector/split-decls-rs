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
mkuse!{use core :: convert :: TryInto ;}
mkuse!{use core :: sync :: atomic :: { Atomic , AtomicUsize , Ordering } ;}
mkuse!{use super :: * ;}
mkuse!{use crate :: cell :: Cell ;}
mkuse!{use crate :: net :: { IpAddr , Ipv4Addr , Ipv6Addr , SocketAddr , ToSocketAddrs } ;}
mkuse!{use crate :: os :: xous :: services ;}
mkuse!{use crate :: sync :: Arc ;}
mkuse!{use crate :: sys :: net :: connection :: each_addr ;}
mkuse!{use crate :: time :: Duration ;}
mkuse!{use crate :: { fmt , io } ;}
mkitem!{macro_rules ! unimpl { () => { return Err (io :: const_error ! (io :: ErrorKind :: Unsupported , "this function is not yet implemented" ,)) ; } ; }}
mkitem!{mkstruct!{#[derive (Clone)] pub struct UdpSocket { fd : u16 , local : SocketAddr , remote : Cell < Option < SocketAddr > > , read_timeout : Cell < u64 > , write_timeout : Cell < u64 > , handle_count : Arc < Atomic < usize > > , nonblocking : Cell < bool > , }}}
mkitem!{mkimpl!{impl UdpSocket { pub fn bind < A : ToSocketAddrs > (addr : A) -> io :: Result < UdpSocket > { return each_addr (addr , inner) ; fn inner (addr : & SocketAddr) -> io :: Result < UdpSocket > { let mut connect_request = ConnectRequest { raw : [0u8 ; 4096] } ; let port_bytes = addr . port () . to_le_bytes () ; connect_request . raw [0] = port_bytes [0] ; connect_request . raw [1] = port_bytes [1] ; match addr . ip () { IpAddr :: V4 (addr) => { connect_request . raw [2] = 4 ; for (dest , src) in connect_request . raw [3 ..] . iter_mut () . zip (addr . octets ()) { * dest = src ; } } IpAddr :: V6 (addr) => { connect_request . raw [2] = 6 ; for (dest , src) in connect_request . raw [3 ..] . iter_mut () . zip (addr . octets ()) { * dest = src ; } } } let response = crate :: os :: xous :: ffi :: lend_mut (services :: net_server () , services :: NetLendMut :: StdUdpBind . into () , & mut connect_request . raw , 0 , 4096 ,) ; let Ok ((_ , valid)) = response else { return Err (io :: const_error ! (io :: ErrorKind :: InvalidInput , "invalid response")) ; } ; let response = connect_request . raw ; if response [0] != 0 || valid == 0 { let errcode = response [1] ; if errcode == NetError :: SocketInUse as u8 { return Err (io :: const_error ! (io :: ErrorKind :: ResourceBusy , "socket in use")) ; } else if errcode == NetError :: Invalid as u8 { return Err (io :: const_error ! (io :: ErrorKind :: InvalidInput , "port can't be 0 or invalid address" ,)) ; } else if errcode == NetError :: LibraryError as u8 { return Err (io :: const_error ! (io :: ErrorKind :: Other , "library error")) ; } else { return Err (io :: const_error ! (io :: ErrorKind :: Other , "unable to connect or internal error" ,)) ; } } let fd = response [1] as u16 ; Ok (UdpSocket { fd , local : * addr , remote : Cell :: new (None) , read_timeout : Cell :: new (0) , write_timeout : Cell :: new (0) , handle_count : Arc :: new (AtomicUsize :: new (1)) , nonblocking : Cell :: new (false) , }) } } pub fn peer_addr (& self) -> io :: Result < SocketAddr > { match self . remote . get () { Some (dest) => Ok (dest) , None => Err (io :: const_error ! (io :: ErrorKind :: NotConnected , "no peer specified")) , } } pub fn socket_addr (& self) -> io :: Result < SocketAddr > { Ok (self . local) } fn recv_inner (& self , buf : & mut [u8] , do_peek : bool) -> io :: Result < (usize , SocketAddr) > { let mut receive_request = ReceiveData { raw : [0u8 ; 4096] } ; if self . nonblocking . get () { receive_request . raw [0] = 0 ; } else { receive_request . raw [0] = 1 ; for (& s , d) in self . read_timeout . get () . to_le_bytes () . iter () . zip (receive_request . raw [1 .. 9] . iter_mut ()) { * d = s ; } } if let Ok ((_offset , _valid)) = crate :: os :: xous :: ffi :: lend_mut (services :: net_server () , services :: NetLendMut :: StdUdpRx (self . fd) . into () , & mut receive_request . raw , if do_peek { 1 } else { 0 } , 0 ,) { if receive_request . raw [0] != 0 { if receive_request . raw [1] == NetError :: TimedOut as u8 { return Err (io :: const_error ! (io :: ErrorKind :: TimedOut , "recv timed out")) ; } else if receive_request . raw [1] == NetError :: WouldBlock as u8 { return Err (io :: const_error ! (io :: ErrorKind :: WouldBlock , "recv would block")) ; } else if receive_request . raw [1] == NetError :: LibraryError as u8 { return Err (io :: const_error ! (io :: ErrorKind :: Other , "library error")) ; } else { return Err (io :: const_error ! (io :: ErrorKind :: Other , "library error")) ; } } else { let rr = & receive_request . raw ; let rxlen = u16 :: from_le_bytes (rr [1 .. 3] . try_into () . unwrap ()) ; let port = u16 :: from_le_bytes (rr [20 .. 22] . try_into () . unwrap ()) ; let addr = if rr [3] == 4 { SocketAddr :: new (IpAddr :: V4 (Ipv4Addr :: new (rr [4] , rr [5] , rr [6] , rr [7])) , port) } else if rr [3] == 6 { SocketAddr :: new (IpAddr :: V6 (Ipv6Addr :: new (u16 :: from_be_bytes (rr [4 .. 6] . try_into () . unwrap ()) , u16 :: from_be_bytes (rr [6 .. 8] . try_into () . unwrap ()) , u16 :: from_be_bytes (rr [8 .. 10] . try_into () . unwrap ()) , u16 :: from_be_bytes (rr [10 .. 12] . try_into () . unwrap ()) , u16 :: from_be_bytes (rr [12 .. 14] . try_into () . unwrap ()) , u16 :: from_be_bytes (rr [14 .. 16] . try_into () . unwrap ()) , u16 :: from_be_bytes (rr [16 .. 18] . try_into () . unwrap ()) , u16 :: from_be_bytes (rr [18 .. 20] . try_into () . unwrap ()) ,)) , port ,) } else { return Err (io :: const_error ! (io :: ErrorKind :: Other , "library error")) ; } ; for (& s , d) in rr [22 .. 22 + rxlen as usize] . iter () . zip (buf . iter_mut ()) { * d = s ; } Ok ((rxlen as usize , addr)) } } else { Err (io :: const_error ! (io :: ErrorKind :: InvalidInput , "unable to recv")) } } pub fn recv_from (& self , buf : & mut [u8]) -> io :: Result < (usize , SocketAddr) > { self . recv_inner (buf , false) } pub fn recv (& self , buf : & mut [u8]) -> io :: Result < usize > { self . recv_from (buf) . map (| (len , _addr) | len) } pub fn peek_from (& self , buf : & mut [u8]) -> io :: Result < (usize , SocketAddr) > { self . recv_inner (buf , true) } pub fn peek (& self , buf : & mut [u8]) -> io :: Result < usize > { self . peek_from (buf) . map (| (len , _addr) | len) } pub fn connect < A : ToSocketAddrs > (& self , addr : A) -> io :: Result < () > { each_addr (addr , | addr | { self . remote . set (Some (* addr)) ; Ok (()) }) } pub fn send (& self , buf : & [u8]) -> io :: Result < usize > { if let Some (addr) = self . remote . get () { self . send_to (buf , & addr) } else { Err (io :: const_error ! (io :: ErrorKind :: NotConnected , "No remote specified")) } } pub fn send_to (& self , buf : & [u8] , addr : & SocketAddr) -> io :: Result < usize > { let mut tx_req = SendData { raw : [0u8 ; 4096] } ; let port_bytes = addr . port () . to_le_bytes () ; tx_req . raw [0] = port_bytes [0] ; tx_req . raw [1] = port_bytes [1] ; match addr . ip () { IpAddr :: V4 (addr) => { tx_req . raw [2] = 4 ; for (dest , src) in tx_req . raw [3 ..] . iter_mut () . zip (addr . octets ()) { * dest = src ; } } IpAddr :: V6 (addr) => { tx_req . raw [2] = 6 ; for (dest , src) in tx_req . raw [3 ..] . iter_mut () . zip (addr . octets ()) { * dest = src ; } } } let len = buf . len () as u16 ; let len_bytes = len . to_le_bytes () ; tx_req . raw [19] = len_bytes [0] ; tx_req . raw [20] = len_bytes [1] ; for (& s , d) in buf . iter () . zip (tx_req . raw [21 ..] . iter_mut ()) { * d = s ; } let now = crate :: time :: Instant :: now () ; let write_timeout = if self . nonblocking . get () { core :: time :: Duration :: ZERO } else { if self . write_timeout . get () == 0 { core :: time :: Duration :: from_millis (u64 :: MAX) } else { core :: time :: Duration :: from_millis (self . write_timeout . get ()) } } ; loop { let response = crate :: os :: xous :: ffi :: try_lend_mut (services :: net_server () , services :: NetLendMut :: StdUdpTx (self . fd) . into () , & mut tx_req . raw , 0 , 4096 ,) ; match response { Ok ((_ , valid)) => { let response = & tx_req . raw ; if response [0] != 0 || valid == 0 { let errcode = response [1] ; if errcode == NetError :: SocketInUse as u8 { return Err (io :: const_error ! (io :: ErrorKind :: ResourceBusy , "socket in use" ,)) ; } else if errcode == NetError :: Invalid as u8 { return Err (io :: const_error ! (io :: ErrorKind :: InvalidInput , "socket not valid" ,)) ; } else if errcode == NetError :: LibraryError as u8 { return Err (io :: const_error ! (io :: ErrorKind :: Other , "library error")) ; } else { return Err (io :: const_error ! (io :: ErrorKind :: Other , "unable to connect" ,)) ; } } else { return Ok (len as usize) ; } } Err (crate :: os :: xous :: ffi :: Error :: ServerQueueFull) => { if now . elapsed () >= write_timeout { return Err (io :: const_error ! (io :: ErrorKind :: WouldBlock , "write timed out")) ; } else { crate :: thread :: yield_now () ; } } _ => return Err (io :: const_error ! (io :: ErrorKind :: Other , "library error")) , } } } pub fn duplicate (& self) -> io :: Result < UdpSocket > { self . handle_count . fetch_add (1 , Ordering :: Relaxed) ; Ok (self . clone ()) } pub fn set_read_timeout (& self , timeout : Option < Duration >) -> io :: Result < () > { if let Some (d) = timeout { if d . is_zero () { return Err (io :: Error :: ZERO_TIMEOUT) ; } } self . read_timeout . set (timeout . map (| t | t . as_millis () . min (u64 :: MAX as u128) as u64) . unwrap_or_default ()) ; Ok (()) } pub fn set_write_timeout (& self , timeout : Option < Duration >) -> io :: Result < () > { if let Some (d) = timeout { if d . is_zero () { return Err (io :: Error :: ZERO_TIMEOUT) ; } } self . write_timeout . set (timeout . map (| t | t . as_millis () . min (u64 :: MAX as u128) as u64) . unwrap_or_default ()) ; Ok (()) } pub fn read_timeout (& self) -> io :: Result < Option < Duration > > { match self . read_timeout . get () { 0 => Ok (None) , t => Ok (Some (Duration :: from_millis (t as u64))) , } } pub fn write_timeout (& self) -> io :: Result < Option < Duration > > { match self . write_timeout . get () { 0 => Ok (None) , t => Ok (Some (Duration :: from_millis (t as u64))) , } } pub fn set_ttl (& self , ttl : u32) -> io :: Result < () > { if ttl > 255 { return Err (io :: const_error ! (io :: ErrorKind :: InvalidInput , "TTL must be less than 256")) ; } crate :: os :: xous :: ffi :: blocking_scalar (services :: net_server () , services :: NetBlockingScalar :: StdSetTtlUdp (self . fd , ttl) . into () ,) . or (Err (io :: const_error ! (io :: ErrorKind :: InvalidInput , "unexpected return value"))) . map (| _ | ()) } pub fn ttl (& self) -> io :: Result < u32 > { Ok (crate :: os :: xous :: ffi :: blocking_scalar (services :: net_server () , services :: NetBlockingScalar :: StdGetTtlUdp (self . fd) . into () ,) . or (Err (io :: const_error ! (io :: ErrorKind :: InvalidInput , "unexpected return value"))) . map (| res | res [0] as _) ?) } pub fn take_error (& self) -> io :: Result < Option < io :: Error > > { Ok (None) } pub fn set_nonblocking (& self , nonblocking : bool) -> io :: Result < () > { self . nonblocking . set (nonblocking) ; Ok (()) } pub fn set_broadcast (& self , _ : bool) -> io :: Result < () > { unimpl ! () ; } pub fn broadcast (& self) -> io :: Result < bool > { unimpl ! () ; } pub fn set_multicast_loop_v4 (& self , _ : bool) -> io :: Result < () > { unimpl ! () ; } pub fn multicast_loop_v4 (& self) -> io :: Result < bool > { unimpl ! () ; } pub fn set_multicast_ttl_v4 (& self , _ : u32) -> io :: Result < () > { unimpl ! () ; } pub fn multicast_ttl_v4 (& self) -> io :: Result < u32 > { unimpl ! () ; } pub fn set_multicast_loop_v6 (& self , _ : bool) -> io :: Result < () > { unimpl ! () ; } pub fn multicast_loop_v6 (& self) -> io :: Result < bool > { unimpl ! () ; } pub fn join_multicast_v4 (& self , _ : & Ipv4Addr , _ : & Ipv4Addr) -> io :: Result < () > { unimpl ! () ; } pub fn join_multicast_v6 (& self , _ : & Ipv6Addr , _ : u32) -> io :: Result < () > { unimpl ! () ; } pub fn leave_multicast_v4 (& self , _ : & Ipv4Addr , _ : & Ipv4Addr) -> io :: Result < () > { unimpl ! () ; } pub fn leave_multicast_v6 (& self , _ : & Ipv6Addr , _ : u32) -> io :: Result < () > { unimpl ! () ; } }}}
mkitem!{mkimpl!{impl fmt :: Debug for UdpSocket { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "UDP listening on {:?} to {:?}" , self . local , self . remote . get ()) } }}}
mkitem!{mkimpl!{impl Drop for UdpSocket { fn drop (& mut self) { if self . handle_count . fetch_sub (1 , Ordering :: Relaxed) == 1 { crate :: os :: xous :: ffi :: blocking_scalar (services :: net_server () , services :: NetBlockingScalar :: StdUdpClose (self . fd) . into () ,) . unwrap () ; } } }}}