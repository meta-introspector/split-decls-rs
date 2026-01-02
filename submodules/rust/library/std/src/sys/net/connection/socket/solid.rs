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
mkuse!{use libc :: { c_int , c_void , size_t } ;}
mkuse!{use self :: netc :: { MSG_PEEK , sockaddr , socklen_t } ;}
mkuse!{use super :: { getsockopt , setsockopt , socket_addr_from_c , socket_addr_to_c } ;}
mkuse!{use crate :: ffi :: CStr ;}
mkuse!{use crate :: io :: { self , BorrowedBuf , BorrowedCursor , ErrorKind , IoSlice , IoSliceMut } ;}
mkuse!{use crate :: net :: { Shutdown , SocketAddr } ;}
mkuse!{use crate :: os :: solid :: io :: { AsFd , AsRawFd , BorrowedFd , FromRawFd , IntoRawFd , OwnedFd } ;}
mkuse!{use crate :: sys :: abi ;}
mkuse!{use crate :: sys_common :: { FromInner , IntoInner } ;}
mkuse!{use crate :: time :: Duration ;}
mkuse!{use crate :: { cmp , mem , ptr , str } ;}
mkmod!{netc, { 
                getname!(netc);
                getsrc!(netc);
                getpath!(netc);
                get_deps!(netc);
                get_crates!(netc);
                mkinclude!(netc);
                mkuse!{pub use crate :: sys :: abi :: sockets :: * ;} 
            }}
mkitem!{#[expect (non_camel_case_types)] pub type wrlen_t = size_t ;}

macro_rules! max_iov_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function max_iov in module {}", module_path!());
    };
}

mkfn!{
    max_iov_introspect!();
    const fn max_iov () -> usize { 1024 }
}
mkitem!{mktrait!{#[doc (hidden)] pub trait IsMinusOne { fn is_minus_one (& self) -> bool ; }}}
mkitem!{macro_rules ! impl_is_minus_one { ($ ($ t : ident) *) => ($ (impl IsMinusOne for $ t { fn is_minus_one (& self) -> bool { * self == - 1 } }) *) }}
mkitem!{impl_is_minus_one ! { i8 i16 i32 i64 isize }}

macro_rules! cvt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function cvt in module {}", module_path!());
    };
}

mkfn!{
    cvt_introspect!();
    pub fn cvt < T : IsMinusOne > (t : T) -> io :: Result < T > { if t . is_minus_one () { Err (last_error ()) } else { Ok (t) } }
}

macro_rules! cvt_gai_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function cvt_gai in module {}", module_path!());
    };
}

mkfn!{
    cvt_gai_introspect!();
    #[doc = " A variant of `cvt` for `getaddrinfo` which return 0 for a success."] pub fn cvt_gai (err : c_int) -> io :: Result < () > { if err == 0 { Ok (()) } else { let msg : & dyn crate :: fmt :: Display = match err { netc :: EAI_NONAME => & "name or service not known" , netc :: EAI_SERVICE => & "service not supported" , netc :: EAI_FAIL => & "non-recoverable failure in name resolution" , netc :: EAI_MEMORY => & "memory allocation failure" , netc :: EAI_FAMILY => & "family not supported" , _ => & err , } ; Err (io :: Error :: new (io :: ErrorKind :: Uncategorized , & format ! ("failed to lookup address information: {msg}") [..] ,)) } }
}

macro_rules! cvt_r_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function cvt_r in module {}", module_path!());
    };
}

mkfn!{
    cvt_r_introspect!();
    #[doc = " Just to provide the same interface as sys/pal/unix/net.rs"] pub fn cvt_r < T , F > (mut f : F) -> io :: Result < T > where T : IsMinusOne , F : FnMut () -> T , { cvt (f ()) }
}

macro_rules! last_error_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function last_error in module {}", module_path!());
    };
}

mkfn!{
    last_error_introspect!();
    #[doc = " Returns the last error from the network subsystem."] fn last_error () -> io :: Error { io :: Error :: from_raw_os_error (unsafe { netc :: SOLID_NET_GetLastError () }) }
}

macro_rules! error_name_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function error_name in module {}", module_path!());
    };
}

