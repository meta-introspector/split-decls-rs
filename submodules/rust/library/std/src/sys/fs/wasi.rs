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
mkuse!{use crate :: ffi :: { CStr , OsStr , OsString } ;}
mkuse!{use crate :: fs :: TryLockError ;}
mkuse!{use crate :: io :: { self , BorrowedCursor , IoSlice , IoSliceMut , SeekFrom } ;}
mkuse!{use crate :: mem :: { self , ManuallyDrop } ;}
mkuse!{use crate :: os :: raw :: c_int ;}
mkuse!{use crate :: os :: wasi :: ffi :: { OsStrExt , OsStringExt } ;}
mkuse!{use crate :: os :: wasi :: io :: { AsFd , AsRawFd , BorrowedFd , FromRawFd , IntoRawFd , RawFd } ;}
mkuse!{use crate :: path :: { Path , PathBuf } ;}
mkuse!{use crate :: sync :: Arc ;}
mkuse!{use crate :: sys :: common :: small_c_string :: run_path_with_cstr ;}
mkuse!{use crate :: sys :: fd :: WasiFd ;}
mkuse!{pub use crate :: sys :: fs :: common :: exists ;}
mkuse!{use crate :: sys :: time :: SystemTime ;}
mkuse!{use crate :: sys :: { unsupported , unsupported_err } ;}
mkuse!{use crate :: sys_common :: { AsInner , FromInner , IntoInner , ignore_notfound } ;}
mkuse!{use crate :: { fmt , iter , ptr } ;}
mkitem!{mkstruct!{pub struct File { fd : WasiFd , }}}
mkitem!{mkstruct!{# [derive (Clone)] pub struct FileAttr { meta : wasi :: Filestat , }}}
mkitem!{mkstruct!{pub struct ReadDir { inner : Arc < ReadDirInner > , state : ReadDirState , }}}
mkitem!{mkenum!{enum ReadDirState { # [doc = " Fill `buf` with `buf.len()` bytes starting from `next_read_offset`."] FillBuffer { next_read_offset : wasi :: Dircookie , buf : Vec < u8 > , } , ProcessEntry { buf : Vec < u8 > , next_read_offset : Option < wasi :: Dircookie > , offset : usize , } , # [doc = " There is no more data to get in [`Self::FillBuffer`]; keep returning"] # [doc = " entries via ProcessEntry until `buf` is exhausted."] RunUntilExhaustion { buf : Vec < u8 > , offset : usize , } , Done , }}}
mkitem!{mkstruct!{struct ReadDirInner { root : PathBuf , dir : File , }}}
mkitem!{mkstruct!{pub struct DirEntry { meta : wasi :: Dirent , name : Vec < u8 > , inner : Arc < ReadDirInner > , }}}
mkitem!{mkstruct!{# [derive (Clone , Debug , Default)] pub struct OpenOptions { read : bool , write : bool , append : bool , dirflags : wasi :: Lookupflags , fdflags : wasi :: Fdflags , oflags : wasi :: Oflags , rights_base : Option < wasi :: Rights > , rights_inheriting : Option < wasi :: Rights > , }}}
mkitem!{mkstruct!{# [derive (Clone , PartialEq , Eq , Debug)] pub struct FilePermissions { readonly : bool , }}}
mkitem!{mkstruct!{# [derive (Copy , Clone , Debug , Default)] pub struct FileTimes { accessed : Option < SystemTime > , modified : Option < SystemTime > , }}}
mkitem!{mkstruct!{# [derive (PartialEq , Eq , Hash , Debug , Copy , Clone)] pub struct FileType { bits : wasi :: Filetype , }}}
mkitem!{mkstruct!{# [derive (Debug)] pub struct DirBuilder { }}}
mkitem!{mkimpl!{impl FileAttr { pub fn size (& self) -> u64 { self . meta . size } pub fn perm (& self) -> FilePermissions { FilePermissions { readonly : false } } pub fn file_type (& self) -> FileType { FileType { bits : self . meta . filetype } } pub fn modified (& self) -> io :: Result < SystemTime > { Ok (SystemTime :: from_wasi_timestamp (self . meta . mtim)) } pub fn accessed (& self) -> io :: Result < SystemTime > { Ok (SystemTime :: from_wasi_timestamp (self . meta . atim)) } pub fn created (& self) -> io :: Result < SystemTime > { Ok (SystemTime :: from_wasi_timestamp (self . meta . ctim)) } pub (crate) fn as_wasi (& self) -> & wasi :: Filestat { & self . meta } }}}
mkitem!{mkimpl!{impl FilePermissions { pub fn readonly (& self) -> bool { self . readonly } pub fn set_readonly (& mut self , readonly : bool) { self . readonly = readonly ; } }}}
mkitem!{mkimpl!{impl FileTimes { pub fn set_accessed (& mut self , t : SystemTime) { self . accessed = Some (t) ; } pub fn set_modified (& mut self , t : SystemTime) { self . modified = Some (t) ; } }}}
mkitem!{mkimpl!{impl FileType { pub fn is_dir (& self) -> bool { self . bits == wasi :: FILETYPE_DIRECTORY } pub fn is_file (& self) -> bool { self . bits == wasi :: FILETYPE_REGULAR_FILE } pub fn is_symlink (& self) -> bool { self . bits == wasi :: FILETYPE_SYMBOLIC_LINK } pub (crate) fn bits (& self) -> wasi :: Filetype { self . bits } }}}
mkitem!{mkimpl!{impl ReadDir { fn new (dir : File , root : PathBuf) -> ReadDir { ReadDir { inner : Arc :: new (ReadDirInner { dir , root }) , state : ReadDirState :: FillBuffer { next_read_offset : 0 , buf : vec ! [0 ; 128] } , } } }}}
mkitem!{mkimpl!{impl fmt :: Debug for ReadDir { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("ReadDir") . finish_non_exhaustive () } }}}
mkitem!{mkimpl!{impl core :: iter :: FusedIterator for ReadDir { }}}
mkitem!{mkimpl!{impl Iterator for ReadDir { type Item = io :: Result < DirEntry > ; fn next (& mut self) -> Option < io :: Result < DirEntry > > { match & mut self . state { ReadDirState :: FillBuffer { next_read_offset , buf } => { let result = self . inner . dir . fd . readdir (buf , * next_read_offset) ; match result { Ok (read_bytes) => { if read_bytes < buf . len () { buf . truncate (read_bytes) ; self . state = ReadDirState :: RunUntilExhaustion { buf : mem :: take (buf) , offset : 0 } ; } else { debug_assert_eq ! (read_bytes , buf . len ()) ; self . state = ReadDirState :: ProcessEntry { buf : mem :: take (buf) , offset : 0 , next_read_offset : Some (* next_read_offset) , } ; } self . next () } Err (e) => { self . state = ReadDirState :: Done ; return Some (Err (e)) ; } } } ReadDirState :: ProcessEntry { buf , next_read_offset , offset } => { let contents = & buf [* offset ..] ; const DIRENT_SIZE : usize = size_of :: < wasi :: Dirent > () ; if contents . len () >= DIRENT_SIZE { let (dirent , data) = contents . split_at (DIRENT_SIZE) ; let dirent = unsafe { ptr :: read_unaligned (dirent . as_ptr () as * const wasi :: Dirent) } ; if data . len () < dirent . d_namlen as usize { if buf . len () < dirent . d_namlen as usize + DIRENT_SIZE { buf . resize (dirent . d_namlen as usize + DIRENT_SIZE , 0) ; } if let Some (next_read_offset) = * next_read_offset { self . state = ReadDirState :: FillBuffer { next_read_offset , buf : mem :: take (buf) } ; } else { self . state = ReadDirState :: Done ; } return self . next () ; } next_read_offset . as_mut () . map (| cookie | { * cookie = dirent . d_next ; }) ; * offset = * offset + DIRENT_SIZE + dirent . d_namlen as usize ; let name = & data [.. (dirent . d_namlen as usize)] ; if name == b"." || name == b".." { return self . next () ; } return Some (Ok (DirEntry { meta : dirent , name : name . to_vec () , inner : self . inner . clone () , })) ; } else if let Some (next_read_offset) = * next_read_offset { self . state = ReadDirState :: FillBuffer { next_read_offset , buf : mem :: take (buf) } ; } else { self . state = ReadDirState :: Done ; } self . next () } ReadDirState :: RunUntilExhaustion { buf , offset } => { if * offset >= buf . len () { self . state = ReadDirState :: Done ; } else { self . state = ReadDirState :: ProcessEntry { buf : mem :: take (buf) , offset : * offset , next_read_offset : None , } ; } self . next () } ReadDirState :: Done => None , } } }}}
mkitem!{mkimpl!{impl DirEntry { pub fn path (& self) -> PathBuf { let name = OsStr :: from_bytes (& self . name) ; self . inner . root . join (name) } pub fn file_name (& self) -> OsString { OsString :: from_vec (self . name . clone ()) } pub fn metadata (& self) -> io :: Result < FileAttr > { metadata_at (& self . inner . dir . fd , 0 , OsStr :: from_bytes (& self . name) . as_ref ()) } pub fn file_type (& self) -> io :: Result < FileType > { Ok (FileType { bits : self . meta . d_type }) } pub fn ino (& self) -> wasi :: Inode { self . meta . d_ino } }}}
mkitem!{mkimpl!{impl OpenOptions { pub fn new () -> OpenOptions { let mut base = OpenOptions :: default () ; base . dirflags = wasi :: LOOKUPFLAGS_SYMLINK_FOLLOW ; base } pub fn read (& mut self , read : bool) { self . read = read ; } pub fn write (& mut self , write : bool) { self . write = write ; } pub fn truncate (& mut self , truncate : bool) { self . oflag (wasi :: OFLAGS_TRUNC , truncate) ; } pub fn create (& mut self , create : bool) { self . oflag (wasi :: OFLAGS_CREAT , create) ; } pub fn create_new (& mut self , create_new : bool) { self . oflag (wasi :: OFLAGS_EXCL , create_new) ; self . oflag (wasi :: OFLAGS_CREAT , create_new) ; } pub fn directory (& mut self , directory : bool) { self . oflag (wasi :: OFLAGS_DIRECTORY , directory) ; } fn oflag (& mut self , bit : wasi :: Oflags , set : bool) { if set { self . oflags |= bit ; } else { self . oflags &= ! bit ; } } pub fn append (& mut self , append : bool) { self . append = append ; self . fdflag (wasi :: FDFLAGS_APPEND , append) ; } pub fn dsync (& mut self , set : bool) { self . fdflag (wasi :: FDFLAGS_DSYNC , set) ; } pub fn nonblock (& mut self , set : bool) { self . fdflag (wasi :: FDFLAGS_NONBLOCK , set) ; } pub fn rsync (& mut self , set : bool) { self . fdflag (wasi :: FDFLAGS_RSYNC , set) ; } pub fn sync (& mut self , set : bool) { self . fdflag (wasi :: FDFLAGS_SYNC , set) ; } fn fdflag (& mut self , bit : wasi :: Fdflags , set : bool) { if set { self . fdflags |= bit ; } else { self . fdflags &= ! bit ; } } pub fn fs_rights_base (& mut self , rights : wasi :: Rights) { self . rights_base = Some (rights) ; } pub fn fs_rights_inheriting (& mut self , rights : wasi :: Rights) { self . rights_inheriting = Some (rights) ; } fn rights_base (& self) -> wasi :: Rights { if let Some (rights) = self . rights_base { return rights ; } let mut base = 0 ; if self . read { base |= wasi :: RIGHTS_FD_READ ; base |= wasi :: RIGHTS_FD_READDIR ; } if self . write || self . append { base |= wasi :: RIGHTS_FD_WRITE ; base |= wasi :: RIGHTS_FD_DATASYNC ; base |= wasi :: RIGHTS_FD_ALLOCATE ; base |= wasi :: RIGHTS_FD_FILESTAT_SET_SIZE ; } base |= wasi :: RIGHTS_FD_ADVISE ; base |= wasi :: RIGHTS_FD_FDSTAT_SET_FLAGS ; base |= wasi :: RIGHTS_FD_FILESTAT_GET ; base |= wasi :: RIGHTS_FD_FILESTAT_SET_TIMES ; base |= wasi :: RIGHTS_FD_SEEK ; base |= wasi :: RIGHTS_FD_SYNC ; base |= wasi :: RIGHTS_FD_TELL ; base |= wasi :: RIGHTS_PATH_CREATE_DIRECTORY ; base |= wasi :: RIGHTS_PATH_CREATE_FILE ; base |= wasi :: RIGHTS_PATH_FILESTAT_GET ; base |= wasi :: RIGHTS_PATH_LINK_SOURCE ; base |= wasi :: RIGHTS_PATH_LINK_TARGET ; base |= wasi :: RIGHTS_PATH_OPEN ; base |= wasi :: RIGHTS_PATH_READLINK ; base |= wasi :: RIGHTS_PATH_REMOVE_DIRECTORY ; base |= wasi :: RIGHTS_PATH_RENAME_SOURCE ; base |= wasi :: RIGHTS_PATH_RENAME_TARGET ; base |= wasi :: RIGHTS_PATH_SYMLINK ; base |= wasi :: RIGHTS_PATH_UNLINK_FILE ; base |= wasi :: RIGHTS_POLL_FD_READWRITE ; base } fn rights_inheriting (& self) -> wasi :: Rights { self . rights_inheriting . unwrap_or_else (| | self . rights_base ()) } pub fn lookup_flags (& mut self , flags : wasi :: Lookupflags) { self . dirflags = flags ; } }}}
mkitem!{mkimpl!{impl File { pub fn open (path : & Path , opts : & OpenOptions) -> io :: Result < File > { let (dir , file) = open_parent (path) ? ; open_at (& dir , & file , opts) } pub fn open_at (& self , path : & Path , opts : & OpenOptions) -> io :: Result < File > { open_at (& self . fd , path , opts) } pub fn file_attr (& self) -> io :: Result < FileAttr > { self . fd . filestat_get () . map (| meta | FileAttr { meta }) } pub fn metadata_at (& self , flags : wasi :: Lookupflags , path : & Path) -> io :: Result < FileAttr > { metadata_at (& self . fd , flags , path) } pub fn fsync (& self) -> io :: Result < () > { self . fd . sync () } pub fn datasync (& self) -> io :: Result < () > { self . fd . datasync () } pub fn lock (& self) -> io :: Result < () > { unsupported () } pub fn lock_shared (& self) -> io :: Result < () > { unsupported () } pub fn try_lock (& self) -> Result < () , TryLockError > { Err (TryLockError :: Error (unsupported_err ())) } pub fn try_lock_shared (& self) -> Result < () , TryLockError > { Err (TryLockError :: Error (unsupported_err ())) } pub fn unlock (& self) -> io :: Result < () > { unsupported () } pub fn truncate (& self , size : u64) -> io :: Result < () > { self . fd . filestat_set_size (size) } pub fn read (& self , buf : & mut [u8]) -> io :: Result < usize > { self . read_vectored (& mut [IoSliceMut :: new (buf)]) } pub fn read_vectored (& self , bufs : & mut [IoSliceMut < '_ >]) -> io :: Result < usize > { self . fd . read (bufs) } # [inline] pub fn is_read_vectored (& self) -> bool { true } pub fn read_buf (& self , cursor : BorrowedCursor < '_ >) -> io :: Result < () > { self . fd . read_buf (cursor) } pub fn write (& self , buf : & [u8]) -> io :: Result < usize > { self . write_vectored (& [IoSlice :: new (buf)]) } pub fn write_vectored (& self , bufs : & [IoSlice < '_ >]) -> io :: Result < usize > { self . fd . write (bufs) } # [inline] pub fn is_write_vectored (& self) -> bool { true } pub fn flush (& self) -> io :: Result < () > { Ok (()) } pub fn seek (& self , pos : SeekFrom) -> io :: Result < u64 > { self . fd . seek (pos) } pub fn size (& self) -> Option < io :: Result < u64 > > { None } pub fn tell (& self) -> io :: Result < u64 > { self . fd . tell () } pub fn duplicate (& self) -> io :: Result < File > { unsupported () } pub fn set_permissions (& self , _perm : FilePermissions) -> io :: Result < () > { unsupported () } pub fn set_times (& self , times : FileTimes) -> io :: Result < () > { let to_timestamp = | time : Option < SystemTime > | match time { Some (time) if let Some (ts) = time . to_wasi_timestamp () => Ok (ts) , Some (_) => Err (io :: const_error ! (io :: ErrorKind :: InvalidInput , "timestamp is too large to set as a file time" ,)) , None => Ok (0) , } ; self . fd . filestat_set_times (to_timestamp (times . accessed) ? , to_timestamp (times . modified) ? , times . accessed . map_or (0 , | _ | wasi :: FSTFLAGS_ATIM) | times . modified . map_or (0 , | _ | wasi :: FSTFLAGS_MTIM) ,) } pub fn read_link (& self , file : & Path) -> io :: Result < PathBuf > { read_link (& self . fd , file) } }}}
mkitem!{mkimpl!{impl AsInner < WasiFd > for File { # [inline] fn as_inner (& self) -> & WasiFd { & self . fd } }}}
mkitem!{mkimpl!{impl IntoInner < WasiFd > for File { fn into_inner (self) -> WasiFd { self . fd } }}}
mkitem!{mkimpl!{impl FromInner < WasiFd > for File { fn from_inner (fd : WasiFd) -> File { File { fd } } }}}
mkitem!{mkimpl!{impl AsFd for File { fn as_fd (& self) -> BorrowedFd < '_ > { self . fd . as_fd () } }}}
mkitem!{mkimpl!{impl AsRawFd for File { # [inline] fn as_raw_fd (& self) -> RawFd { self . fd . as_raw_fd () } }}}
mkitem!{mkimpl!{impl IntoRawFd for File { fn into_raw_fd (self) -> RawFd { self . fd . into_raw_fd () } }}}
mkitem!{mkimpl!{impl FromRawFd for File { unsafe fn from_raw_fd (raw_fd : RawFd) -> Self { unsafe { Self { fd : FromRawFd :: from_raw_fd (raw_fd) } } } }}}
mkitem!{mkimpl!{impl DirBuilder { pub fn new () -> DirBuilder { DirBuilder { } } pub fn mkdir (& self , p : & Path) -> io :: Result < () > { let (dir , file) = open_parent (p) ? ; dir . create_directory (osstr2str (file . as_ref ()) ?) } }}}
mkitem!{mkimpl!{impl fmt :: Debug for File { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("File") . field ("fd" , & self . as_raw_fd ()) . finish () } }}}

macro_rules! readdir_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function readdir in module {}", module_path!());
    };
}

mkfn!{
    readdir_introspect!();
    pub fn readdir (p : & Path) -> io :: Result < ReadDir > { let mut opts = OpenOptions :: new () ; opts . directory (true) ; opts . read (true) ; let dir = File :: open (p , & opts) ? ; Ok (ReadDir :: new (dir , p . to_path_buf ())) }
}

macro_rules! unlink_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function unlink in module {}", module_path!());
    };
}

mkfn!{
    unlink_introspect!();
    pub fn unlink (p : & Path) -> io :: Result < () > { let (dir , file) = open_parent (p) ? ; dir . unlink_file (osstr2str (file . as_ref ()) ?) }
}

macro_rules! rename_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function rename in module {}", module_path!());
    };
}

mkfn!{
    rename_introspect!();
    pub fn rename (old : & Path , new : & Path) -> io :: Result < () > { let (old , old_file) = open_parent (old) ? ; let (new , new_file) = open_parent (new) ? ; old . rename (osstr2str (old_file . as_ref ()) ? , & new , osstr2str (new_file . as_ref ()) ?) }
}

macro_rules! set_perm_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_perm in module {}", module_path!());
    };
}

mkfn!{
    set_perm_introspect!();
    pub fn set_perm (_p : & Path , _perm : FilePermissions) -> io :: Result < () > { unsupported () }
}

macro_rules! rmdir_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function rmdir in module {}", module_path!());
    };
}

mkfn!{
    rmdir_introspect!();
    pub fn rmdir (p : & Path) -> io :: Result < () > { let (dir , file) = open_parent (p) ? ; dir . remove_directory (osstr2str (file . as_ref ()) ?) }
}

macro_rules! readlink_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function readlink in module {}", module_path!());
    };
}

mkfn!{
    readlink_introspect!();
    pub fn readlink (p : & Path) -> io :: Result < PathBuf > { let (dir , file) = open_parent (p) ? ; read_link (& dir , & file) }
}

macro_rules! read_link_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function read_link in module {}", module_path!());
    };
}

mkfn!{
    read_link_introspect!();
    fn read_link (fd : & WasiFd , file : & Path) -> io :: Result < PathBuf > { let meta = metadata_at (fd , 0 , file) ? ; let initial_size = if meta . file_type () . is_symlink () { (meta . size () as usize) . saturating_add (1) } else { 1 } ; let file = osstr2str (file . as_ref ()) ? ; let mut destination = vec ! [0u8 ; initial_size] ; loop { let len = fd . readlink (file , & mut destination) ? ; if len < destination . len () { destination . truncate (len) ; destination . shrink_to_fit () ; return Ok (PathBuf :: from (OsString :: from_vec (destination))) ; } let amt_to_add = destination . len () ; destination . extend (iter :: repeat (0) . take (amt_to_add)) ; } }
}

macro_rules! symlink_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function symlink in module {}", module_path!());
    };
}

mkfn!{
    symlink_introspect!();
    pub fn symlink (original : & Path , link : & Path) -> io :: Result < () > { let (link , link_file) = open_parent (link) ? ; link . symlink (osstr2str (original . as_ref ()) ? , osstr2str (link_file . as_ref ()) ?) }
}

macro_rules! link_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function link in module {}", module_path!());
    };
}

mkfn!{
    link_introspect!();
    pub fn link (original : & Path , link : & Path) -> io :: Result < () > { let (original , original_file) = open_parent (original) ? ; let (link , link_file) = open_parent (link) ? ; original . link (0 , osstr2str (original_file . as_ref ()) ? , & link , osstr2str (link_file . as_ref ()) ?) }
}

macro_rules! stat_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function stat in module {}", module_path!());
    };
}

mkfn!{
    stat_introspect!();
    pub fn stat (p : & Path) -> io :: Result < FileAttr > { let (dir , file) = open_parent (p) ? ; metadata_at (& dir , wasi :: LOOKUPFLAGS_SYMLINK_FOLLOW , & file) }
}

macro_rules! lstat_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lstat in module {}", module_path!());
    };
}

mkfn!{
    lstat_introspect!();
    pub fn lstat (p : & Path) -> io :: Result < FileAttr > { let (dir , file) = open_parent (p) ? ; metadata_at (& dir , 0 , & file) }
}

macro_rules! metadata_at_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function metadata_at in module {}", module_path!());
    };
}

mkfn!{
    metadata_at_introspect!();
    fn metadata_at (fd : & WasiFd , flags : wasi :: Lookupflags , path : & Path) -> io :: Result < FileAttr > { let meta = fd . path_filestat_get (flags , osstr2str (path . as_ref ()) ?) ? ; Ok (FileAttr { meta }) }
}

macro_rules! canonicalize_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function canonicalize in module {}", module_path!());
    };
}

mkfn!{
    canonicalize_introspect!();
    pub fn canonicalize (_p : & Path) -> io :: Result < PathBuf > { unsupported () }
}

macro_rules! open_at_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function open_at in module {}", module_path!());
    };
}

