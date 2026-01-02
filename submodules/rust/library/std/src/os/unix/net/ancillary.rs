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
mkuse!{use super :: { SocketAddr , sockaddr_un } ;}
mkuse!{use crate :: io :: { self , IoSlice , IoSliceMut } ;}
mkuse!{use crate :: marker :: PhantomData ;}
mkuse!{use crate :: mem :: zeroed ;}
mkuse!{use crate :: os :: unix :: io :: RawFd ;}
mkuse!{use crate :: path :: Path ;}
mkuse!{use crate :: ptr :: { eq , read_unaligned } ;}
mkuse!{use crate :: slice :: from_raw_parts ;}
mkuse!{use crate :: sys :: net :: Socket ;}
mkmod!{libc, { 
                getname!(libc);
                getsrc!(libc);
                getpath!(libc);
                get_deps!(libc);
                get_crates!(libc);
                mkinclude!(libc);
                mkuse!{pub use core :: ffi :: c_int ;}
mkitem!{mkstruct!{pub struct ucred ;}}
mkitem!{mkstruct!{pub struct cmsghdr ;}}
mkitem!{mkstruct!{pub struct sockcred2 ;}}
mkitem!{pub type pid_t = i32 ;}
mkitem!{pub type gid_t = u32 ;}
mkitem!{pub type uid_t = u32 ;} 
            }}

macro_rules! recv_vectored_with_ancillary_from_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function recv_vectored_with_ancillary_from in module {}", module_path!());
    };
}

mkfn!{
    recv_vectored_with_ancillary_from_introspect!();
    pub (super) fn recv_vectored_with_ancillary_from (socket : & Socket , bufs : & mut [IoSliceMut < '_ >] , ancillary : & mut SocketAncillary < '_ > ,) -> io :: Result < (usize , bool , io :: Result < SocketAddr >) > { unsafe { let mut msg_name : libc :: sockaddr_un = zeroed () ; let mut msg : libc :: msghdr = zeroed () ; msg . msg_name = (& raw mut msg_name) as * mut _ ; msg . msg_namelen = size_of :: < libc :: sockaddr_un > () as libc :: socklen_t ; msg . msg_iov = bufs . as_mut_ptr () . cast () ; msg . msg_iovlen = bufs . len () as _ ; msg . msg_controllen = ancillary . buffer . len () as _ ; if msg . msg_controllen > 0 { msg . msg_control = ancillary . buffer . as_mut_ptr () . cast () ; } let count = socket . recv_msg (& mut msg) ? ; ancillary . length = msg . msg_controllen as usize ; ancillary . truncated = msg . msg_flags & libc :: MSG_CTRUNC == libc :: MSG_CTRUNC ; let truncated = msg . msg_flags & libc :: MSG_TRUNC == libc :: MSG_TRUNC ; let addr = SocketAddr :: from_parts (msg_name , msg . msg_namelen) ; Ok ((count , truncated , addr)) } }
}

macro_rules! send_vectored_with_ancillary_to_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function send_vectored_with_ancillary_to in module {}", module_path!());
    };
}

mkfn!{
    send_vectored_with_ancillary_to_introspect!();
    pub (super) fn send_vectored_with_ancillary_to (socket : & Socket , path : Option < & Path > , bufs : & [IoSlice < '_ >] , ancillary : & mut SocketAncillary < '_ > ,) -> io :: Result < usize > { unsafe { let (mut msg_name , msg_namelen) = if let Some (path) = path { sockaddr_un (path) ? } else { (zeroed () , 0) } ; let mut msg : libc :: msghdr = zeroed () ; msg . msg_name = (& raw mut msg_name) as * mut _ ; msg . msg_namelen = msg_namelen ; msg . msg_iov = bufs . as_ptr () as * mut _ ; msg . msg_iovlen = bufs . len () as _ ; msg . msg_controllen = ancillary . length as _ ; if msg . msg_controllen > 0 { msg . msg_control = ancillary . buffer . as_mut_ptr () . cast () ; } ancillary . truncated = false ; socket . send_msg (& mut msg) } }
}

macro_rules! add_to_ancillary_data_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function add_to_ancillary_data in module {}", module_path!());
    };
}