mkfn!{
    error_name_introspect!();
    pub fn error_name (er : abi :: ER) -> Option < & 'static str > { unsafe { CStr :: from_ptr (netc :: strerror (er)) } . to_str () . ok () }
}

macro_rules! is_interrupted_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_interrupted in module {}", module_path!());
    };
}

mkfn!{
    is_interrupted_introspect!();
    #[inline] pub fn is_interrupted (er : abi :: ER) -> bool { er == netc :: SOLID_NET_ERR_BASE - libc :: EINTR }
}

macro_rules! decode_error_kind_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function decode_error_kind in module {}", module_path!());
    };
}

mkfn!{
    decode_error_kind_introspect!();
    pub fn decode_error_kind (er : abi :: ER) -> ErrorKind { let errno = netc :: SOLID_NET_ERR_BASE - er ; match errno as libc :: c_int { libc :: ECONNREFUSED => ErrorKind :: ConnectionRefused , libc :: ECONNRESET => ErrorKind :: ConnectionReset , libc :: EPERM | libc :: EACCES => ErrorKind :: PermissionDenied , libc :: EPIPE => ErrorKind :: BrokenPipe , libc :: ENOTCONN => ErrorKind :: NotConnected , libc :: ECONNABORTED => ErrorKind :: ConnectionAborted , libc :: EADDRNOTAVAIL => ErrorKind :: AddrNotAvailable , libc :: EADDRINUSE => ErrorKind :: AddrInUse , libc :: ENOENT => ErrorKind :: NotFound , libc :: EINTR => ErrorKind :: Interrupted , libc :: EINVAL => ErrorKind :: InvalidInput , libc :: ETIMEDOUT => ErrorKind :: TimedOut , libc :: EEXIST => ErrorKind :: AlreadyExists , libc :: ENOSYS => ErrorKind :: Unsupported , libc :: ENOMEM => ErrorKind :: OutOfMemory , libc :: EAGAIN => ErrorKind :: WouldBlock , _ => ErrorKind :: Uncategorized , } }
}

macro_rules! init_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function init in module {}", module_path!());
    };
}

