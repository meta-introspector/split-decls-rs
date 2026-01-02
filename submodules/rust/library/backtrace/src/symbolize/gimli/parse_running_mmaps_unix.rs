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
mkuse!{use super :: mystd :: ffi :: OsString ;}
mkuse!{use super :: mystd :: fs :: File ;}
mkuse!{use super :: mystd :: io :: Read ;}
mkuse!{use alloc :: string :: String ;}
mkuse!{use alloc :: vec :: Vec ;}
mkuse!{use core :: str :: FromStr ;}
mkitem!{mkstruct!{#[derive (PartialEq , Eq , Debug)] pub (super) struct MapsEntry { #[doc = " start (inclusive) and limit (exclusive) of address range."] address : (usize , usize) , #[doc = " The perms field are the permissions for the entry"] #[doc = ""] #[doc = " r = read"] #[doc = " w = write"] #[doc = " x = execute"] #[doc = " s = shared"] #[doc = " p = private (copy on write)"] perms : [char ; 4] , #[doc = " Offset into the file (or \"whatever\")."] offset : u64 , #[doc = " device (major, minor)"] dev : (usize , usize) , #[doc = " inode on the device. 0 indicates that no inode is associated with the memory region (e.g. uninitalized data aka BSS)."] inode : usize , #[doc = " Usually the file backing the mapping."] #[doc = ""] #[doc = " Note: The man page for proc includes a note about \"coordination\" by"] #[doc = " using readelf to see the Offset field in ELF program headers. pnkfelix"] #[doc = " is not yet sure if that is intended to be a comment on pathname, or what"] #[doc = " form/purpose such coordination is meant to have."] #[doc = ""] #[doc = " There are also some pseudo-paths:"] #[doc = " \"[stack]\": The initial process's (aka main thread's) stack."] #[doc = " \"[stack:<tid>]\": a specific thread's stack. (This was only present for a limited range of Linux verisons; it was determined to be too expensive to provide.)"] #[doc = " \"[vdso]\": Virtual dynamically linked shared object"] #[doc = " \"[heap]\": The process's heap"] #[doc = ""] #[doc = " The pathname can be blank, which means it is an anonymous mapping"] #[doc = " obtained via mmap."] #[doc = ""] #[doc = " Newlines in pathname are replaced with an octal escape sequence."] #[doc = ""] #[doc = " The pathname may have \"(deleted)\" appended onto it if the file-backed"] #[doc = " path has been deleted."] #[doc = ""] #[doc = " Note that modifications like the latter two indicated above imply that"] #[doc = " in general the pathname may be ambiguous. (I.e. you cannot tell if the"] #[doc = " denoted filename actually ended with the text \"(deleted)\", or if that"] #[doc = " was added by the maps rendering."] pathname : OsString , }}}

macro_rules! parse_maps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_maps in module {}", module_path!());
    };
}