mkfn!{
    open_at_introspect!();
    fn open_at (fd : & WasiFd , path : & Path , opts : & OpenOptions) -> io :: Result < File > { let fd = fd . open (opts . dirflags , osstr2str (path . as_ref ()) ? , opts . oflags , opts . rights_base () , opts . rights_inheriting () , opts . fdflags ,) ? ; Ok (File { fd }) }
}

macro_rules! open_parent_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function open_parent in module {}", module_path!());
    };
}

mkfn!{
    open_parent_introspect!();
    # [doc = " Attempts to open a bare path `p`."] # [doc = ""] # [doc = " WASI has no fundamental capability to do this. All syscalls and operations"] # [doc = " are relative to already-open file descriptors. The C library, however,"] # [doc = " manages a map of pre-opened file descriptors to their path, and then the C"] # [doc = " library provides an API to look at this. In other words, when you want to"] # [doc = " open a path `p`, you have to find a previously opened file descriptor in a"] # [doc = " global table and then see if `p` is relative to that file descriptor."] # [doc = ""] # [doc = " This function, if successful, will return two items:"] # [doc = ""] # [doc = " * The first is a `ManuallyDrop<WasiFd>`. This represents a pre-opened file"] # [doc = "   descriptor which we don't have ownership of, but we can use. You shouldn't"] # [doc = "   actually drop the `fd`."] # [doc = ""] # [doc = " * The second is a path that should be a part of `p` and represents a"] # [doc = "   relative traversal from the file descriptor specified to the desired"] # [doc = "   location `p`."] # [doc = ""] # [doc = " If successful you can use the returned file descriptor to perform"] # [doc = " file-descriptor-relative operations on the path returned as well. The"] # [doc = " `rights` argument indicates what operations are desired on the returned file"] # [doc = " descriptor, and if successful the returned file descriptor should have the"] # [doc = " appropriate rights for performing `rights` actions."] # [doc = ""] # [doc = " Note that this can fail if `p` doesn't look like it can be opened relative"] # [doc = " to any pre-opened file descriptor."] fn open_parent (p : & Path) -> io :: Result < (ManuallyDrop < WasiFd > , PathBuf) > { run_path_with_cstr (p , & | p | { let mut buf = Vec :: < u8 > :: with_capacity (512) ; loop { unsafe { let mut relative_path = buf . as_ptr () . cast () ; let mut abs_prefix = ptr :: null () ; let fd = __wasilibc_find_relpath (p . as_ptr () , & mut abs_prefix , & mut relative_path , buf . capacity () ,) ; if fd == - 1 { if io :: Error :: last_os_error () . raw_os_error () == Some (libc :: ENOMEM) { let cap = buf . capacity () ; buf . set_len (cap) ; buf . reserve (1) ; continue ; } let msg = format ! ("failed to find a pre-opened file descriptor \
                        through which {p:?} could be opened" ,) ; return Err (io :: Error :: new (io :: ErrorKind :: Uncategorized , msg)) ; } let relative = CStr :: from_ptr (relative_path) . to_bytes () . to_vec () ; return Ok ((ManuallyDrop :: new (WasiFd :: from_raw_fd (fd as c_int)) , PathBuf :: from (OsString :: from_vec (relative)) ,)) ; } } unsafe extern "C" { pub fn __wasilibc_find_relpath (path : * const libc :: c_char , abs_prefix : * mut * const libc :: c_char , relative_path : * mut * const libc :: c_char , relative_path_len : libc :: size_t ,) -> libc :: c_int ; } }) }
}