mkfn!{
    init_introspect!();
    pub fn init () { }
}
mkitem!{mkstruct!{pub struct Socket (OwnedFd) ;}}
mkitem!{mkimpl!{impl Socket { pub fn new (addr : & SocketAddr , ty : c_int) -> io :: Result < Socket > { let fam = match * addr { SocketAddr :: V4 (..) => netc :: AF_INET , SocketAddr :: V6 (..) => netc :: AF_INET6 , } ; Socket :: new_raw (fam , ty) } pub fn new_raw (fam : c_int , ty : c_int) -> io :: Result < Socket > { unsafe { let fd = cvt (netc :: socket (fam , ty , 0)) ? ; Ok (Self :: from_raw_fd (fd)) } } pub fn connect (& self , addr : & SocketAddr) -> io :: Result < () > { let (addr , len) = socket_addr_to_c (addr) ; cvt (unsafe { netc :: connect (self . as_raw_fd () , addr . as_ptr () , len) }) ? ; Ok (()) } pub fn connect_timeout (& self , addr : & SocketAddr , timeout : Duration) -> io :: Result < () > { self . set_nonblocking (true) ? ; let r = self . connect (addr) ; self . set_nonblocking (false) ? ; match r { Ok (_) => return Ok (()) , Err (ref e) if e . raw_os_error () == Some (netc :: EINPROGRESS) => { } Err (e) => return Err (e) , } if timeout . as_secs () == 0 && timeout . subsec_nanos () == 0 { return Err (io :: Error :: ZERO_TIMEOUT) ; } let mut timeout = netc :: timeval { tv_sec : timeout . as_secs () as _ , tv_usec : timeout . subsec_micros () as _ } ; if timeout . tv_sec == 0 && timeout . tv_usec == 0 { timeout . tv_usec = 1 ; } let fds = netc :: fd_set { num_fds : 1 , fds : [self . as_raw_fd ()] } ; let mut writefds = fds ; let mut errorfds = fds ; let n = unsafe { cvt (netc :: select (self . as_raw_fd () + 1 , ptr :: null_mut () , & mut writefds , & mut errorfds , & mut timeout ,)) ? } ; match n { 0 => Err (io :: const_error ! (io :: ErrorKind :: TimedOut , "connection timed out")) , _ => { let can_write = writefds . num_fds != 0 ; if ! can_write { if let Some (e) = self . take_error () ? { return Err (e) ; } } Ok (()) } } } pub fn accept (& self , storage : * mut sockaddr , len : * mut socklen_t) -> io :: Result < Socket > { let fd = cvt_r (| | unsafe { netc :: accept (self . as_raw_fd () , storage , len) }) ? ; unsafe { Ok (Self :: from_raw_fd (fd)) } } pub fn duplicate (& self) -> io :: Result < Socket > { Ok (Self (self . 0 . try_clone () ?)) } fn recv_with_flags (& self , mut buf : BorrowedCursor < '_ > , flags : c_int) -> io :: Result < () > { let ret = cvt (unsafe { netc :: recv (self . as_raw_fd () , buf . as_mut () . as_mut_ptr () . cast () , buf . capacity () , flags) }) ? ; unsafe { buf . advance_unchecked (ret as usize) ; } Ok (()) } pub fn read (& self , buf : & mut [u8]) -> io :: Result < usize > { let mut buf = BorrowedBuf :: from (buf) ; self . recv_with_flags (buf . unfilled () , 0) ? ; Ok (buf . len ()) } pub fn peek (& self , buf : & mut [u8]) -> io :: Result < usize > { let mut buf = BorrowedBuf :: from (buf) ; self . recv_with_flags (buf . unfilled () , MSG_PEEK) ? ; Ok (buf . len ()) } pub fn read_buf (& self , buf : BorrowedCursor < '_ >) -> io :: Result < () > { self . recv_with_flags (buf , 0) } pub fn read_vectored (& self , bufs : & mut [IoSliceMut < '_ >]) -> io :: Result < usize > { let ret = cvt (unsafe { netc :: readv (self . as_raw_fd () , bufs . as_ptr () as * const netc :: iovec , cmp :: min (bufs . len () , max_iov ()) as c_int ,) }) ? ; Ok (ret as usize) } #[inline] pub fn is_read_vectored (& self) -> bool { true } fn recv_from_with_flags (& self , buf : & mut [u8] , flags : c_int ,) -> io :: Result < (usize , SocketAddr) > { let mut storage : netc :: sockaddr_storage = unsafe { mem :: zeroed () } ; let mut addrlen = size_of_val (& storage) as netc :: socklen_t ; let n = cvt (unsafe { netc :: recvfrom (self . as_raw_fd () , buf . as_mut_ptr () as * mut c_void , buf . len () , flags , & mut storage as * mut _ as * mut _ , & mut addrlen ,) }) ? ; Ok ((n as usize , unsafe { socket_addr_from_c (& storage , addrlen as usize) ? })) } pub fn recv_from (& self , buf : & mut [u8]) -> io :: Result < (usize , SocketAddr) > { self . recv_from_with_flags (buf , 0) } pub fn peek_from (& self , buf : & mut [u8]) -> io :: Result < (usize , SocketAddr) > { self . recv_from_with_flags (buf , MSG_PEEK) } pub fn write_vectored (& self , bufs : & [IoSlice < '_ >]) -> io :: Result < usize > { let ret = cvt (unsafe { netc :: writev (self . as_raw_fd () , bufs . as_ptr () as * const netc :: iovec , cmp :: min (bufs . len () , max_iov ()) as c_int ,) }) ? ; Ok (ret as usize) } #[inline] pub fn is_write_vectored (& self) -> bool { true } pub fn set_timeout (& self , dur : Option < Duration > , kind : c_int) -> io :: Result < () > { let timeout = match dur { Some (dur) => { if dur . as_secs () == 0 && dur . subsec_nanos () == 0 { return Err (io :: Error :: ZERO_TIMEOUT) ; } let secs = if dur . as_secs () > netc :: c_long :: MAX as u64 { netc :: c_long :: MAX } else { dur . as_secs () as netc :: c_long } ; let mut timeout = netc :: timeval { tv_sec : secs , tv_usec : dur . subsec_micros () as _ } ; if timeout . tv_sec == 0 && timeout . tv_usec == 0 { timeout . tv_usec = 1 ; } timeout } None => netc :: timeval { tv_sec : 0 , tv_usec : 0 } , } ; setsockopt (self , netc :: SOL_SOCKET , kind , timeout) } pub fn timeout (& self , kind : c_int) -> io :: Result < Option < Duration > > { let raw : netc :: timeval = getsockopt (self , netc :: SOL_SOCKET , kind) ? ; if raw . tv_sec == 0 && raw . tv_usec == 0 { Ok (None) } else { let sec = raw . tv_sec as u64 ; let nsec = (raw . tv_usec as u32) * 1000 ; Ok (Some (Duration :: new (sec , nsec))) } } pub fn shutdown (& self , how : Shutdown) -> io :: Result < () > { let how = match how { Shutdown :: Write => netc :: SHUT_WR , Shutdown :: Read => netc :: SHUT_RD , Shutdown :: Both => netc :: SHUT_RDWR , } ; cvt (unsafe { netc :: shutdown (self . as_raw_fd () , how) }) ? ; Ok (()) } pub fn set_linger (& self , linger : Option < Duration >) -> io :: Result < () > { let linger = netc :: linger { l_onoff : linger . is_some () as netc :: c_int , l_linger : linger . unwrap_or_default () . as_secs () as netc :: c_int , } ; setsockopt (self , netc :: SOL_SOCKET , netc :: SO_LINGER , linger) } pub fn linger (& self) -> io :: Result < Option < Duration > > { let val : netc :: linger = getsockopt (self , netc :: SOL_SOCKET , netc :: SO_LINGER) ? ; Ok ((val . l_onoff != 0) . then (| | Duration :: from_secs (val . l_linger as u64))) } pub fn set_nodelay (& self , nodelay : bool) -> io :: Result < () > { setsockopt (self , netc :: IPPROTO_TCP , netc :: TCP_NODELAY , nodelay as c_int) } pub fn nodelay (& self) -> io :: Result < bool > { let raw : c_int = getsockopt (self , netc :: IPPROTO_TCP , netc :: TCP_NODELAY) ? ; Ok (raw != 0) } pub fn set_nonblocking (& self , nonblocking : bool) -> io :: Result < () > { let mut nonblocking = nonblocking as c_int ; cvt (unsafe { netc :: ioctl (self . as_raw_fd () , netc :: FIONBIO , (& mut nonblocking) as * mut c_int as _) }) . map (drop) } pub fn take_error (& self) -> io :: Result < Option < io :: Error > > { let raw : c_int = getsockopt (self , netc :: SOL_SOCKET , netc :: SO_ERROR) ? ; if raw == 0 { Ok (None) } else { Ok (Some (io :: Error :: from_raw_os_error (raw as i32))) } } pub fn as_raw (& self) -> c_int { self . as_raw_fd () } }}}
mkitem!{mkimpl!{impl FromInner < OwnedFd > for Socket { #[inline] fn from_inner (sock : OwnedFd) -> Socket { Socket (sock) } }}}
mkitem!{mkimpl!{impl IntoInner < OwnedFd > for Socket { #[inline] fn into_inner (self) -> OwnedFd { self . 0 } }}}
mkitem!{mkimpl!{impl AsFd for Socket { #[inline] fn as_fd (& self) -> BorrowedFd < '_ > { self . 0 . as_fd () } }}}
mkitem!{mkimpl!{impl AsRawFd for Socket { #[inline] fn as_raw_fd (& self) -> c_int { self . 0 . as_raw_fd () } }}}
mkitem!{mkimpl!{impl FromRawFd for Socket { #[inline] unsafe fn from_raw_fd (fd : c_int) -> Socket { unsafe { Self (FromRawFd :: from_raw_fd (fd)) } } }}}
mkitem!{mkimpl!{impl IntoRawFd for Socket { #[inline] fn into_raw_fd (self) -> c_int { self . 0 . into_raw_fd () } }}}