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
mkuse!{use std :: env ;}
mkuse!{use std :: error :: Error ;}
mkuse!{use std :: ffi :: OsString ;}
mkuse!{use std :: fs :: { self , File } ;}
mkuse!{use std :: io :: { self , BufWriter , Write } ;}
mkuse!{use std :: path :: { Path , PathBuf } ;}
mkuse!{use ar_archive_writer :: { ArchiveKind , COFFShortExport , MachineTypes , NewArchiveMember , write_archive_to_stream , } ;}
mkuse!{pub use ar_archive_writer :: { DEFAULT_OBJECT_READER , ObjectReader } ;}
mkuse!{use object :: read :: archive :: ArchiveFile ;}
mkuse!{use object :: read :: macho :: FatArch ;}
mkuse!{use rustc_data_structures :: fx :: FxIndexSet ;}
mkuse!{use rustc_data_structures :: memmap :: Mmap ;}
mkuse!{use rustc_fs_util :: TempDirBuilder ;}
mkuse!{use rustc_metadata :: EncodedMetadata ;}
mkuse!{use rustc_session :: Session ;}
mkuse!{use rustc_span :: Symbol ;}
mkuse!{use tracing :: trace ;}
mkuse!{use super :: metadata :: { create_compressed_metadata_file , search_for_section } ;}
mkuse!{use crate :: common ;}
mkuse!{pub use crate :: errors :: { ArchiveBuildFailure , ExtractBundledLibsError , UnknownArchiveKind } ;}
mkuse!{use crate :: errors :: { DlltoolFailImportLibrary , ErrorCallingDllTool , ErrorCreatingImportLibrary , ErrorWritingDEFFile , } ;}
mkitem!{mkstruct!{#[doc = " An item to be included in an import library."] #[doc = " This is a slimmed down version of `COFFShortExport` from `ar-archive-writer`."] pub struct ImportLibraryItem { #[doc = " The name to be exported."] pub name : String , #[doc = " The ordinal to be exported, if any."] pub ordinal : Option < u16 > , #[doc = " The original, decorated name if `name` is not decorated."] pub symbol_name : Option < String > , #[doc = " True if this is a data export, false if it is a function export."] pub is_data : bool , }}}
mkitem!{mkimpl!{impl ImportLibraryItem { fn into_coff_short_export (self , sess : & Session) -> COFFShortExport { let import_name = (sess . target . arch == "arm64ec") . then (| | self . name . clone ()) ; COFFShortExport { name : self . name , ext_name : None , symbol_name : self . symbol_name , import_name , export_as : None , ordinal : self . ordinal . unwrap_or (0) , noname : self . ordinal . is_some () , data : self . is_data , private : false , constant : false , } } }}}
mkitem!{mktrait!{pub trait ArchiveBuilderBuilder { fn new_archive_builder < 'a > (& self , sess : & 'a Session) -> Box < dyn ArchiveBuilder + 'a > ; fn create_dylib_metadata_wrapper (& self , sess : & Session , metadata : & EncodedMetadata , symbol_name : & str ,) -> Vec < u8 > { create_compressed_metadata_file (sess , metadata , symbol_name) } #[doc = " Creates a DLL Import Library <https://docs.microsoft.com/en-us/windows/win32/dlls/dynamic-link-library-creation#creating-an-import-library>."] #[doc = " and returns the path on disk to that import library."] #[doc = " This functions doesn't take `self` so that it can be called from"] #[doc = " `linker_with_args`, which is specialized on `ArchiveBuilder` but"] #[doc = " doesn't take or create an instance of that type."] fn create_dll_import_lib (& self , sess : & Session , lib_name : & str , items : Vec < ImportLibraryItem > , output_path : & Path ,) { if common :: is_mingw_gnu_toolchain (& sess . target) { create_mingw_dll_import_lib (sess , lib_name , items , output_path) ; } else { trace ! ("creating import library") ; trace ! ("  dll_name {:#?}" , lib_name) ; trace ! ("  output_path {}" , output_path . display ()) ; trace ! ("  import names: {}" , items . iter () . map (| ImportLibraryItem { name , .. } | name . clone ()) . collect ::< Vec < _ >> () . join (", ") ,) ; let mut file = match fs :: File :: create_new (& output_path) { Ok (file) => file , Err (error) => sess . dcx () . emit_fatal (ErrorCreatingImportLibrary { lib_name , error : error . to_string () }) , } ; let exports = items . into_iter () . map (| item | item . into_coff_short_export (sess)) . collect :: < Vec < _ > > () ; let machine = match & * sess . target . arch { "x86_64" => MachineTypes :: AMD64 , "x86" => MachineTypes :: I386 , "aarch64" => MachineTypes :: ARM64 , "arm64ec" => MachineTypes :: ARM64EC , "arm" => MachineTypes :: ARMNT , cpu => panic ! ("unsupported cpu type {cpu}") , } ; if let Err (error) = ar_archive_writer :: write_import_library (& mut file , lib_name , & exports , machine , ! sess . target . is_like_msvc , true , & [] ,) { sess . dcx () . emit_fatal (ErrorCreatingImportLibrary { lib_name , error : error . to_string () }) ; } } } fn extract_bundled_libs < 'a > (& 'a self , rlib : & 'a Path , outdir : & Path , bundled_lib_file_names : & FxIndexSet < Symbol > ,) -> Result < () , ExtractBundledLibsError < 'a > > { let archive_map = unsafe { Mmap :: map (File :: open (rlib) . map_err (| e | ExtractBundledLibsError :: OpenFile { rlib , error : Box :: new (e) }) ? ,) . map_err (| e | ExtractBundledLibsError :: MmapFile { rlib , error : Box :: new (e) }) ? } ; let archive = ArchiveFile :: parse (& * archive_map) . map_err (| e | ExtractBundledLibsError :: ParseArchive { rlib , error : Box :: new (e) }) ? ; for entry in archive . members () { let entry = entry . map_err (| e | ExtractBundledLibsError :: ReadEntry { rlib , error : Box :: new (e) }) ? ; let data = entry . data (& * archive_map) . map_err (| e | ExtractBundledLibsError :: ArchiveMember { rlib , error : Box :: new (e) }) ? ; let name = std :: str :: from_utf8 (entry . name ()) . map_err (| e | ExtractBundledLibsError :: ConvertName { rlib , error : Box :: new (e) }) ? ; if ! bundled_lib_file_names . contains (& Symbol :: intern (name)) { continue ; } let data = search_for_section (rlib , data , ".bundled_lib") . map_err (| e | { ExtractBundledLibsError :: ExtractSection { rlib , error : Box :: < dyn Error > :: from (e) } }) ? ; std :: fs :: write (& outdir . join (& name) , data) . map_err (| e | ExtractBundledLibsError :: WriteFile { rlib , error : Box :: new (e) }) ? ; } Ok (()) } }}}

macro_rules! create_mingw_dll_import_lib_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function create_mingw_dll_import_lib in module {}", module_path!());
    };
}

mkfn!{
    create_mingw_dll_import_lib_introspect!();
    fn create_mingw_dll_import_lib (sess : & Session , lib_name : & str , items : Vec < ImportLibraryItem > , output_path : & Path ,) { let def_file_path = output_path . with_extension ("def") ; let def_file_content = format ! ("EXPORTS\n{}" , items . into_iter () . map (| ImportLibraryItem { name , ordinal , .. } | { match ordinal { Some (n) => format ! ("{name} @{n} NONAME") , None => name , } }) . collect ::< Vec < String >> () . join ("\n")) ; match std :: fs :: write (& def_file_path , def_file_content) { Ok (_) => { } Err (e) => { sess . dcx () . emit_fatal (ErrorWritingDEFFile { error : e }) ; } } ; let dlltool = find_binutils_dlltool (sess) ; let temp_prefix = { let mut path = PathBuf :: from (& output_path) ; path . pop () ; path . push (lib_name) ; path } ; let (dlltool_target_arch , dlltool_target_bitness) = match sess . target . arch . as_ref () { "x86_64" => ("i386:x86-64" , "--64") , "x86" => ("i386" , "--32") , "aarch64" => ("arm64" , "--64") , "arm" => ("arm" , "--32") , _ => panic ! ("unsupported arch {}" , sess . target . arch) , } ; let mut dlltool_cmd = std :: process :: Command :: new (& dlltool) ; dlltool_cmd . arg ("-d") . arg (def_file_path) . arg ("-D") . arg (lib_name) . arg ("-l") . arg (& output_path) . arg ("-m") . arg (dlltool_target_arch) . arg ("-f") . arg (dlltool_target_bitness) . arg ("--no-leading-underscore") . arg ("--temp-prefix") . arg (temp_prefix) ; match dlltool_cmd . output () { Err (e) => { sess . dcx () . emit_fatal (ErrorCallingDllTool { dlltool_path : dlltool . to_string_lossy () , error : e , }) ; } Ok (output) if ! output . stderr . is_empty () => { sess . dcx () . emit_fatal (DlltoolFailImportLibrary { dlltool_path : dlltool . to_string_lossy () , dlltool_args : dlltool_cmd . get_args () . map (| arg | arg . to_string_lossy ()) . collect :: < Vec < _ > > () . join (" ") , stdout : String :: from_utf8_lossy (& output . stdout) , stderr : String :: from_utf8_lossy (& output . stderr) , }) } _ => { } } }
}

macro_rules! find_binutils_dlltool_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function find_binutils_dlltool in module {}", module_path!());
    };
}

