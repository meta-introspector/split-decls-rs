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
mkuse!{use crate :: io :: { self , BorrowedCursor , IoSlice , IoSliceMut } ;}
mkuse!{use crate :: ops :: Neg ;}
mkuse!{use crate :: os :: windows :: prelude :: * ;}
mkuse!{use crate :: sys :: handle :: Handle ;}
mkuse!{use crate :: sys :: { api , c } ;}
mkuse!{use crate :: sys_common :: { FromInner , IntoInner } ;}
mkuse!{use crate :: { mem , ptr } ;}
mkitem!{mkstruct!{pub struct AnonPipe { inner : Handle , }}}
mkitem!{mkimpl!{impl IntoInner < Handle > for AnonPipe { fn into_inner (self) -> Handle { self . inner } }}}
mkitem!{mkimpl!{impl FromInner < Handle > for AnonPipe { fn from_inner (inner : Handle) -> AnonPipe { Self { inner } } }}}
mkitem!{mkstruct!{pub struct Pipes { pub ours : AnonPipe , pub theirs : AnonPipe , }}}

macro_rules! anon_pipe_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function anon_pipe in module {}", module_path!());
    };
}

mkfn!{
    anon_pipe_introspect!();
    #[doc = " Although this looks similar to `anon_pipe` in the Unix module it's actually"] #[doc = " subtly different. Here we'll return two pipes in the `Pipes` return value,"] #[doc = " but one is intended for \"us\" where as the other is intended for \"someone"] #[doc = " else\"."] #[doc = ""] #[doc = " Currently the only use case for this function is pipes for stdio on"] #[doc = " processes in the standard library, so \"ours\" is the one that'll stay in our"] #[doc = " process whereas \"theirs\" will be inherited to a child."] #[doc = ""] #[doc = " The ours/theirs pipes are *not* specifically readable or writable. Each"] #[doc = " one only supports a read or a write, but which is which depends on the"] #[doc = " boolean flag given. If `ours_readable` is `true`, then `ours` is readable and"] #[doc = " `theirs` is writable. Conversely, if `ours_readable` is `false`, then `ours`"] #[doc = " is writable and `theirs` is readable."] #[doc = ""] #[doc = " Also note that the `ours` pipe is always a handle opened up in overlapped"] #[doc = " mode. This means that technically speaking it should only ever be used"] #[doc = " with `OVERLAPPED` instances, but also works out ok if it's only ever used"] #[doc = " once at a time (which we do indeed guarantee)."] pub fn anon_pipe (ours_readable : bool , their_handle_inheritable : bool) -> io :: Result < Pipes > { const PIPE_BUFFER_CAPACITY : u32 = 64 * 1024 ; unsafe { let mut io_status = c :: IO_STATUS_BLOCK :: default () ; let mut object_attributes = c :: OBJECT_ATTRIBUTES :: default () ; object_attributes . Length = size_of :: < c :: OBJECT_ATTRIBUTES > () as u32 ; let pipe_fs = { let path = api :: unicode_str ! (r"\??\PIPE\") ; object_attributes . ObjectName = path . as_ptr () ; let mut pipe_fs = ptr :: null_mut () ; let status = c :: NtOpenFile (& mut pipe_fs , c :: SYNCHRONIZE | c :: GENERIC_READ , & object_attributes , & mut io_status , c :: FILE_SHARE_READ | c :: FILE_SHARE_WRITE , c :: FILE_SYNCHRONOUS_IO_NONALERT ,) ; if c :: nt_success (status) { Handle :: from_raw_handle (pipe_fs) } else { return Err (io :: Error :: from_raw_os_error (c :: RtlNtStatusToDosError (status) as i32)) ; } } ; let empty = c :: UNICODE_STRING :: default () ; object_attributes . ObjectName = & raw const empty ; let ours = { object_attributes . RootDirectory = pipe_fs . as_raw_handle () ; let timeout = (50_i64 * 10000) . neg () as u64 ; let mut ours = ptr :: null_mut () ; let status = c :: NtCreateNamedPipeFile (& mut ours , c :: SYNCHRONIZE | if ours_readable { c :: GENERIC_READ } else { c :: GENERIC_WRITE } , & object_attributes , & mut io_status , if ours_readable { c :: FILE_SHARE_WRITE } else { c :: FILE_SHARE_READ } , c :: FILE_CREATE , 0 , c :: FILE_PIPE_BYTE_STREAM_TYPE , c :: FILE_PIPE_BYTE_STREAM_MODE , c :: FILE_PIPE_QUEUE_OPERATION , 1 , PIPE_BUFFER_CAPACITY , PIPE_BUFFER_CAPACITY , & timeout ,) ; if c :: nt_success (status) { Handle :: from_raw_handle (ours) } else { return Err (io :: Error :: from_raw_os_error (c :: RtlNtStatusToDosError (status) as i32)) ; } } ; let theirs = { object_attributes . RootDirectory = ours . as_raw_handle () ; if their_handle_inheritable { object_attributes . Attributes |= c :: OBJ_INHERIT ; } let mut theirs = ptr :: null_mut () ; let status = c :: NtOpenFile (& mut theirs , c :: SYNCHRONIZE | if ours_readable { c :: GENERIC_WRITE | c :: FILE_READ_ATTRIBUTES } else { c :: GENERIC_READ } , & object_attributes , & mut io_status , 0 , c :: FILE_NON_DIRECTORY_FILE | c :: FILE_SYNCHRONOUS_IO_NONALERT ,) ; if c :: nt_success (status) { Handle :: from_raw_handle (theirs) } else { return Err (io :: Error :: from_raw_os_error (c :: RtlNtStatusToDosError (status) as i32)) ; } } ; Ok (Pipes { ours : AnonPipe { inner : ours } , theirs : AnonPipe { inner : theirs } }) } }
}

