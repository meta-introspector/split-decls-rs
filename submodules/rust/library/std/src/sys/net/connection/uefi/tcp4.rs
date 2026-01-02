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
mkuse!{use r_efi :: efi :: { self , Status } ;}
mkuse!{use r_efi :: protocols :: tcp4 ;}
mkuse!{use crate :: io ;}
mkuse!{use crate :: net :: SocketAddrV4 ;}
mkuse!{use crate :: ptr :: NonNull ;}
mkuse!{use crate :: sync :: atomic :: { AtomicBool , Ordering } ;}
mkuse!{use crate :: sys :: pal :: helpers ;}
mkuse!{use crate :: time :: { Duration , Instant } ;}
mkitem!{const TYPE_OF_SERVICE : u8 = 8 ;}
mkitem!{const TIME_TO_LIVE : u8 = 255 ;}
mkitem!{mkstruct!{pub (crate) struct Tcp4 { protocol : NonNull < tcp4 :: Protocol > , flag : AtomicBool , #[expect (dead_code)] service_binding : helpers :: ServiceProtocol , }}}
mkitem!{const DEFAULT_ADDR : efi :: Ipv4Address = efi :: Ipv4Address { addr : [0u8 ; 4] } ;}
mkitem!{mkimpl!{impl Tcp4 { pub (crate) fn new () -> io :: Result < Self > { let service_binding = helpers :: ServiceProtocol :: open (tcp4 :: SERVICE_BINDING_PROTOCOL_GUID) ? ; let protocol = helpers :: open_protocol (service_binding . child_handle () , tcp4 :: PROTOCOL_GUID) ? ; Ok (Self { service_binding , protocol , flag : AtomicBool :: new (false) }) } pub (crate) fn configure (& self , active : bool , remote_address : Option < & SocketAddrV4 > , station_address : Option < & SocketAddrV4 > ,) -> io :: Result < () > { let protocol = self . protocol . as_ptr () ; let (remote_address , remote_port) = if let Some (x) = remote_address { (helpers :: ipv4_to_r_efi (* x . ip ()) , x . port ()) } else { (DEFAULT_ADDR , 0) } ; assert ! (station_address . is_none ()) ; let use_default_address = efi :: Boolean :: TRUE ; let (station_address , station_port) = (DEFAULT_ADDR , 0) ; let subnet_mask = helpers :: ipv4_to_r_efi (crate :: net :: Ipv4Addr :: new (0 , 0 , 0 , 0)) ; let mut config_data = tcp4 :: ConfigData { type_of_service : TYPE_OF_SERVICE , time_to_live : TIME_TO_LIVE , access_point : tcp4 :: AccessPoint { use_default_address , remote_address , remote_port , active_flag : active . into () , station_address , station_port , subnet_mask , } , control_option : crate :: ptr :: null_mut () , } ; let r = unsafe { ((* protocol) . configure) (protocol , & mut config_data) } ; if r . is_error () { Err (crate :: io :: Error :: from_raw_os_error (r . as_usize ())) } else { Ok (()) } } pub (crate) fn get_mode_data (& self) -> io :: Result < tcp4 :: ConfigData > { let mut config_data = tcp4 :: ConfigData :: default () ; let protocol = self . protocol . as_ptr () ; let r = unsafe { ((* protocol) . get_mode_data) (protocol , crate :: ptr :: null_mut () , & mut config_data , crate :: ptr :: null_mut () , crate :: ptr :: null_mut () , crate :: ptr :: null_mut () ,) } ; if r . is_error () { Err (io :: Error :: from_raw_os_error (r . as_usize ())) } else { Ok (config_data) } } pub (crate) fn connect (& self , timeout : Option < Duration >) -> io :: Result < () > { let evt = unsafe { self . create_evt () } ? ; let completion_token = tcp4 :: CompletionToken { event : evt . as_ptr () , status : Status :: SUCCESS } ; let protocol = self . protocol . as_ptr () ; let mut conn_token = tcp4 :: ConnectionToken { completion_token } ; let r = unsafe { ((* protocol) . connect) (protocol , & mut conn_token) } ; if r . is_error () { return Err (io :: Error :: from_raw_os_error (r . as_usize ())) ; } unsafe { self . wait_or_cancel (timeout , & mut conn_token . completion_token) } ? ; if completion_token . status . is_error () { Err (io :: Error :: from_raw_os_error (completion_token . status . as_usize ())) } else { Ok (()) } } pub (crate) fn write (& self , buf : & [u8] , timeout : Option < Duration >) -> io :: Result < usize > { let evt = unsafe { self . create_evt () } ? ; let completion_token = tcp4 :: CompletionToken { event : evt . as_ptr () , status : Status :: SUCCESS } ; let data_len = u32 :: try_from (buf . len ()) . unwrap_or (u32 :: MAX) ; let fragment = tcp4 :: FragmentData { fragment_length : data_len , fragment_buffer : buf . as_ptr () . cast :: < crate :: ffi :: c_void > () . cast_mut () , } ; let mut tx_data = tcp4 :: TransmitData { push : r_efi :: efi :: Boolean :: FALSE , urgent : r_efi :: efi :: Boolean :: FALSE , data_length : data_len , fragment_count : 1 , fragment_table : [fragment] , } ; let protocol = self . protocol . as_ptr () ; let mut token = tcp4 :: IoToken { completion_token , packet : tcp4 :: IoTokenPacket { tx_data : (& raw mut tx_data) . cast :: < tcp4 :: TransmitData < 0 > > () , } , } ; let r = unsafe { ((* protocol) . transmit) (protocol , & mut token) } ; if r . is_error () { return Err (io :: Error :: from_raw_os_error (r . as_usize ())) ; } unsafe { self . wait_or_cancel (timeout , & mut token . completion_token) } ? ; if completion_token . status . is_error () { Err (io :: Error :: from_raw_os_error (completion_token . status . as_usize ())) } else { Ok (data_len as usize) } } pub (crate) fn read (& self , buf : & mut [u8] , timeout : Option < Duration >) -> io :: Result < usize > { let evt = unsafe { self . create_evt () } ? ; let completion_token = tcp4 :: CompletionToken { event : evt . as_ptr () , status : Status :: SUCCESS } ; let data_len = u32 :: try_from (buf . len ()) . unwrap_or (u32 :: MAX) ; let fragment = tcp4 :: FragmentData { fragment_length : data_len , fragment_buffer : buf . as_mut_ptr () . cast :: < crate :: ffi :: c_void > () , } ; let mut tx_data = tcp4 :: ReceiveData { urgent_flag : r_efi :: efi :: Boolean :: FALSE , data_length : data_len , fragment_count : 1 , fragment_table : [fragment] , } ; let protocol = self . protocol . as_ptr () ; let mut token = tcp4 :: IoToken { completion_token , packet : tcp4 :: IoTokenPacket { rx_data : (& raw mut tx_data) . cast :: < tcp4 :: ReceiveData < 0 > > () , } , } ; let r = unsafe { ((* protocol) . receive) (protocol , & mut token) } ; if r . is_error () { return Err (io :: Error :: from_raw_os_error (r . as_usize ())) ; } unsafe { self . wait_or_cancel (timeout , & mut token . completion_token) } ? ; if completion_token . status . is_error () { Err (io :: Error :: from_raw_os_error (completion_token . status . as_usize ())) } else { Ok (data_len as usize) } } #[doc = " Wait for an event to finish. This is checked by an atomic boolean that is supposed to be set"] #[doc = " to true in the event callback."] #[doc = ""] #[doc = " Optionally, allow specifying a timeout."] #[doc = ""] #[doc = " If a timeout is provided, the operation (specified by its `EFI_TCP4_COMPLETION_TOKEN`) is"] #[doc = " canceled and Error of kind TimedOut is returned."] #[doc = ""] #[doc = " # SAFETY"] #[doc = ""] #[doc = " Pointer to a valid `EFI_TCP4_COMPLETION_TOKEN`"] unsafe fn wait_or_cancel (& self , timeout : Option < Duration > , token : * mut tcp4 :: CompletionToken ,) -> io :: Result < () > { if ! self . wait_for_flag (timeout) { let _ = unsafe { self . cancel (token) } ; return Err (io :: Error :: new (io :: ErrorKind :: TimedOut , "Operation Timed out")) ; } Ok (()) } #[doc = " Abort an asynchronous connection, listen, transmission or receive request."] #[doc = ""] #[doc = " If token is NULL, then all pending tokens issued by EFI_TCP4_PROTOCOL.Connect(),"] #[doc = " EFI_TCP4_PROTOCOL.Accept(), EFI_TCP4_PROTOCOL.Transmit() or EFI_TCP4_PROTOCOL.Receive() are"] #[doc = " aborted."] #[doc = ""] #[doc = " # SAFETY"] #[doc = ""] #[doc = " Pointer to a valid `EFI_TCP4_COMPLETION_TOKEN` or NULL"] unsafe fn cancel (& self , token : * mut tcp4 :: CompletionToken) -> io :: Result < () > { let protocol = self . protocol . as_ptr () ; let r = unsafe { ((* protocol) . cancel) (protocol , token) } ; if r . is_error () { return Err (io :: Error :: from_raw_os_error (r . as_usize ())) ; } else { Ok (()) } } unsafe fn create_evt (& self) -> io :: Result < helpers :: OwnedEvent > { self . flag . store (false , Ordering :: Relaxed) ; helpers :: OwnedEvent :: new (efi :: EVT_NOTIFY_SIGNAL , efi :: TPL_CALLBACK , Some (toggle_atomic_flag) , Some (unsafe { NonNull :: new_unchecked (self . flag . as_ptr () . cast ()) }) ,) } fn wait_for_flag (& self , timeout : Option < Duration >) -> bool { let start = Instant :: now () ; while ! self . flag . load (Ordering :: Relaxed) { let _ = self . poll () ; if let Some (t) = timeout { if Instant :: now () . duration_since (start) >= t { return false ; } } } true } fn poll (& self) -> io :: Result < () > { let protocol = self . protocol . as_ptr () ; let r = unsafe { ((* protocol) . poll) (protocol) } ; if r . is_error () { Err (io :: Error :: from_raw_os_error (r . as_usize ())) } else { Ok (()) } } }}}

macro_rules! toggle_atomic_flag_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function toggle_atomic_flag in module {}", module_path!());
    };
}

mkfn!{
    toggle_atomic_flag_introspect!();
    extern "efiapi" fn toggle_atomic_flag (_ : r_efi :: efi :: Event , ctx : * mut crate :: ffi :: c_void) { let flag = unsafe { AtomicBool :: from_ptr (ctx . cast ()) } ; flag . store (true , Ordering :: Relaxed) ; }
}