macro_rules! osstr2str_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function osstr2str in module {}", module_path!());
    };
}

mkfn!{
    osstr2str_introspect!();
    pub fn osstr2str (f : & OsStr) -> io :: Result < & str > { f . to_str () . ok_or_else (| | io :: const_error ! (io :: ErrorKind :: Uncategorized , "input must be utf-8")) }
}

macro_rules! copy_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function copy in module {}", module_path!());
    };
}

mkfn!{
    copy_introspect!();
    pub fn copy (from : & Path , to : & Path) -> io :: Result < u64 > { use crate :: fs :: File ; let mut reader = File :: open (from) ? ; let mut writer = File :: create (to) ? ; io :: copy (& mut reader , & mut writer) }
}

macro_rules! remove_dir_all_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function remove_dir_all in module {}", module_path!());
    };
}

mkfn!{
    remove_dir_all_introspect!();
    pub fn remove_dir_all (path : & Path) -> io :: Result < () > { let (parent , path) = open_parent (path) ? ; remove_dir_all_recursive (& parent , & path) }
}

macro_rules! remove_dir_all_recursive_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function remove_dir_all_recursive in module {}", module_path!());
    };
}

mkfn!{
    remove_dir_all_recursive_introspect!();
    fn remove_dir_all_recursive (parent : & WasiFd , path : & Path) -> io :: Result < () > { let mut opts = OpenOptions :: new () ; opts . lookup_flags (0) ; opts . directory (true) ; opts . read (true) ; let fd = open_at (parent , path , & opts) ? ; if fd . file_attr () ? . file_type () . is_symlink () { return parent . unlink_file (osstr2str (path . as_ref ()) ?) ; } let dummy_root = PathBuf :: new () ; for entry in ReadDir :: new (fd , dummy_root) { let entry = entry ? ; let path = crate :: str :: from_utf8 (& entry . name) . map_err (| _ | { io :: const_error ! (io :: ErrorKind :: Uncategorized , "invalid utf-8 file name found") }) ? ; let result : io :: Result < () > = try { if entry . file_type () ? . is_dir () { remove_dir_all_recursive (& entry . inner . dir . fd , path . as_ref ()) ? ; } else { entry . inner . dir . fd . unlink_file (path) ? ; } } ; if let Err (err) = & result && err . kind () != io :: ErrorKind :: NotFound { return result ; } } ignore_notfound (parent . remove_directory (osstr2str (path . as_ref ()) ?)) }
}