macro_rules! spawn_pipe_relay_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function spawn_pipe_relay in module {}", module_path!());
    };
}

mkfn!{
    spawn_pipe_relay_introspect!();
    #[doc = " Takes an asynchronous source pipe and returns a synchronous pipe suitable"] #[doc = " for sending to a child process."] #[doc = ""] #[doc = " This is achieved by creating a new set of pipes and spawning a thread that"] #[doc = " relays messages between the source and the synchronous pipe."] pub fn spawn_pipe_relay (source : & AnonPipe , ours_readable : bool , their_handle_inheritable : bool ,) -> io :: Result < AnonPipe > { let source = source . try_clone () ? ; let Pipes { theirs , ours } = anon_pipe (ours_readable , their_handle_inheritable) ? ; let (reader , writer) = if ours_readable { (ours , source) } else { (source , ours) } ; crate :: thread :: spawn (move | | { let mut buf = [0_u8 ; 4096] ; 'reader : while let Ok (len) = reader . read (& mut buf) { if len == 0 { break ; } let mut start = 0 ; while let Ok (written) = writer . write (& buf [start .. len]) { start += written ; if start == len { continue 'reader ; } } break ; } }) ; Ok (theirs) }
}
mkitem!{mkimpl!{impl AnonPipe { pub fn handle (& self) -> & Handle { & self . inner } pub fn into_handle (self) -> Handle { self . inner } pub fn try_clone (& self) -> io :: Result < Self > { self . inner . duplicate (0 , false , c :: DUPLICATE_SAME_ACCESS) . map (| inner | AnonPipe { inner }) } pub fn read (& self , buf : & mut [u8]) -> io :: Result < usize > { let result = unsafe { let len = crate :: cmp :: min (buf . len () , u32 :: MAX as usize) as u32 ; let ptr = buf . as_mut_ptr () ; self . alertable_io_internal (| overlapped , callback | { c :: ReadFileEx (self . inner . as_raw_handle () , ptr , len , overlapped , callback) }) } ; match result { Err (ref e) if e . kind () == io :: ErrorKind :: BrokenPipe => Ok (0) , _ => result , } } pub fn read_buf (& self , mut buf : BorrowedCursor < '_ >) -> io :: Result < () > { let result = unsafe { let len = crate :: cmp :: min (buf . capacity () , u32 :: MAX as usize) as u32 ; let ptr = buf . as_mut () . as_mut_ptr () . cast :: < u8 > () ; self . alertable_io_internal (| overlapped , callback | { c :: ReadFileEx (self . inner . as_raw_handle () , ptr , len , overlapped , callback) }) } ; match result { Err (ref e) if e . kind () == io :: ErrorKind :: BrokenPipe => Ok (()) , Err (e) => Err (e) , Ok (n) => { unsafe { buf . advance_unchecked (n) ; } Ok (()) } } } pub fn read_vectored (& self , bufs : & mut [IoSliceMut < '_ >]) -> io :: Result < usize > { self . inner . read_vectored (bufs) } #[inline] pub fn is_read_vectored (& self) -> bool { self . inner . is_read_vectored () } pub fn read_to_end (& self , buf : & mut Vec < u8 >) -> io :: Result < usize > { self . handle () . read_to_end (buf) } pub fn write (& self , buf : & [u8]) -> io :: Result < usize > { unsafe { let len = crate :: cmp :: min (buf . len () , u32 :: MAX as usize) as u32 ; self . alertable_io_internal (| overlapped , callback | { c :: WriteFileEx (self . inner . as_raw_handle () , buf . as_ptr () , len , overlapped , callback) }) } } pub fn write_vectored (& self , bufs : & [IoSlice < '_ >]) -> io :: Result < usize > { self . inner . write_vectored (bufs) } #[inline] pub fn is_write_vectored (& self) -> bool { self . inner . is_write_vectored () } #[doc = " Synchronizes asynchronous reads or writes using our anonymous pipe."] #[doc = ""] #[doc = " This is a wrapper around [`ReadFileEx`] or [`WriteFileEx`] that uses"] #[doc = " [Asynchronous Procedure Call] (APC) to synchronize reads or writes."] #[doc = ""] #[doc = " Note: This should not be used for handles we don't create."] #[doc = ""] #[doc = " # Safety"] #[doc = ""] #[doc = " `buf` must be a pointer to a buffer that's valid for reads or writes"] #[doc = " up to `len` bytes. The `AlertableIoFn` must be either `ReadFileEx` or `WriteFileEx`"] #[doc = ""] #[doc = " [`ReadFileEx`]: https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-readfileex"] #[doc = " [`WriteFileEx`]: https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-writefileex"] #[doc = " [Asynchronous Procedure Call]: https://docs.microsoft.com/en-us/windows/win32/sync/asynchronous-procedure-calls"] unsafe fn alertable_io_internal (& self , io : impl FnOnce (& mut c :: OVERLAPPED , c :: LPOVERLAPPED_COMPLETION_ROUTINE) -> c :: BOOL ,) -> io :: Result < usize > { let mut async_result : Option < AsyncResult > = None ; struct AsyncResult { error : u32 , transferred : u32 , } unsafe extern "system" fn callback (dwErrorCode : u32 , dwNumberOfBytesTransferred : u32 , lpOverlapped : * mut c :: OVERLAPPED ,) { unsafe { let result = AsyncResult { error : dwErrorCode , transferred : dwNumberOfBytesTransferred } ; * (* lpOverlapped) . hEvent . cast :: < Option < AsyncResult > > () = Some (result) ; } } let mut overlapped : c :: OVERLAPPED = unsafe { crate :: mem :: zeroed () } ; overlapped . hEvent = (& raw mut async_result) as * mut _ ; let result = io (& mut overlapped , Some (callback)) ; if result == c :: FALSE { return Err (io :: Error :: last_os_error ()) ; } let result = loop { unsafe { c :: SleepEx (c :: INFINITE , c :: TRUE) } ; if let Some (result) = async_result { break result ; } } ; match result . error { c :: ERROR_SUCCESS => Ok (result . transferred as usize) , error => Err (io :: Error :: from_raw_os_error (error as _)) , } } }}}

macro_rules! read2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function read2 in module {}", module_path!());
    };
}

