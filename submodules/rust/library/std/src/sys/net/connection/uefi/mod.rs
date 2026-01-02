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
mkuse!{use super :: each_addr ;}
mkuse!{use crate :: fmt ;}
mkuse!{use crate :: io :: { self , BorrowedCursor , IoSlice , IoSliceMut } ;}
mkuse!{use crate :: net :: { Ipv4Addr , Ipv6Addr , Shutdown , SocketAddr , ToSocketAddrs } ;}
mkuse!{use crate :: sync :: { Arc , Mutex } ;}
mkuse!{use crate :: sys :: unsupported ;}
mkuse!{use crate :: time :: Duration ;}
mkmod!{tcp, { 
                getname!(tcp);
                getsrc!(tcp);
                getpath!(tcp);
                get_deps!(tcp);
                get_crates!(tcp);
                mkinclude!(tcp);
                 
            }}
mkmod!{tcp4, { 
                getname!(tcp4);
                getsrc!(tcp4);
                getpath!(tcp4);
                get_deps!(tcp4);
                get_crates!(tcp4);
                mkinclude!(tcp4);
                 
            }}
mkitem!{mkstruct!{pub struct TcpStream { inner : tcp :: Tcp , read_timeout : Arc < Mutex < Option < Duration > > > , write_timeout : Arc < Mutex < Option < Duration > > > , }}}
mkitem!{mkimpl!{impl TcpStream { pub fn connect < A : ToSocketAddrs > (addr : A) -> io :: Result < TcpStream > { return each_addr (addr , inner) ; fn inner (addr : & SocketAddr) -> io :: Result < TcpStream > { let inner = tcp :: Tcp :: connect (addr , None) ? ; Ok (TcpStream { inner , read_timeout : Arc :: new (Mutex :: new (None)) , write_timeout : Arc :: new (Mutex :: new (None)) , }) } } pub fn connect_timeout (addr : & SocketAddr , timeout : Duration) -> io :: Result < TcpStream > { let inner = tcp :: Tcp :: connect (addr , Some (timeout)) ? ; Ok (Self { inner , read_timeout : Arc :: new (Mutex :: new (None)) , write_timeout : Arc :: new (Mutex :: new (None)) , }) } pub fn set_read_timeout (& self , t : Option < Duration >) -> io :: Result < () > { self . read_timeout . set (t) . unwrap () ; Ok (()) } pub fn set_write_timeout (& self , t : Option < Duration >) -> io :: Result < () > { self . write_timeout . set (t) . unwrap () ; Ok (()) } pub fn read_timeout (& self) -> io :: Result < Option < Duration > > { Ok (self . read_timeout . get_cloned () . unwrap ()) } pub fn write_timeout (& self) -> io :: Result < Option < Duration > > { Ok (self . write_timeout . get_cloned () . unwrap ()) } pub fn peek (& self , _ : & mut [u8]) -> io :: Result < usize > { unsupported () } pub fn read (& self , buf : & mut [u8]) -> io :: Result < usize > { self . inner . read (buf , self . read_timeout () ?) } pub fn read_buf (& self , cursor : BorrowedCursor < '_ >) -> io :: Result < () > { crate :: io :: default_read_buf (| buf | self . read (buf) , cursor) } pub fn read_vectored (& self , buf : & mut [IoSliceMut < '_ >]) -> io :: Result < usize > { crate :: io :: default_read_vectored (| b | self . read (b) , buf) } pub fn is_read_vectored (& self) -> bool { false } pub fn write (& self , buf : & [u8]) -> io :: Result < usize > { self . inner . write (buf , self . write_timeout () ?) } pub fn write_vectored (& self , buf : & [IoSlice < '_ >]) -> io :: Result < usize > { crate :: io :: default_write_vectored (| b | self . write (b) , buf) } pub fn is_write_vectored (& self) -> bool { false } pub fn peer_addr (& self) -> io :: Result < SocketAddr > { self . inner . peer_addr () } pub fn socket_addr (& self) -> io :: Result < SocketAddr > { self . inner . socket_addr () } pub fn shutdown (& self , _ : Shutdown) -> io :: Result < () > { unsupported () } pub fn duplicate (& self) -> io :: Result < TcpStream > { unsupported () } pub fn set_linger (& self , _ : Option < Duration >) -> io :: Result < () > { unsupported () } pub fn linger (& self) -> io :: Result < Option < Duration > > { unsupported () } pub fn set_nodelay (& self , _ : bool) -> io :: Result < () > { unsupported () } pub fn nodelay (& self) -> io :: Result < bool > { self . inner . nodelay () } pub fn set_ttl (& self , _ : u32) -> io :: Result < () > { unsupported () } pub fn ttl (& self) -> io :: Result < u32 > { self . inner . ttl () } pub fn take_error (& self) -> io :: Result < Option < io :: Error > > { unsupported () } pub fn set_nonblocking (& self , _ : bool) -> io :: Result < () > { unsupported () } }}}
mkitem!{mkimpl!{impl fmt :: Debug for TcpStream { fn fmt (& self , _f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { todo ! () } }}}
mkitem!{mkstruct!{pub struct TcpListener { inner : tcp :: Tcp , }}}
mkitem!{mkimpl!{impl TcpListener { pub fn bind < A : ToSocketAddrs > (_ : A) -> io :: Result < TcpListener > { unsupported () } pub fn socket_addr (& self) -> io :: Result < SocketAddr > { unsupported () } pub fn accept (& self) -> io :: Result < (TcpStream , SocketAddr) > { unsupported () } pub fn duplicate (& self) -> io :: Result < TcpListener > { unsupported () } pub fn set_ttl (& self , _ : u32) -> io :: Result < () > { unsupported () } pub fn ttl (& self) -> io :: Result < u32 > { self . inner . ttl () } pub fn set_only_v6 (& self , _ : bool) -> io :: Result < () > { unsupported () } pub fn only_v6 (& self) -> io :: Result < bool > { unsupported () } pub fn take_error (& self) -> io :: Result < Option < io :: Error > > { unsupported () } pub fn set_nonblocking (& self , _ : bool) -> io :: Result < () > { unsupported () } }}}
mkitem!{mkimpl!{impl fmt :: Debug for TcpListener { fn fmt (& self , _f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { todo ! () } }}}
mkitem!{mkstruct!{pub struct UdpSocket (!) ;}}
mkitem!{mkimpl!{impl UdpSocket { pub fn bind < A : ToSocketAddrs > (_ : A) -> io :: Result < UdpSocket > { unsupported () } pub fn peer_addr (& self) -> io :: Result < SocketAddr > { self . 0 } pub fn socket_addr (& self) -> io :: Result < SocketAddr > { self . 0 } pub fn recv_from (& self , _ : & mut [u8]) -> io :: Result < (usize , SocketAddr) > { self . 0 } pub fn peek_from (& self , _ : & mut [u8]) -> io :: Result < (usize , SocketAddr) > { self . 0 } pub fn send_to (& self , _ : & [u8] , _ : & SocketAddr) -> io :: Result < usize > { self . 0 } pub fn duplicate (& self) -> io :: Result < UdpSocket > { self . 0 } pub fn set_read_timeout (& self , _ : Option < Duration >) -> io :: Result < () > { self . 0 } pub fn set_write_timeout (& self , _ : Option < Duration >) -> io :: Result < () > { self . 0 } pub fn read_timeout (& self) -> io :: Result < Option < Duration > > { self . 0 } pub fn write_timeout (& self) -> io :: Result < Option < Duration > > { self . 0 } pub fn set_broadcast (& self , _ : bool) -> io :: Result < () > { self . 0 } pub fn broadcast (& self) -> io :: Result < bool > { self . 0 } pub fn set_multicast_loop_v4 (& self , _ : bool) -> io :: Result < () > { self . 0 } pub fn multicast_loop_v4 (& self) -> io :: Result < bool > { self . 0 } pub fn set_multicast_ttl_v4 (& self , _ : u32) -> io :: Result < () > { self . 0 } pub fn multicast_ttl_v4 (& self) -> io :: Result < u32 > { self . 0 } pub fn set_multicast_loop_v6 (& self , _ : bool) -> io :: Result < () > { self . 0 } pub fn multicast_loop_v6 (& self) -> io :: Result < bool > { self . 0 } pub fn join_multicast_v4 (& self , _ : & Ipv4Addr , _ : & Ipv4Addr) -> io :: Result < () > { self . 0 } pub fn join_multicast_v6 (& self , _ : & Ipv6Addr , _ : u32) -> io :: Result < () > { self . 0 } pub fn leave_multicast_v4 (& self , _ : & Ipv4Addr , _ : & Ipv4Addr) -> io :: Result < () > { self . 0 } pub fn leave_multicast_v6 (& self , _ : & Ipv6Addr , _ : u32) -> io :: Result < () > { self . 0 } pub fn set_ttl (& self , _ : u32) -> io :: Result < () > { self . 0 } pub fn ttl (& self) -> io :: Result < u32 > { self . 0 } pub fn take_error (& self) -> io :: Result < Option < io :: Error > > { self . 0 } pub fn set_nonblocking (& self , _ : bool) -> io :: Result < () > { self . 0 } pub fn recv (& self , _ : & mut [u8]) -> io :: Result < usize > { self . 0 } pub fn peek (& self , _ : & mut [u8]) -> io :: Result < usize > { self . 0 } pub fn send (& self , _ : & [u8]) -> io :: Result < usize > { self . 0 } pub fn connect < A : ToSocketAddrs > (& self , _ : A) -> io :: Result < () > { self . 0 } }}}
mkitem!{mkimpl!{impl fmt :: Debug for UdpSocket { fn fmt (& self , _f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . 0 } }}}
mkitem!{mkstruct!{pub struct LookupHost (!) ;}}
mkitem!{mkimpl!{impl LookupHost { pub fn port (& self) -> u16 { self . 0 } }}}
mkitem!{mkimpl!{impl Iterator for LookupHost { type Item = SocketAddr ; fn next (& mut self) -> Option < SocketAddr > { self . 0 } }}}
mkitem!{mkimpl!{impl TryFrom < & str > for LookupHost { type Error = io :: Error ; fn try_from (_v : & str) -> io :: Result < LookupHost > { unsupported () } }}}
mkitem!{mkimpl!{impl < 'a > TryFrom < (& 'a str , u16) > for LookupHost { type Error = io :: Error ; fn try_from (_v : (& 'a str , u16)) -> io :: Result < LookupHost > { unsupported () } }}}