mkfn!{
    find_binutils_dlltool_introspect!();
    fn find_binutils_dlltool (sess : & Session) -> OsString { assert ! (sess . target . options . is_like_windows && ! sess . target . options . is_like_msvc) ; if let Some (dlltool_path) = & sess . opts . cg . dlltool { return dlltool_path . clone () . into_os_string () ; } let tool_name : OsString = if sess . host . options . is_like_windows { "dlltool.exe" } else { match sess . target . arch . as_ref () { "x86_64" => "x86_64-w64-mingw32-dlltool" , "x86" => "i686-w64-mingw32-dlltool" , "aarch64" => "aarch64-w64-mingw32-dlltool" , _ => "dlltool" , } } . into () ; for dir in env :: split_paths (& env :: var_os ("PATH") . unwrap_or_default ()) { let full_path = dir . join (& tool_name) ; if full_path . is_file () { return full_path . into_os_string () ; } } tool_name }
}
mkitem!{mktrait!{pub trait ArchiveBuilder { fn add_file (& mut self , path : & Path) ; fn add_archive (& mut self , archive : & Path , skip : Box < dyn FnMut (& str) -> bool + 'static > ,) -> io :: Result < () > ; fn build (self : Box < Self > , output : & Path) -> bool ; }}}
mkitem!{mkstruct!{pub struct ArArchiveBuilderBuilder ;}}
mkitem!{mkimpl!{impl ArchiveBuilderBuilder for ArArchiveBuilderBuilder { fn new_archive_builder < 'a > (& self , sess : & 'a Session) -> Box < dyn ArchiveBuilder + 'a > { Box :: new (ArArchiveBuilder :: new (sess , & DEFAULT_OBJECT_READER)) } }}}
mkitem!{mkstruct!{#[must_use = "must call build() to finish building the archive"] pub struct ArArchiveBuilder < 'a > { sess : & 'a Session , object_reader : & 'static ObjectReader , src_archives : Vec < (PathBuf , Mmap) > , entries : Vec < (Vec < u8 > , ArchiveEntry) > , }}}
mkitem!{mkenum!{#[derive (Debug)] enum ArchiveEntry { FromArchive { archive_index : usize , file_range : (u64 , u64) } , File (PathBuf) , }}}
mkitem!{mkimpl!{impl < 'a > ArArchiveBuilder < 'a > { pub fn new (sess : & 'a Session , object_reader : & 'static ObjectReader) -> ArArchiveBuilder < 'a > { ArArchiveBuilder { sess , object_reader , src_archives : vec ! [] , entries : vec ! [] } } }}}

macro_rules! try_filter_fat_archs_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function try_filter_fat_archs in module {}", module_path!());
    };
}

mkfn!{
    try_filter_fat_archs_introspect!();
    fn try_filter_fat_archs (archs : & [impl FatArch] , target_arch : object :: Architecture , archive_path : & Path , archive_map_data : & [u8] ,) -> io :: Result < Option < PathBuf > > { let desired = match archs . iter () . find (| a | a . architecture () == target_arch) { Some (a) => a , None => return Ok (None) , } ; let (mut new_f , extracted_path) = tempfile :: Builder :: new () . suffix (archive_path . file_name () . unwrap ()) . tempfile () ? . keep () . unwrap () ; new_f . write_all (desired . data (archive_map_data) . map_err (| e | io :: Error :: new (io :: ErrorKind :: Other , e)) ? ,) ? ; Ok (Some (extracted_path)) }
}

macro_rules! try_extract_macho_fat_archive_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function try_extract_macho_fat_archive in module {}", module_path!());
    };
}

mkfn!{
    try_extract_macho_fat_archive_introspect!();
    pub fn try_extract_macho_fat_archive (sess : & Session , archive_path : & Path ,) -> io :: Result < Option < PathBuf > > { let archive_map = unsafe { Mmap :: map (File :: open (& archive_path) ?) ? } ; let target_arch = match sess . target . arch . as_ref () { "aarch64" => object :: Architecture :: Aarch64 , "x86_64" => object :: Architecture :: X86_64 , _ => return Ok (None) , } ; if let Ok (h) = object :: read :: macho :: MachOFatFile32 :: parse (& * archive_map) { let archs = h . arches () ; try_filter_fat_archs (archs , target_arch , archive_path , & * archive_map) } else if let Ok (h) = object :: read :: macho :: MachOFatFile64 :: parse (& * archive_map) { let archs = h . arches () ; try_filter_fat_archs (archs , target_arch , archive_path , & * archive_map) } else { Ok (None) } }
}
mkitem!{mkimpl!{impl < 'a > ArchiveBuilder for ArArchiveBuilder < 'a > { fn add_archive (& mut self , archive_path : & Path , mut skip : Box < dyn FnMut (& str) -> bool + 'static > ,) -> io :: Result < () > { let mut archive_path = archive_path . to_path_buf () ; if self . sess . target . llvm_target . contains ("-apple-macosx") && let Some (new_archive_path) = try_extract_macho_fat_archive (self . sess , & archive_path) ? { archive_path = new_archive_path } if self . src_archives . iter () . any (| archive | archive . 0 == archive_path) { return Ok (()) ; } let archive_map = unsafe { Mmap :: map (File :: open (& archive_path) ?) ? } ; let archive = ArchiveFile :: parse (& * archive_map) . map_err (| err | io :: Error :: new (io :: ErrorKind :: InvalidData , err)) ? ; let archive_index = self . src_archives . len () ; for entry in archive . members () { let entry = entry . map_err (| err | io :: Error :: new (io :: ErrorKind :: InvalidData , err)) ? ; let file_name = String :: from_utf8 (entry . name () . to_vec ()) . map_err (| err | io :: Error :: new (io :: ErrorKind :: InvalidData , err)) ? ; if ! skip (& file_name) { if entry . is_thin () { let member_path = archive_path . parent () . unwrap () . join (Path :: new (& file_name)) ; self . entries . push ((file_name . into_bytes () , ArchiveEntry :: File (member_path))) ; } else { self . entries . push ((file_name . into_bytes () , ArchiveEntry :: FromArchive { archive_index , file_range : entry . file_range () } ,)) ; } } } self . src_archives . push ((archive_path , archive_map)) ; Ok (()) } #[doc = " Adds an arbitrary file to this archive"] fn add_file (& mut self , file : & Path) { self . entries . push ((file . file_name () . unwrap () . to_str () . unwrap () . to_string () . into_bytes () , ArchiveEntry :: File (file . to_owned ()) ,)) ; } #[doc = " Combine the provided files, rlibs, and native libraries into a single"] #[doc = " `Archive`."] fn build (self : Box < Self > , output : & Path) -> bool { let sess = self . sess ; match self . build_inner (output) { Ok (any_members) => any_members , Err (error) => { sess . dcx () . emit_fatal (ArchiveBuildFailure { path : output . to_owned () , error }) } } } }}}
mkitem!{mkimpl!{impl < 'a > ArArchiveBuilder < 'a > { fn build_inner (self , output : & Path) -> io :: Result < bool > { let archive_kind = match & * self . sess . target . archive_format { "gnu" => ArchiveKind :: Gnu , "bsd" => ArchiveKind :: Bsd , "darwin" => ArchiveKind :: Darwin , "coff" => ArchiveKind :: Coff , "aix_big" => ArchiveKind :: AixBig , kind => { self . sess . dcx () . emit_fatal (UnknownArchiveKind { kind }) ; } } ; let mut entries = Vec :: new () ; for (entry_name , entry) in self . entries { let data = match entry { ArchiveEntry :: FromArchive { archive_index , file_range } => { let src_archive = & self . src_archives [archive_index] ; let data = & src_archive . 1 [file_range . 0 as usize .. file_range . 0 as usize + file_range . 1 as usize] ; Box :: new (data) as Box < dyn AsRef < [u8] > > } ArchiveEntry :: File (file) => unsafe { Box :: new (Mmap :: map (File :: open (file) . map_err (| err | { io_error_context ("failed to open object file" , err) }) ?) . map_err (| err | io_error_context ("failed to map object file" , err)) ? ,) as Box < dyn AsRef < [u8] > > } , } ; entries . push (NewArchiveMember { buf : data , object_reader : self . object_reader , member_name : String :: from_utf8 (entry_name) . unwrap () , mtime : 0 , uid : 0 , gid : 0 , perms : 0o644 , }) } let archive_tmpdir = TempDirBuilder :: new () . suffix (".temp-archive") . tempdir_in (output . parent () . unwrap_or_else (| | Path :: new (""))) . map_err (| err | { io_error_context ("couldn't create a directory for the temp file" , err) }) ? ; let archive_tmpfile_path = archive_tmpdir . path () . join ("tmp.a") ; let archive_tmpfile = File :: create_new (& archive_tmpfile_path) . map_err (| err | io_error_context ("couldn't create the temp file" , err)) ? ; let mut archive_tmpfile = BufWriter :: new (archive_tmpfile) ; write_archive_to_stream (& mut archive_tmpfile , & entries , archive_kind , false , Some (self . sess . target . arch == "arm64ec") ,) ? ; archive_tmpfile . flush () ? ; drop (archive_tmpfile) ; let any_entries = ! entries . is_empty () ; drop (entries) ; drop (self . src_archives) ; fs :: rename (archive_tmpfile_path , output) . map_err (| err | io_error_context ("failed to rename archive file" , err)) ? ; archive_tmpdir . close () . map_err (| err | io_error_context ("failed to remove temporary directory" , err)) ? ; Ok (any_entries) } }}}

macro_rules! io_error_context_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function io_error_context in module {}", module_path!());
    };
}

mkfn!{
    io_error_context_introspect!();
    fn io_error_context (context : & str , err : io :: Error) -> io :: Error { io :: Error :: new (io :: ErrorKind :: Other , format ! ("{context}: {err}")) }
}