mkfn!{
    read2_introspect!();
    pub fn read2 (p1 : AnonPipe , v1 : & mut Vec < u8 > , p2 : AnonPipe , v2 : & mut Vec < u8 >) -> io :: Result < () > { let p1 = p1 . into_handle () ; let p2 = p2 . into_handle () ; let mut p1 = AsyncPipe :: new (p1 , v1) ? ; let mut p2 = AsyncPipe :: new (p2 , v2) ? ; let objs = [p1 . event . as_raw_handle () , p2 . event . as_raw_handle ()] ; loop { let res = unsafe { c :: WaitForMultipleObjects (2 , objs . as_ptr () , c :: FALSE , c :: INFINITE) } ; if res == c :: WAIT_OBJECT_0 { if ! p1 . result () ? || ! p1 . schedule_read () ? { return p2 . finish () ; } } else if res == c :: WAIT_OBJECT_0 + 1 { if ! p2 . result () ? || ! p2 . schedule_read () ? { return p1 . finish () ; } } else { return Err (io :: Error :: last_os_error ()) ; } } }
}
mkitem!{mkstruct!{struct AsyncPipe < 'a > { pipe : Handle , event : Handle , overlapped : Box < c :: OVERLAPPED > , dst : & 'a mut Vec < u8 > , state : State , }}}
mkitem!{mkenum!{#[derive (PartialEq , Debug)] enum State { NotReading , Reading , Read (usize) , }}}
mkitem!{mkimpl!{impl < 'a > AsyncPipe < 'a > { fn new (pipe : Handle , dst : & 'a mut Vec < u8 >) -> io :: Result < AsyncPipe < 'a > > { let event = Handle :: new_event (true , true) ? ; let mut overlapped : Box < c :: OVERLAPPED > = unsafe { Box :: new (mem :: zeroed ()) } ; overlapped . hEvent = event . as_raw_handle () ; Ok (AsyncPipe { pipe , overlapped , event , dst , state : State :: NotReading }) } #[doc = " Executes an overlapped read operation."] #[doc = ""] #[doc = " Must not currently be reading, and returns whether the pipe is currently"] #[doc = " at EOF or not. If the pipe is not at EOF then `result()` must be called"] #[doc = " to complete the read later on (may block), but if the pipe is at EOF"] #[doc = " then `result()` should not be called as it will just block forever."] fn schedule_read (& mut self) -> io :: Result < bool > { assert_eq ! (self . state , State :: NotReading) ; let amt = unsafe { if self . dst . capacity () == self . dst . len () { let additional = if self . dst . capacity () == 0 { 16 } else { 1 } ; self . dst . reserve (additional) ; } self . pipe . read_overlapped (self . dst . spare_capacity_mut () , & mut * self . overlapped) ? } ; self . state = match amt { Some (0) => return Ok (false) , Some (amt) => State :: Read (amt) , None => State :: Reading , } ; Ok (true) } #[doc = " Wait for the result of the overlapped operation previously executed."] #[doc = ""] #[doc = " Takes a parameter `wait` which indicates if this pipe is currently being"] #[doc = " read whether the function should block waiting for the read to complete."] #[doc = ""] #[doc = " Returns values:"] #[doc = ""] #[doc = " * `true` - finished any pending read and the pipe is not at EOF (keep"] #[doc = "            going)"] #[doc = " * `false` - finished any pending read and pipe is at EOF (stop issuing"] #[doc = "             reads)"] fn result (& mut self) -> io :: Result < bool > { let amt = match self . state { State :: NotReading => return Ok (true) , State :: Reading => self . pipe . overlapped_result (& mut * self . overlapped , true) ? , State :: Read (amt) => amt , } ; self . state = State :: NotReading ; unsafe { let len = self . dst . len () ; self . dst . set_len (len + amt) ; } Ok (amt != 0) } #[doc = " Finishes out reading this pipe entirely."] #[doc = ""] #[doc = " Waits for any pending and schedule read, and then calls `read_to_end`"] #[doc = " if necessary to read all the remaining information."] fn finish (& mut self) -> io :: Result < () > { while self . result () ? && self . schedule_read () ? { } Ok (()) } }}}
mkitem!{mkimpl!{impl < 'a > Drop for AsyncPipe < 'a > { fn drop (& mut self) { match self . state { State :: Reading => { } _ => return , } if self . pipe . cancel_io () . is_err () || self . result () . is_err () { let buf = mem :: take (self . dst) ; let overlapped = Box :: new (unsafe { mem :: zeroed () }) ; let overlapped = mem :: replace (& mut self . overlapped , overlapped) ; mem :: forget ((buf , overlapped)) ; } } }}}