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
mkuse!{use r_efi :: protocols :: file ;}
mkuse!{use crate :: ffi :: OsString ;}
mkuse!{use crate :: fmt ;}
mkuse!{use crate :: fs :: TryLockError ;}
mkuse!{use crate :: hash :: Hash ;}
mkuse!{use crate :: io :: { self , BorrowedCursor , IoSlice , IoSliceMut , SeekFrom } ;}
mkuse!{use crate :: path :: { Path , PathBuf } ;}
mkuse!{use crate :: sys :: time :: SystemTime ;}
mkuse!{use crate :: sys :: unsupported ;}
mkitem!{# [expect (dead_code)] const FILE_PERMISSIONS_MASK : u64 = r_efi :: protocols :: file :: READ_ONLY ;}
mkitem!{mkstruct!{pub struct File (!) ;}}
mkitem!{mkstruct!{# [derive (Clone)] pub struct FileAttr { attr : u64 , size : u64 , }}}
mkitem!{mkstruct!{pub struct ReadDir (!) ;}}
mkitem!{mkstruct!{pub struct DirEntry (!) ;}}
mkitem!{mkstruct!{# [derive (Clone , Debug)] pub struct OpenOptions { mode : u64 , append : bool , truncate : bool , create_new : bool , }}}
mkitem!{mkstruct!{# [derive (Copy , Clone , Debug , Default)] pub struct FileTimes { }}}
mkitem!{mkstruct!{# [derive (Clone , PartialEq , Eq , Debug)] pub struct FilePermissions (bool) ;}}
mkitem!{mkstruct!{# [derive (Clone , Copy , PartialEq , Eq , Hash , Debug)] pub struct FileType (bool) ;}}
mkitem!{mkstruct!{# [derive (Debug)] pub struct DirBuilder ;}}
mkitem!{mkimpl!{impl FileAttr { pub fn size (& self) -> u64 { self . size } pub fn perm (& self) -> FilePermissions { FilePermissions :: from_attr (self . attr) } pub fn file_type (& self) -> FileType { FileType :: from_attr (self . attr) } pub fn modified (& self) -> io :: Result < SystemTime > { unsupported () } pub fn accessed (& self) -> io :: Result < SystemTime > { unsupported () } pub fn created (& self) -> io :: Result < SystemTime > { unsupported () } }}}
mkitem!{mkimpl!{impl FilePermissions { pub fn readonly (& self) -> bool { self . 0 } pub fn set_readonly (& mut self , readonly : bool) { self . 0 = readonly } const fn from_attr (attr : u64) -> Self { Self (attr & r_efi :: protocols :: file :: READ_ONLY != 0) } # [expect (dead_code)] const fn to_attr (& self) -> u64 { if self . 0 { r_efi :: protocols :: file :: READ_ONLY } else { 0 } } }}}
mkitem!{mkimpl!{impl FileTimes { pub fn set_accessed (& mut self , _t : SystemTime) { } pub fn set_modified (& mut self , _t : SystemTime) { } }}}
mkitem!{mkimpl!{impl FileType { pub fn is_dir (& self) -> bool { self . 0 } pub fn is_file (& self) -> bool { ! self . is_dir () } pub fn is_symlink (& self) -> bool { false } const fn from_attr (attr : u64) -> Self { Self (attr & r_efi :: protocols :: file :: DIRECTORY != 0) } }}}
mkitem!{mkimpl!{impl fmt :: Debug for ReadDir { fn fmt (& self , _f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . 0 } }}}
mkitem!{mkimpl!{impl Iterator for ReadDir { type Item = io :: Result < DirEntry > ; fn next (& mut self) -> Option < io :: Result < DirEntry > > { self . 0 } }}}
mkitem!{mkimpl!{impl DirEntry { pub fn path (& self) -> PathBuf { self . 0 } pub fn file_name (& self) -> OsString { self . 0 } pub fn metadata (& self) -> io :: Result < FileAttr > { self . 0 } pub fn file_type (& self) -> io :: Result < FileType > { self . 0 } }}}
mkitem!{mkimpl!{impl OpenOptions { pub fn new () -> OpenOptions { OpenOptions { mode : 0 , append : false , create_new : false , truncate : false } } pub fn read (& mut self , read : bool) { if read { self . mode |= file :: MODE_READ ; } else { self . mode &= ! file :: MODE_READ ; } } pub fn write (& mut self , write : bool) { if write { self . read (true) ; self . mode |= file :: MODE_WRITE ; } else { self . mode &= ! file :: MODE_WRITE ; } } pub fn append (& mut self , append : bool) { if append { self . write (true) ; } self . append = append ; } pub fn truncate (& mut self , truncate : bool) { self . truncate = truncate ; } pub fn create (& mut self , create : bool) { if create { self . mode |= file :: MODE_CREATE ; } else { self . mode &= ! file :: MODE_CREATE ; } } pub fn create_new (& mut self , create_new : bool) { self . create_new = create_new ; } # [expect (dead_code)] const fn is_mode_valid (& self) -> bool { self . mode == file :: MODE_READ || self . mode == (file :: MODE_READ | file :: MODE_WRITE) || self . mode == (file :: MODE_READ | file :: MODE_WRITE | file :: MODE_CREATE) } }}}
mkitem!{mkimpl!{impl File { pub fn open (_path : & Path , _opts : & OpenOptions) -> io :: Result < File > { unsupported () } pub fn file_attr (& self) -> io :: Result < FileAttr > { self . 0 } pub fn fsync (& self) -> io :: Result < () > { self . 0 } pub fn datasync (& self) -> io :: Result < () > { self . 0 } pub fn lock (& self) -> io :: Result < () > { self . 0 } pub fn lock_shared (& self) -> io :: Result < () > { self . 0 } pub fn try_lock (& self) -> Result < () , TryLockError > { self . 0 } pub fn try_lock_shared (& self) -> Result < () , TryLockError > { self . 0 } pub fn unlock (& self) -> io :: Result < () > { self . 0 } pub fn truncate (& self , _size : u64) -> io :: Result < () > { self . 0 } pub fn read (& self , _buf : & mut [u8]) -> io :: Result < usize > { self . 0 } pub fn read_vectored (& self , _bufs : & mut [IoSliceMut < '_ >]) -> io :: Result < usize > { self . 0 } pub fn is_read_vectored (& self) -> bool { self . 0 } pub fn read_buf (& self , _cursor : BorrowedCursor < '_ >) -> io :: Result < () > { self . 0 } pub fn write (& self , _buf : & [u8]) -> io :: Result < usize > { self . 0 } pub fn write_vectored (& self , _bufs : & [IoSlice < '_ >]) -> io :: Result < usize > { self . 0 } pub fn is_write_vectored (& self) -> bool { self . 0 } pub fn flush (& self) -> io :: Result < () > { self . 0 } pub fn seek (& self , _pos : SeekFrom) -> io :: Result < u64 > { self . 0 } pub fn size (& self) -> Option < io :: Result < u64 > > { self . 0 } pub fn tell (& self) -> io :: Result < u64 > { self . 0 } pub fn duplicate (& self) -> io :: Result < File > { self . 0 } pub fn set_permissions (& self , _perm : FilePermissions) -> io :: Result < () > { self . 0 } pub fn set_times (& self , _times : FileTimes) -> io :: Result < () > { self . 0 } }}}
mkitem!{mkimpl!{impl DirBuilder { pub fn new () -> DirBuilder { DirBuilder } pub fn mkdir (& self , p : & Path) -> io :: Result < () > { uefi_fs :: mkdir (p) } }}}
mkitem!{mkimpl!{impl fmt :: Debug for File { fn fmt (& self , _f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . 0 } }}}

macro_rules! readdir_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function readdir in module {}", module_path!());
    };
}

mkfn!{
    readdir_introspect!();
    pub fn readdir (_p : & Path) -> io :: Result < ReadDir > { unsupported () }
}

macro_rules! unlink_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function unlink in module {}", module_path!());
    };
}

mkfn!{
    unlink_introspect!();
    pub fn unlink (_p : & Path) -> io :: Result < () > { unsupported () }
}

macro_rules! rename_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function rename in module {}", module_path!());
    };
}

mkfn!{
    rename_introspect!();
    pub fn rename (_old : & Path , _new : & Path) -> io :: Result < () > { unsupported () }
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
    pub fn rmdir (_p : & Path) -> io :: Result < () > { unsupported () }
}

macro_rules! remove_dir_all_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function remove_dir_all in module {}", module_path!());
    };
}

mkfn!{
    remove_dir_all_introspect!();
    pub fn remove_dir_all (_path : & Path) -> io :: Result < () > { unsupported () }
}

macro_rules! exists_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function exists in module {}", module_path!());
    };
}

mkfn!{
    exists_introspect!();
    pub fn exists (path : & Path) -> io :: Result < bool > { let f = uefi_fs :: File :: from_path (path , r_efi :: protocols :: file :: MODE_READ , 0) ; match f { Ok (_) => Ok (true) , Err (e) if e . kind () == io :: ErrorKind :: NotFound => Ok (false) , Err (e) => Err (e) , } }
}

macro_rules! readlink_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function readlink in module {}", module_path!());
    };
}

mkfn!{
    readlink_introspect!();
    pub fn readlink (_p : & Path) -> io :: Result < PathBuf > { unsupported () }
}

macro_rules! symlink_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function symlink in module {}", module_path!());
    };
}

mkfn!{
    symlink_introspect!();
    pub fn symlink (_original : & Path , _link : & Path) -> io :: Result < () > { unsupported () }
}

macro_rules! link_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function link in module {}", module_path!());
    };
}

mkfn!{
    link_introspect!();
    pub fn link (_src : & Path , _dst : & Path) -> io :: Result < () > { unsupported () }
}

macro_rules! stat_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function stat in module {}", module_path!());
    };
}

mkfn!{
    stat_introspect!();
    pub fn stat (_p : & Path) -> io :: Result < FileAttr > { unsupported () }
}

macro_rules! lstat_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lstat in module {}", module_path!());
    };
}

mkfn!{
    lstat_introspect!();
    pub fn lstat (p : & Path) -> io :: Result < FileAttr > { stat (p) }
}

macro_rules! canonicalize_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function canonicalize in module {}", module_path!());
    };
}

mkfn!{
    canonicalize_introspect!();
    pub fn canonicalize (p : & Path) -> io :: Result < PathBuf > { crate :: path :: absolute (p) }
}

macro_rules! copy_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function copy in module {}", module_path!());
    };
}

mkfn!{
    copy_introspect!();
    pub fn copy (_from : & Path , _to : & Path) -> io :: Result < u64 > { unsupported () }
}
mkmod!{uefi_fs, { 
                getname!(uefi_fs);
                getsrc!(uefi_fs);
                getpath!(uefi_fs);
                get_deps!(uefi_fs);
                get_crates!(uefi_fs);
                mkinclude!(uefi_fs);
                mkuse!{use r_efi :: protocols :: { device_path , file , simple_file_system } ;}
mkuse!{use crate :: boxed :: Box ;}
mkuse!{use crate :: io ;}
mkuse!{use crate :: path :: Path ;}
mkuse!{use crate :: ptr :: NonNull ;}
mkuse!{use crate :: sys :: helpers ;}
mkitem!{mkstruct!{pub (crate) struct File (NonNull < file :: Protocol >) ;}}
mkitem!{mkimpl!{impl File { pub (crate) fn from_path (path : & Path , open_mode : u64 , attr : u64) -> io :: Result < Self > { let absolute = crate :: path :: absolute (path) ? ; let p = helpers :: OwnedDevicePath :: from_text (absolute . as_os_str ()) ? ; let (vol , mut path_remaining) = Self :: open_volume_from_device_path (p . borrow ()) ? ; vol . open (& mut path_remaining , open_mode , attr) } # [doc = " Open Filesystem volume given a devicepath to the volume, or a file/directory in the"] # [doc = " volume. The path provided should be absolute UEFI device path, without any UEFI shell"] # [doc = " mappings."] # [doc = ""] # [doc = " Returns"] # [doc = " 1. The volume as a UEFI File"] # [doc = " 2. Path relative to the volume."] # [doc = ""] # [doc = " For example, given \"PciRoot(0x0)/Pci(0x1,0x1)/Ata(Secondary,Slave,0x0)/\\abc\\run.efi\","] # [doc = " this will open the volume \"PciRoot(0x0)/Pci(0x1,0x1)/Ata(Secondary,Slave,0x0)\""] # [doc = " and return the remaining file path \"\\abc\\run.efi\"."] fn open_volume_from_device_path (path : helpers :: BorrowedDevicePath < '_ > ,) -> io :: Result < (Self , Box < [u16] >) > { let handles = match helpers :: locate_handles (simple_file_system :: PROTOCOL_GUID) { Ok (x) => x , Err (e) => return Err (e) , } ; for handle in handles { let volume_device_path : NonNull < device_path :: Protocol > = match helpers :: open_protocol (handle , device_path :: PROTOCOL_GUID) { Ok (x) => x , Err (_) => continue , } ; let volume_device_path = helpers :: BorrowedDevicePath :: new (volume_device_path) ; if let Some (left_path) = path_best_match (& volume_device_path , & path) { return Ok ((Self :: open_volume (handle) ? , left_path)) ; } } Err (io :: const_error ! (io :: ErrorKind :: NotFound , "Volume Not Found")) } fn open_volume (device_handle : NonNull < crate :: ffi :: c_void >) -> io :: Result < Self > { let simple_file_system_protocol = helpers :: open_protocol :: < simple_file_system :: Protocol > (device_handle , simple_file_system :: PROTOCOL_GUID ,) ? ; let mut file_protocol = crate :: ptr :: null_mut () ; let r = unsafe { ((* simple_file_system_protocol . as_ptr ()) . open_volume) (simple_file_system_protocol . as_ptr () , & mut file_protocol ,) } ; if r . is_error () { return Err (io :: Error :: from_raw_os_error (r . as_usize ())) ; } let p = NonNull :: new (file_protocol) . unwrap () ; Ok (Self (p)) } fn open (& self , path : & mut [u16] , open_mode : u64 , attr : u64) -> io :: Result < Self > { let file_ptr = self . 0 . as_ptr () ; let mut file_opened = crate :: ptr :: null_mut () ; let r = unsafe { ((* file_ptr) . open) (file_ptr , & mut file_opened , path . as_mut_ptr () , open_mode , attr) } ; if r . is_error () { return Err (io :: Error :: from_raw_os_error (r . as_usize ())) ; } let p = NonNull :: new (file_opened) . unwrap () ; Ok (File (p)) } }}}
mkitem!{mkimpl!{impl Drop for File { fn drop (& mut self) { let file_ptr = self . 0 . as_ptr () ; let _ = unsafe { ((* self . 0 . as_ptr ()) . close) (file_ptr) } ; } }}}

macro_rules! path_best_match_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function path_best_match in module {}", module_path!());
    };
}

mkfn!{
    path_best_match_introspect!();
    # [doc = " A helper to check that target path is a descendent of source. It is expected to be used with"] # [doc = " absolute UEFI device paths without any UEFI shell mappings."] # [doc = ""] # [doc = " Returns the path relative to source"] # [doc = ""] # [doc = " For example, given \"PciRoot(0x0)/Pci(0x1,0x1)/Ata(Secondary,Slave,0x0)/\" and"] # [doc = " \"PciRoot(0x0)/Pci(0x1,0x1)/Ata(Secondary,Slave,0x0)/\\abc\\run.efi\", this will return"] # [doc = " \"\\abc\\run.efi\""] fn path_best_match (source : & helpers :: BorrowedDevicePath < '_ > , target : & helpers :: BorrowedDevicePath < '_ > ,) -> Option < Box < [u16] > > { let mut source_iter = source . iter () . take_while (| x | ! x . is_end_instance ()) ; let mut target_iter = target . iter () . take_while (| x | ! x . is_end_instance ()) ; loop { match (source_iter . next () , target_iter . next ()) { (Some (x) , Some (y)) if x == y => continue , (None , Some (y)) => { let p = y . to_path () . to_text () . ok () ? ; return helpers :: os_string_to_raw (& p) ; } _ => return None , } } }
}

macro_rules! mkdir_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function mkdir in module {}", module_path!());
    };
}

mkfn!{
    mkdir_introspect!();
    # [doc = " An implementation of mkdir to allow creating new directory without having to open the"] # [doc = " volume twice (once for checking and once for creating)"] pub (crate) fn mkdir (path : & Path) -> io :: Result < () > { let absolute = crate :: path :: absolute (path) ? ; let p = helpers :: OwnedDevicePath :: from_text (absolute . as_os_str ()) ? ; let (vol , mut path_remaining) = File :: open_volume_from_device_path (p . borrow ()) ? ; match vol . open (& mut path_remaining , file :: MODE_READ , 0) { Ok (_) => { return Err (io :: Error :: new (io :: ErrorKind :: AlreadyExists , "Path already exists")) ; } Err (e) if e . kind () == io :: ErrorKind :: NotFound => { } Err (e) => return Err (e) , } let _ = vol . open (& mut path_remaining , file :: MODE_READ | file :: MODE_WRITE | file :: MODE_CREATE , file :: DIRECTORY ,) ? ; Ok (()) }
} 
            }}