mkfn!{
    add_to_ancillary_data_introspect!();
    fn add_to_ancillary_data < T > (buffer : & mut [u8] , length : & mut usize , source : & [T] , cmsg_level : libc :: c_int , cmsg_type : libc :: c_int ,) -> bool { # [cfg (not (target_os = "freebsd"))] let cmsg_size = source . len () . checked_mul (size_of :: < T > ()) ; # [cfg (target_os = "freebsd")] let cmsg_size = Some (unsafe { libc :: SOCKCRED2SIZE (1) }) ; let source_len = if let Some (source_len) = cmsg_size { if let Ok (source_len) = u32 :: try_from (source_len) { source_len } else { return false ; } } else { return false ; } ; unsafe { let additional_space = libc :: CMSG_SPACE (source_len) as usize ; let new_length = if let Some (new_length) = additional_space . checked_add (* length) { new_length } else { return false ; } ; if new_length > buffer . len () { return false ; } buffer [* length .. new_length] . fill (0) ; * length = new_length ; let mut msg : libc :: msghdr = zeroed () ; msg . msg_control = buffer . as_mut_ptr () . cast () ; msg . msg_controllen = * length as _ ; let mut cmsg = libc :: CMSG_FIRSTHDR (& msg) ; let mut previous_cmsg = cmsg ; while ! cmsg . is_null () { previous_cmsg = cmsg ; cmsg = libc :: CMSG_NXTHDR (& msg , cmsg) ; if eq (cmsg , previous_cmsg) { break ; } } if previous_cmsg . is_null () { return false ; } (* previous_cmsg) . cmsg_level = cmsg_level ; (* previous_cmsg) . cmsg_type = cmsg_type ; (* previous_cmsg) . cmsg_len = libc :: CMSG_LEN (source_len) as _ ; let data = libc :: CMSG_DATA (previous_cmsg) . cast () ; libc :: memcpy (data , source . as_ptr () . cast () , source_len as usize) ; } true }
}
mkitem!{mkstruct!{struct AncillaryDataIter < 'a , T > { data : & 'a [u8] , phantom : PhantomData < T > , }}}
mkitem!{mkimpl!{impl < 'a , T > AncillaryDataIter < 'a , T > { # [doc = " Creates `AncillaryDataIter` struct to iterate through the data unit in the control message."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `data` must contain a valid control message."] unsafe fn new (data : & 'a [u8]) -> AncillaryDataIter < 'a , T > { AncillaryDataIter { data , phantom : PhantomData } } }}}
mkitem!{mkimpl!{impl < 'a , T > Iterator for AncillaryDataIter < 'a , T > { type Item = T ; fn next (& mut self) -> Option < T > { if size_of :: < T > () <= self . data . len () { unsafe { let unit = read_unaligned (self . data . as_ptr () . cast ()) ; self . data = & self . data [size_of :: < T > () ..] ; Some (unit) } } else { None } } }}}
mkitem!{mkstruct!{# [cfg (all (doc , not (target_os = "android") , not (target_os = "linux") , not (target_os = "netbsd") , not (target_os = "freebsd") , not (target_os = "cygwin") ,))] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] # [derive (Clone)] pub struct SocketCred (()) ;}}
mkitem!{mkstruct!{# [doc = " Unix credential."] # [cfg (any (target_os = "android" , target_os = "linux" , target_os = "cygwin"))] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] # [derive (Clone)] pub struct SocketCred (libc :: ucred) ;}}
mkitem!{mkstruct!{# [cfg (target_os = "netbsd")] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] # [derive (Clone)] pub struct SocketCred (libc :: sockcred) ;}}
mkitem!{mkstruct!{# [cfg (target_os = "freebsd")] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] # [derive (Clone)] pub struct SocketCred (libc :: sockcred2) ;}}
mkitem!{mkimpl!{# [doc (cfg (any (target_os = "android" , target_os = "linux" , target_os = "cygwin")))] # [cfg (any (target_os = "android" , target_os = "linux" , target_os = "cygwin"))] impl SocketCred { # [doc = " Creates a Unix credential struct."] # [doc = ""] # [doc = " PID, UID and GID is set to 0."] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] # [must_use] pub fn new () -> SocketCred { SocketCred (libc :: ucred { pid : 0 , uid : 0 , gid : 0 }) } # [doc = " Set the PID."] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] pub fn set_pid (& mut self , pid : libc :: pid_t) { self . 0 . pid = pid ; } # [doc = " Gets the current PID."] # [must_use] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] pub fn get_pid (& self) -> libc :: pid_t { self . 0 . pid } # [doc = " Set the UID."] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] pub fn set_uid (& mut self , uid : libc :: uid_t) { self . 0 . uid = uid ; } # [doc = " Gets the current UID."] # [must_use] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] pub fn get_uid (& self) -> libc :: uid_t { self . 0 . uid } # [doc = " Set the GID."] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] pub fn set_gid (& mut self , gid : libc :: gid_t) { self . 0 . gid = gid ; } # [doc = " Gets the current GID."] # [must_use] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] pub fn get_gid (& self) -> libc :: gid_t { self . 0 . gid } }}}
mkitem!{mkimpl!{# [cfg (target_os = "freebsd")] impl SocketCred { # [doc = " Creates a Unix credential struct."] # [doc = ""] # [doc = " PID, UID and GID is set to 0."] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] # [must_use] pub fn new () -> SocketCred { SocketCred (libc :: sockcred2 { sc_version : 0 , sc_pid : 0 , sc_uid : 0 , sc_euid : 0 , sc_gid : 0 , sc_egid : 0 , sc_ngroups : 0 , sc_groups : [0 ; 1] , }) } # [doc = " Set the PID."] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] pub fn set_pid (& mut self , pid : libc :: pid_t) { self . 0 . sc_pid = pid ; } # [doc = " Gets the current PID."] # [must_use] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] pub fn get_pid (& self) -> libc :: pid_t { self . 0 . sc_pid } # [doc = " Set the UID."] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] pub fn set_uid (& mut self , uid : libc :: uid_t) { self . 0 . sc_euid = uid ; } # [doc = " Gets the current UID."] # [must_use] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] pub fn get_uid (& self) -> libc :: uid_t { self . 0 . sc_euid } # [doc = " Set the GID."] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] pub fn set_gid (& mut self , gid : libc :: gid_t) { self . 0 . sc_egid = gid ; } # [doc = " Gets the current GID."] # [must_use] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] pub fn get_gid (& self) -> libc :: gid_t { self . 0 . sc_egid } }}}
mkitem!{mkimpl!{# [cfg (target_os = "netbsd")] impl SocketCred { # [doc = " Creates a Unix credential struct."] # [doc = ""] # [doc = " PID, UID and GID is set to 0."] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] pub fn new () -> SocketCred { SocketCred (libc :: sockcred { sc_pid : 0 , sc_uid : 0 , sc_euid : 0 , sc_gid : 0 , sc_egid : 0 , sc_ngroups : 0 , sc_groups : [0u32 ; 1] , }) } # [doc = " Set the PID."] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] pub fn set_pid (& mut self , pid : libc :: pid_t) { self . 0 . sc_pid = pid ; } # [doc = " Gets the current PID."] # [must_use] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] pub fn get_pid (& self) -> libc :: pid_t { self . 0 . sc_pid } # [doc = " Set the UID."] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] pub fn set_uid (& mut self , uid : libc :: uid_t) { self . 0 . sc_uid = uid ; } # [doc = " Gets the current UID."] # [must_use] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] pub fn get_uid (& self) -> libc :: uid_t { self . 0 . sc_uid } # [doc = " Set the GID."] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] pub fn set_gid (& mut self , gid : libc :: gid_t) { self . 0 . sc_gid = gid ; } # [doc = " Gets the current GID."] # [must_use] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] pub fn get_gid (& self) -> libc :: gid_t { self . 0 . sc_gid } }}}
mkitem!{mkstruct!{# [doc = " This control message contains file descriptors."] # [doc = ""] # [doc = " The level is equal to `SOL_SOCKET` and the type is equal to `SCM_RIGHTS`."] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] pub struct ScmRights < 'a > (AncillaryDataIter < 'a , RawFd >) ;}}
mkitem!{mkimpl!{# [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] impl < 'a > Iterator for ScmRights < 'a > { type Item = RawFd ; fn next (& mut self) -> Option < RawFd > { self . 0 . next () } }}}
mkitem!{mkstruct!{# [cfg (all (doc , not (target_os = "android") , not (target_os = "linux") , not (target_os = "netbsd") , not (target_os = "freebsd") , not (target_os = "cygwin") ,))] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] pub struct ScmCredentials < 'a > (AncillaryDataIter < 'a , () >) ;}}
mkitem!{mkstruct!{# [doc = " This control message contains unix credentials."] # [doc = ""] # [doc = " The level is equal to `SOL_SOCKET` and the type is equal to `SCM_CREDENTIALS` or `SCM_CREDS`."] # [cfg (any (target_os = "android" , target_os = "linux" , target_os = "cygwin"))] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] pub struct ScmCredentials < 'a > (AncillaryDataIter < 'a , libc :: ucred >) ;}}
mkitem!{mkstruct!{# [cfg (target_os = "freebsd")] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] pub struct ScmCredentials < 'a > (AncillaryDataIter < 'a , libc :: sockcred2 >) ;}}
mkitem!{mkstruct!{# [cfg (target_os = "netbsd")] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] pub struct ScmCredentials < 'a > (AncillaryDataIter < 'a , libc :: sockcred >) ;}}
mkitem!{mkimpl!{# [cfg (any (doc , target_os = "android" , target_os = "linux" , target_os = "netbsd" , target_os = "freebsd" , target_os = "cygwin" ,))] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] impl < 'a > Iterator for ScmCredentials < 'a > { type Item = SocketCred ; fn next (& mut self) -> Option < SocketCred > { Some (SocketCred (self . 0 . next () ?)) } }}}
mkitem!{mkenum!{# [doc = " The error type which is returned from parsing the type a control message."] # [non_exhaustive] # [derive (Debug)] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] pub enum AncillaryError { Unknown { cmsg_level : i32 , cmsg_type : i32 } , }}}
mkitem!{mkenum!{# [doc = " This enum represent one control message of variable type."] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] pub enum AncillaryData < 'a > { ScmRights (ScmRights < 'a >) , # [cfg (any (doc , target_os = "android" , target_os = "linux" , target_os = "netbsd" , target_os = "freebsd" , target_os = "cygwin" ,))] ScmCredentials (ScmCredentials < 'a >) , }}}
mkitem!{mkimpl!{impl < 'a > AncillaryData < 'a > { # [doc = " Creates an `AncillaryData::ScmRights` variant."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `data` must contain a valid control message and the control message must be type of"] # [doc = " `SOL_SOCKET` and level of `SCM_RIGHTS`."] unsafe fn as_rights (data : & 'a [u8]) -> Self { let ancillary_data_iter = AncillaryDataIter :: new (data) ; let scm_rights = ScmRights (ancillary_data_iter) ; AncillaryData :: ScmRights (scm_rights) } # [doc = " Creates an `AncillaryData::ScmCredentials` variant."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `data` must contain a valid control message and the control message must be type of"] # [doc = " `SOL_SOCKET` and level of `SCM_CREDENTIALS` or `SCM_CREDS`."] # [cfg (any (doc , target_os = "android" , target_os = "linux" , target_os = "netbsd" , target_os = "freebsd" , target_os = "cygwin" ,))] unsafe fn as_credentials (data : & 'a [u8]) -> Self { let ancillary_data_iter = AncillaryDataIter :: new (data) ; let scm_credentials = ScmCredentials (ancillary_data_iter) ; AncillaryData :: ScmCredentials (scm_credentials) } fn try_from_cmsghdr (cmsg : & 'a libc :: cmsghdr) -> Result < Self , AncillaryError > { unsafe { let cmsg_len_zero = libc :: CMSG_LEN (0) as usize ; let data_len = (* cmsg) . cmsg_len as usize - cmsg_len_zero ; let data = libc :: CMSG_DATA (cmsg) . cast () ; let data = from_raw_parts (data , data_len) ; match (* cmsg) . cmsg_level { libc :: SOL_SOCKET => match (* cmsg) . cmsg_type { libc :: SCM_RIGHTS => Ok (AncillaryData :: as_rights (data)) , # [cfg (any (target_os = "android" , target_os = "linux" , target_os = "cygwin"))] libc :: SCM_CREDENTIALS => Ok (AncillaryData :: as_credentials (data)) , # [cfg (target_os = "freebsd")] libc :: SCM_CREDS2 => Ok (AncillaryData :: as_credentials (data)) , # [cfg (target_os = "netbsd")] libc :: SCM_CREDS => Ok (AncillaryData :: as_credentials (data)) , cmsg_type => { Err (AncillaryError :: Unknown { cmsg_level : libc :: SOL_SOCKET , cmsg_type }) } } , cmsg_level => { Err (AncillaryError :: Unknown { cmsg_level , cmsg_type : (* cmsg) . cmsg_type }) } } } } }}}
mkitem!{mkstruct!{# [doc = " This struct is used to iterate through the control messages."] # [must_use = "iterators are lazy and do nothing unless consumed"] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] pub struct Messages < 'a > { buffer : & 'a [u8] , current : Option < & 'a libc :: cmsghdr > , }}}
mkitem!{mkimpl!{# [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] impl < 'a > Iterator for Messages < 'a > { type Item = Result < AncillaryData < 'a > , AncillaryError > ; fn next (& mut self) -> Option < Self :: Item > { unsafe { let mut msg : libc :: msghdr = zeroed () ; msg . msg_control = self . buffer . as_ptr () as * mut _ ; msg . msg_controllen = self . buffer . len () as _ ; let cmsg = if let Some (current) = self . current { libc :: CMSG_NXTHDR (& msg , current) } else { libc :: CMSG_FIRSTHDR (& msg) } ; let cmsg = cmsg . as_ref () ? ; if let Some (current) = self . current { if eq (current , cmsg) { return None ; } } self . current = Some (cmsg) ; let ancillary_result = AncillaryData :: try_from_cmsghdr (cmsg) ; Some (ancillary_result) } } }}}
mkitem!{mkstruct!{# [doc = " A Unix socket Ancillary data struct."] # [doc = ""] # [doc = " # Example"] # [doc = " ```no_run"] # [doc = " #![feature(unix_socket_ancillary_data)]"] # [doc = " use std::os::unix::net::{UnixStream, SocketAncillary, AncillaryData};"] # [doc = " use std::io::IoSliceMut;"] # [doc = ""] # [doc = " fn main() -> std::io::Result<()> {"] # [doc = "     let sock = UnixStream::connect(\"/tmp/sock\")?;"] # [doc = ""] # [doc = "     let mut fds = [0; 8];"] # [doc = "     let mut ancillary_buffer = [0; 128];"] # [doc = "     let mut ancillary = SocketAncillary::new(&mut ancillary_buffer[..]);"] # [doc = ""] # [doc = "     let mut buf = [1; 8];"] # [doc = "     let mut bufs = &mut [IoSliceMut::new(&mut buf[..])][..];"] # [doc = "     sock.recv_vectored_with_ancillary(bufs, &mut ancillary)?;"] # [doc = ""] # [doc = "     for ancillary_result in ancillary.messages() {"] # [doc = "         if let AncillaryData::ScmRights(scm_rights) = ancillary_result.unwrap() {"] # [doc = "             for fd in scm_rights {"] # [doc = "                 println!(\"receive file descriptor: {fd}\");"] # [doc = "             }"] # [doc = "         }"] # [doc = "     }"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] # [derive (Debug)] pub struct SocketAncillary < 'a > { buffer : & 'a mut [u8] , length : usize , truncated : bool , }}}
mkitem!{mkimpl!{impl < 'a > SocketAncillary < 'a > { # [doc = " Creates an ancillary data with the given buffer."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # #![allow(unused_mut)]"] # [doc = " #![feature(unix_socket_ancillary_data)]"] # [doc = " use std::os::unix::net::SocketAncillary;"] # [doc = " let mut ancillary_buffer = [0; 128];"] # [doc = " let mut ancillary = SocketAncillary::new(&mut ancillary_buffer[..]);"] # [doc = " ```"] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] pub fn new (buffer : & 'a mut [u8]) -> Self { SocketAncillary { buffer , length : 0 , truncated : false } } # [doc = " Returns the capacity of the buffer."] # [must_use] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] pub fn capacity (& self) -> usize { self . buffer . len () } # [doc = " Returns `true` if the ancillary data is empty."] # [must_use] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] pub fn is_empty (& self) -> bool { self . length == 0 } # [doc = " Returns the number of used bytes."] # [must_use] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] pub fn len (& self) -> usize { self . length } # [doc = " Returns the iterator of the control messages."] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] pub fn messages (& self) -> Messages < '_ > { Messages { buffer : & self . buffer [.. self . length] , current : None } } # [doc = " Is `true` if during a recv operation the ancillary was truncated."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```no_run"] # [doc = " #![feature(unix_socket_ancillary_data)]"] # [doc = " use std::os::unix::net::{UnixStream, SocketAncillary};"] # [doc = " use std::io::IoSliceMut;"] # [doc = ""] # [doc = " fn main() -> std::io::Result<()> {"] # [doc = "     let sock = UnixStream::connect(\"/tmp/sock\")?;"] # [doc = ""] # [doc = "     let mut ancillary_buffer = [0; 128];"] # [doc = "     let mut ancillary = SocketAncillary::new(&mut ancillary_buffer[..]);"] # [doc = ""] # [doc = "     let mut buf = [1; 8];"] # [doc = "     let mut bufs = &mut [IoSliceMut::new(&mut buf[..])][..];"] # [doc = "     sock.recv_vectored_with_ancillary(bufs, &mut ancillary)?;"] # [doc = ""] # [doc = "     println!(\"Is truncated: {}\", ancillary.truncated());"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [must_use] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] pub fn truncated (& self) -> bool { self . truncated } # [doc = " Add file descriptors to the ancillary data."] # [doc = ""] # [doc = " The function returns `true` if there was enough space in the buffer."] # [doc = " If there was not enough space then no file descriptors was appended."] # [doc = " Technically, that means this operation adds a control message with the level `SOL_SOCKET`"] # [doc = " and type `SCM_RIGHTS`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```no_run"] # [doc = " #![feature(unix_socket_ancillary_data)]"] # [doc = " use std::os::unix::net::{UnixStream, SocketAncillary};"] # [doc = " use std::os::unix::io::AsRawFd;"] # [doc = " use std::io::IoSlice;"] # [doc = ""] # [doc = " fn main() -> std::io::Result<()> {"] # [doc = "     let sock = UnixStream::connect(\"/tmp/sock\")?;"] # [doc = ""] # [doc = "     let mut ancillary_buffer = [0; 128];"] # [doc = "     let mut ancillary = SocketAncillary::new(&mut ancillary_buffer[..]);"] # [doc = "     ancillary.add_fds(&[sock.as_raw_fd()][..]);"] # [doc = ""] # [doc = "     let buf = [1; 8];"] # [doc = "     let mut bufs = &mut [IoSlice::new(&buf[..])][..];"] # [doc = "     sock.send_vectored_with_ancillary(bufs, &mut ancillary)?;"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] pub fn add_fds (& mut self , fds : & [RawFd]) -> bool { self . truncated = false ; add_to_ancillary_data (& mut self . buffer , & mut self . length , fds , libc :: SOL_SOCKET , libc :: SCM_RIGHTS ,) } # [doc = " Add credentials to the ancillary data."] # [doc = ""] # [doc = " The function returns `true` if there is enough space in the buffer."] # [doc = " If there is not enough space then no credentials will be appended."] # [doc = " Technically, that means this operation adds a control message with the level `SOL_SOCKET`"] # [doc = " and type `SCM_CREDENTIALS`, `SCM_CREDS`, or `SCM_CREDS2`."] # [doc = ""] # [cfg (any (doc , target_os = "android" , target_os = "linux" , target_os = "netbsd" , target_os = "freebsd" , target_os = "cygwin" ,))] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] pub fn add_creds (& mut self , creds : & [SocketCred]) -> bool { self . truncated = false ; add_to_ancillary_data (& mut self . buffer , & mut self . length , creds , libc :: SOL_SOCKET , # [cfg (not (any (target_os = "netbsd" , target_os = "freebsd")))] libc :: SCM_CREDENTIALS , # [cfg (target_os = "freebsd")] libc :: SCM_CREDS2 , # [cfg (target_os = "netbsd")] libc :: SCM_CREDS ,) } # [doc = " Clears the ancillary data, removing all values."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```no_run"] # [doc = " #![feature(unix_socket_ancillary_data)]"] # [doc = " use std::os::unix::net::{UnixStream, SocketAncillary, AncillaryData};"] # [doc = " use std::io::IoSliceMut;"] # [doc = ""] # [doc = " fn main() -> std::io::Result<()> {"] # [doc = "     let sock = UnixStream::connect(\"/tmp/sock\")?;"] # [doc = ""] # [doc = "     let mut fds1 = [0; 8];"] # [doc = "     let mut fds2 = [0; 8];"] # [doc = "     let mut ancillary_buffer = [0; 128];"] # [doc = "     let mut ancillary = SocketAncillary::new(&mut ancillary_buffer[..]);"] # [doc = ""] # [doc = "     let mut buf = [1; 8];"] # [doc = "     let mut bufs = &mut [IoSliceMut::new(&mut buf[..])][..];"] # [doc = ""] # [doc = "     sock.recv_vectored_with_ancillary(bufs, &mut ancillary)?;"] # [doc = "     for ancillary_result in ancillary.messages() {"] # [doc = "         if let AncillaryData::ScmRights(scm_rights) = ancillary_result.unwrap() {"] # [doc = "             for fd in scm_rights {"] # [doc = "                 println!(\"receive file descriptor: {fd}\");"] # [doc = "             }"] # [doc = "         }"] # [doc = "     }"] # [doc = ""] # [doc = "     ancillary.clear();"] # [doc = ""] # [doc = "     sock.recv_vectored_with_ancillary(bufs, &mut ancillary)?;"] # [doc = "     for ancillary_result in ancillary.messages() {"] # [doc = "         if let AncillaryData::ScmRights(scm_rights) = ancillary_result.unwrap() {"] # [doc = "             for fd in scm_rights {"] # [doc = "                 println!(\"receive file descriptor: {fd}\");"] # [doc = "             }"] # [doc = "         }"] # [doc = "     }"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] pub fn clear (& mut self) { self . length = 0 ; self . truncated = false ; } }}}