mkfn!{
    parse_maps_introspect!();
    pub (super) fn parse_maps () -> Result < Vec < MapsEntry > , & 'static str > { let mut v = Vec :: new () ; let mut proc_self_maps = File :: open ("/proc/self/maps") . map_err (| _ | "Couldn't open /proc/self/maps") ? ; let mut buf = String :: new () ; let _bytes_read = proc_self_maps . read_to_string (& mut buf) . map_err (| _ | "Couldn't read /proc/self/maps") ? ; for line in buf . lines () { v . push (line . parse () ?) ; } Ok (v) }
}
mkitem!{mkimpl!{impl MapsEntry { pub (super) fn pathname (& self) -> & OsString { & self . pathname } pub (super) fn ip_matches (& self , ip : usize) -> bool { self . address . 0 <= ip && ip < self . address . 1 } #[cfg (target_os = "android")] pub (super) fn offset (& self) -> u64 { self . offset } }}}
mkitem!{mkimpl!{impl FromStr for MapsEntry { type Err = & 'static str ; fn from_str (s : & str) -> Result < Self , Self :: Err > { let (range_str , s) = s . trim_start () . split_once (' ') . unwrap_or ((s , "")) ; if range_str . is_empty () { return Err ("Couldn't find address") ; } let (perms_str , s) = s . trim_start () . split_once (' ') . unwrap_or ((s , "")) ; if perms_str . is_empty () { return Err ("Couldn't find permissions") ; } let (offset_str , s) = s . trim_start () . split_once (' ') . unwrap_or ((s , "")) ; if offset_str . is_empty () { return Err ("Couldn't find offset") ; } let (dev_str , s) = s . trim_start () . split_once (' ') . unwrap_or ((s , "")) ; if dev_str . is_empty () { return Err ("Couldn't find dev") ; } let (inode_str , s) = s . trim_start () . split_once (' ') . unwrap_or ((s , "")) ; if inode_str . is_empty () { return Err ("Couldn't find inode") ; } let pathname_str = s . trim_start () ; let hex = | s | usize :: from_str_radix (s , 16) . map_err (| _ | "Couldn't parse hex number") ; let hex64 = | s | u64 :: from_str_radix (s , 16) . map_err (| _ | "Couldn't parse hex number") ; let address = if let Some ((start , limit)) = range_str . split_once ('-') { (hex (start) ? , hex (limit) ?) } else { return Err ("Couldn't parse address range") ; } ; let perms : [char ; 4] = { let mut chars = perms_str . chars () ; let mut c = | | chars . next () . ok_or ("insufficient perms") ; let perms = [c () ? , c () ? , c () ? , c () ?] ; if chars . next () . is_some () { return Err ("too many perms") ; } perms } ; let offset = hex64 (offset_str) ? ; let dev = if let Some ((major , minor)) = dev_str . split_once (':') { (hex (major) ? , hex (minor) ?) } else { return Err ("Couldn't parse dev") ; } ; let inode = hex (inode_str) ? ; let pathname = pathname_str . into () ; Ok (MapsEntry { address , perms , offset , dev , inode , pathname , }) } }}}

macro_rules! check_maps_entry_parsing_64bit_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_maps_entry_parsing_64bit in module {}", module_path!());
    };
}

mkfn!{
    check_maps_entry_parsing_64bit_introspect!();
    #[cfg (target_pointer_width = "64")] #[test] fn check_maps_entry_parsing_64bit () { assert_eq ! ("ffffffffff600000-ffffffffff601000 --xp 00000000 00:00 0                  \
                [vsyscall]" . parse ::< MapsEntry > () . unwrap () , MapsEntry { address : (0xffffffffff600000 , 0xffffffffff601000) , perms : ['-' , '-' , 'x' , 'p'] , offset : 0x00000000 , dev : (0x00 , 0x00) , inode : 0x0 , pathname : "[vsyscall]" . into () , }) ; assert_eq ! ("7f5985f46000-7f5985f48000 rw-p 00039000 103:06 76021795                  \
                /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2" . parse ::< MapsEntry > () . unwrap () , MapsEntry { address : (0x7f5985f46000 , 0x7f5985f48000) , perms : ['r' , 'w' , '-' , 'p'] , offset : 0x00039000 , dev : (0x103 , 0x06) , inode : 0x76021795 , pathname : "/usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2" . into () , }) ; assert_eq ! ("35b1a21000-35b1a22000 rw-p 00000000 00:00 0" . parse ::< MapsEntry > () . unwrap () , MapsEntry { address : (0x35b1a21000 , 0x35b1a22000) , perms : ['r' , 'w' , '-' , 'p'] , offset : 0x00000000 , dev : (0x00 , 0x00) , inode : 0x0 , pathname : Default :: default () , }) ; }
}

macro_rules! check_maps_entry_parsing_32bit_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_maps_entry_parsing_32bit in module {}", module_path!());
    };
}

mkfn!{
    check_maps_entry_parsing_32bit_introspect!();
    #[test] fn check_maps_entry_parsing_32bit () { assert_eq ! ("08056000-08077000 rw-p 00000000 00:00 0          \
                [heap]" . parse ::< MapsEntry > () . unwrap () , MapsEntry { address : (0x08056000 , 0x08077000) , perms : ['r' , 'w' , '-' , 'p'] , offset : 0x00000000 , dev : (0x00 , 0x00) , inode : 0x0 , pathname : "[heap]" . into () , }) ; assert_eq ! ("b7c79000-b7e02000 r--p 00000000 08:01 60662705   \
                /usr/lib/locale/locale-archive" . parse ::< MapsEntry > () . unwrap () , MapsEntry { address : (0xb7c79000 , 0xb7e02000) , perms : ['r' , '-' , '-' , 'p'] , offset : 0x00000000 , dev : (0x08 , 0x01) , inode : 0x60662705 , pathname : "/usr/lib/locale/locale-archive" . into () , }) ; assert_eq ! ("b7e02000-b7e03000 rw-p 00000000 00:00 0" . parse ::< MapsEntry > () . unwrap () , MapsEntry { address : (0xb7e02000 , 0xb7e03000) , perms : ['r' , 'w' , '-' , 'p'] , offset : 0x00000000 , dev : (0x00 , 0x00) , inode : 0x0 , pathname : Default :: default () , }) ; assert_eq ! ("b7c79000-b7e02000 r--p 00000000 08:01 60662705   \
                /executable/path/with some spaces" . parse ::< MapsEntry > () . unwrap () , MapsEntry { address : (0xb7c79000 , 0xb7e02000) , perms : ['r' , '-' , '-' , 'p'] , offset : 0x00000000 , dev : (0x08 , 0x01) , inode : 0x60662705 , pathname : "/executable/path/with some spaces" . into () , }) ; assert_eq ! ("b7c79000-b7e02000 r--p 00000000 08:01 60662705   \
                /executable/path/with  multiple-continuous    spaces  " . parse ::< MapsEntry > () . unwrap () , MapsEntry { address : (0xb7c79000 , 0xb7e02000) , perms : ['r' , '-' , '-' , 'p'] , offset : 0x00000000 , dev : (0x08 , 0x01) , inode : 0x60662705 , pathname : "/executable/path/with  multiple-continuous    spaces  " . into () , }) ; assert_eq ! ("  b7c79000-b7e02000  r--p  00000000  08:01  60662705   \
                /executable/path/starts-with-spaces" . parse ::< MapsEntry > () . unwrap () , MapsEntry { address : (0xb7c79000 , 0xb7e02000) , perms : ['r' , '-' , '-' , 'p'] , offset : 0x00000000 , dev : (0x08 , 0x01) , inode : 0x60662705 , pathname : "/executable/path/starts-with-spaces" . into () , }) ; }
}