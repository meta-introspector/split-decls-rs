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
mkuse!{# [doc = " GCC requires to use the same toolchain for the whole compilation when doing LTO."] # [doc = " So, we need the same version/commit of the linker (gcc) and lto front-end binaries (lto1,"] # [doc = " lto-wrapper, liblto_plugin.so)."] use std :: ffi :: { CStr , CString } ;}
mkuse!{use std :: fs :: { self , File } ;}
mkuse!{use std :: path :: { Path , PathBuf } ;}
mkuse!{use std :: sync :: Arc ;}
mkuse!{use gccjit :: { Context , OutputKind } ;}
mkuse!{use object :: read :: archive :: ArchiveFile ;}
mkuse!{use rustc_codegen_ssa :: back :: lto :: { SerializedModule , ThinModule , ThinShared } ;}
mkuse!{use rustc_codegen_ssa :: back :: write :: { CodegenContext , FatLtoInput } ;}
mkuse!{use rustc_codegen_ssa :: traits :: * ;}
mkuse!{use rustc_codegen_ssa :: { ModuleCodegen , ModuleKind , looks_like_rust_object_file } ;}
mkuse!{use rustc_data_structures :: memmap :: Mmap ;}
mkuse!{use rustc_errors :: DiagCtxtHandle ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: dep_graph :: WorkProduct ;}
mkuse!{use rustc_session :: config :: Lto ;}
mkuse!{use rustc_target :: spec :: RelocModel ;}
mkuse!{use tempfile :: { TempDir , tempdir } ;}
mkuse!{use crate :: back :: write :: save_temp_bitcode ;}
mkuse!{use crate :: errors :: LtoBitcodeFromRlib ;}
mkuse!{use crate :: { GccCodegenBackend , GccContext , SyncContext , to_gcc_opt_level } ;}
mkitem!{mkstruct!{struct LtoData { upstream_modules : Vec < (SerializedModule < ModuleBuffer > , CString) > , tmp_path : TempDir , }}}

macro_rules! prepare_lto_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function prepare_lto in module {}", module_path!());
    };
}

mkfn!{
    prepare_lto_introspect!();
    fn prepare_lto (cgcx : & CodegenContext < GccCodegenBackend > , each_linked_rlib_for_lto : & [PathBuf] , dcx : DiagCtxtHandle < '_ > ,) -> LtoData { let tmp_path = match tempdir () { Ok (tmp_path) => tmp_path , Err (error) => { dcx . fatal (format ! ("Cannot create temporary directory: {}" , error)) ; } } ; let mut upstream_modules = Vec :: new () ; if cgcx . lto != Lto :: ThinLocal { for path in each_linked_rlib_for_lto { let archive_data = unsafe { Mmap :: map (File :: open (path) . expect ("couldn't open rlib")) . expect ("couldn't map rlib") } ; let archive = ArchiveFile :: parse (& * archive_data) . expect ("wanted an rlib") ; let obj_files = archive . members () . filter_map (| child | { child . ok () . and_then (| c | { std :: str :: from_utf8 (c . name ()) . ok () . map (| name | (name . trim () , c)) }) }) . filter (| & (name , _) | looks_like_rust_object_file (name)) ; for (name , child) in obj_files { info ! ("adding bitcode from {}" , name) ; let path = tmp_path . path () . join (name) ; match save_as_file (child . data (& * archive_data) . expect ("corrupt rlib") , & path) { Ok (()) => { let buffer = ModuleBuffer :: new (path) ; let module = SerializedModule :: Local (buffer) ; upstream_modules . push ((module , CString :: new (name) . unwrap ())) ; } Err (e) => { dcx . emit_fatal (e) ; } } } } } LtoData { upstream_modules , tmp_path } }
}

macro_rules! save_as_file_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function save_as_file in module {}", module_path!());
    };
}

mkfn!{
    save_as_file_introspect!();
    fn save_as_file (obj : & [u8] , path : & Path) -> Result < () , LtoBitcodeFromRlib > { fs :: write (path , obj) . map_err (| error | LtoBitcodeFromRlib { gcc_err : format ! ("write object file to temp dir: {}" , error) , }) }
}

macro_rules! run_fat_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function run_fat in module {}", module_path!());
    };
}

mkfn!{
    run_fat_introspect!();
    # [doc = " Performs fat LTO by merging all modules into a single one and returning it"] # [doc = " for further optimization."] pub (crate) fn run_fat (cgcx : & CodegenContext < GccCodegenBackend > , each_linked_rlib_for_lto : & [PathBuf] , modules : Vec < FatLtoInput < GccCodegenBackend > > ,) -> ModuleCodegen < GccContext > { let dcx = cgcx . create_dcx () ; let dcx = dcx . handle () ; let lto_data = prepare_lto (cgcx , each_linked_rlib_for_lto , dcx) ; fat_lto (cgcx , dcx , modules , lto_data . upstream_modules , lto_data . tmp_path ,) }
}

macro_rules! fat_lto_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fat_lto in module {}", module_path!());
    };
}

mkfn!{
    fat_lto_introspect!();
    fn fat_lto (cgcx : & CodegenContext < GccCodegenBackend > , _dcx : DiagCtxtHandle < '_ > , modules : Vec < FatLtoInput < GccCodegenBackend > > , mut serialized_modules : Vec < (SerializedModule < ModuleBuffer > , CString) > , tmp_path : TempDir ,) -> ModuleCodegen < GccContext > { let _timer = cgcx . prof . generic_activity ("GCC_fat_lto_build_monolithic_module") ; info ! ("going for a fat lto") ; let mut in_memory = Vec :: new () ; for module in modules { match module { FatLtoInput :: InMemory (m) => in_memory . push (m) , FatLtoInput :: Serialized { name , buffer } => { info ! ("pushing serialized module {:?}" , name) ; serialized_modules . push ((buffer , CString :: new (name) . unwrap ())) ; } } } let costliest_module = in_memory . iter () . enumerate () . filter (| & (_ , module) | module . kind == ModuleKind :: Regular) . map (| (i , _module) | { (0 , i) }) . max () ; let mut module : ModuleCodegen < GccContext > = match costliest_module { Some ((_cost , i)) => in_memory . remove (i) , None => { unimplemented ! ("Incremental") ; } } ; { info ! ("using {:?} as a base module" , module . name) ; for module in in_memory { let path = tmp_path . path () . to_path_buf () . join (& module . name) ; let path = path . to_str () . expect ("path") ; let context = & module . module_llvm . context ; let config = & cgcx . module_config ; context . set_optimization_level (to_gcc_opt_level (config . opt_level)) ; context . add_command_line_option ("-flto=auto") ; context . add_command_line_option ("-flto-partition=one") ; context . compile_to_file (OutputKind :: ObjectFile , path) ; let buffer = ModuleBuffer :: new (PathBuf :: from (path)) ; let llmod_id = CString :: new (& module . name [..]) . unwrap () ; serialized_modules . push ((SerializedModule :: Local (buffer) , llmod_id)) ; } serialized_modules . sort_by (| module1 , module2 | module1 . 1 . cmp (& module2 . 1)) ; for (bc_decoded , name) in serialized_modules { let _timer = cgcx . prof . generic_activity_with_arg_recorder ("GCC_fat_lto_link_module" , | recorder | { recorder . record_arg (format ! ("{:?}" , name)) }) ; info ! ("linking {:?}" , name) ; match bc_decoded { SerializedModule :: Local (ref module_buffer) => { module . module_llvm . should_combine_object_files = true ; module . module_llvm . context . add_driver_option (module_buffer . 0 . to_str () . expect ("path")) ; } SerializedModule :: FromRlib (_) => unimplemented ! ("from rlib") , SerializedModule :: FromUncompressedFile (_) => { unimplemented ! ("from uncompressed file") } } } save_temp_bitcode (cgcx , & module , "lto.input") ; save_temp_bitcode (cgcx , & module , "lto.after-restriction") ; } module . module_llvm . temp_dir = Some (tmp_path) ; module }
}
mkitem!{mkstruct!{pub struct ModuleBuffer (PathBuf) ;}}
mkitem!{mkimpl!{impl ModuleBuffer { pub fn new (path : PathBuf) -> ModuleBuffer { ModuleBuffer (path) } }}}
mkitem!{mkimpl!{impl ModuleBufferMethods for ModuleBuffer { fn data (& self) -> & [u8] { & [] } }}}

macro_rules! run_thin_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function run_thin in module {}", module_path!());
    };
}

mkfn!{
    run_thin_introspect!();
    # [doc = " Performs thin LTO by performing necessary global analysis and returning two"] # [doc = " lists, one of the modules that need optimization and another for modules that"] # [doc = " can simply be copied over from the incr. comp. cache."] pub (crate) fn run_thin (cgcx : & CodegenContext < GccCodegenBackend > , each_linked_rlib_for_lto : & [PathBuf] , modules : Vec < (String , ThinBuffer) > , cached_modules : Vec < (SerializedModule < ModuleBuffer > , WorkProduct) > ,) -> (Vec < ThinModule < GccCodegenBackend > > , Vec < WorkProduct >) { let dcx = cgcx . create_dcx () ; let dcx = dcx . handle () ; let lto_data = prepare_lto (cgcx , each_linked_rlib_for_lto , dcx) ; if cgcx . opts . cg . linker_plugin_lto . enabled () { unreachable ! ("We should never reach this case if the LTO step \
                      is deferred to the linker") ; } thin_lto (cgcx , dcx , modules , lto_data . upstream_modules , lto_data . tmp_path , cached_modules ,) }
}

macro_rules! prepare_thin_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function prepare_thin in module {}", module_path!());
    };
}

mkfn!{
    prepare_thin_introspect!();
    pub (crate) fn prepare_thin (module : ModuleCodegen < GccContext >) -> (String , ThinBuffer) { let name = module . name ; let buffer = ThinBuffer :: new (& module . module_llvm . context) ; (name , buffer) }
}

macro_rules! thin_lto_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function thin_lto in module {}", module_path!());
    };
}

mkfn!{
    thin_lto_introspect!();
    # [doc = " Prepare \"thin\" LTO to get run on these modules."] # [doc = ""] # [doc = " The general structure of ThinLTO is quite different from the structure of"] # [doc = " \"fat\" LTO above. With \"fat\" LTO all LLVM modules in question are merged into"] # [doc = " one giant LLVM module, and then we run more optimization passes over this"] # [doc = " big module after internalizing most symbols. Thin LTO, on the other hand,"] # [doc = " avoid this large bottleneck through more targeted optimization."] # [doc = ""] # [doc = " At a high level Thin LTO looks like:"] # [doc = ""] # [doc = "    1. Prepare a \"summary\" of each LLVM module in question which describes"] # [doc = "       the values inside, cost of the values, etc."] # [doc = "    2. Merge the summaries of all modules in question into one \"index\""] # [doc = "    3. Perform some global analysis on this index"] # [doc = "    4. For each module, use the index and analysis calculated previously to"] # [doc = "       perform local transformations on the module, for example inlining"] # [doc = "       small functions from other modules."] # [doc = "    5. Run thin-specific optimization passes over each module, and then code"] # [doc = "       generate everything at the end."] # [doc = ""] # [doc = " The summary for each module is intended to be quite cheap, and the global"] # [doc = " index is relatively quite cheap to create as well. As a result, the goal of"] # [doc = " ThinLTO is to reduce the bottleneck on LTO and enable LTO to be used in more"] # [doc = " situations. For example one cheap optimization is that we can parallelize"] # [doc = " all codegen modules, easily making use of all the cores on a machine."] # [doc = ""] # [doc = " With all that in mind, the function here is designed at specifically just"] # [doc = " calculating the *index* for ThinLTO. This index will then be shared amongst"] # [doc = " all of the `LtoModuleCodegen` units returned below and destroyed once"] # [doc = " they all go out of scope."] fn thin_lto (cgcx : & CodegenContext < GccCodegenBackend > , _dcx : DiagCtxtHandle < '_ > , modules : Vec < (String , ThinBuffer) > , serialized_modules : Vec < (SerializedModule < ModuleBuffer > , CString) > , tmp_path : TempDir , cached_modules : Vec < (SerializedModule < ModuleBuffer > , WorkProduct) > ,) -> (Vec < ThinModule < GccCodegenBackend > > , Vec < WorkProduct >) { let _timer = cgcx . prof . generic_activity ("LLVM_thin_lto_global_analysis") ; info ! ("going for that thin, thin LTO") ; let full_scope_len = modules . len () + serialized_modules . len () + cached_modules . len () ; let mut thin_buffers = Vec :: with_capacity (modules . len ()) ; let mut module_names = Vec :: with_capacity (full_scope_len) ; for (i , (name , buffer)) in modules . into_iter () . enumerate () { info ! ("local module: {} - {}" , i , name) ; let cname = CString :: new (name . as_bytes ()) . unwrap () ; thin_buffers . push (buffer) ; module_names . push (cname) ; } let mut serialized = Vec :: with_capacity (serialized_modules . len () + cached_modules . len ()) ; let cached_modules = cached_modules . into_iter () . map (| (sm , wp) | (sm , CString :: new (wp . cgu_name) . unwrap ())) ; for (module , name) in serialized_modules . into_iter () . chain (cached_modules) { info ! ("upstream or cached module {:?}" , name) ; match module { SerializedModule :: Local (_) => { } SerializedModule :: FromRlib (_) => unimplemented ! ("from rlib") , SerializedModule :: FromUncompressedFile (_) => { unimplemented ! ("from uncompressed file") } } serialized . push (module) ; module_names . push (name) ; } let data = ThinData ; info ! ("thin LTO data created") ; let shared = Arc :: new (ThinShared { data , thin_buffers , serialized_modules : serialized , module_names }) ; let copy_jobs = vec ! [] ; let mut opt_jobs = vec ! [] ; info ! ("checking which modules can be-reused and which have to be re-optimized.") ; for (module_index , module_name) in shared . module_names . iter () . enumerate () { let module_name = module_name_to_str (module_name) ; info ! (" - {}: re-compiled" , module_name) ; opt_jobs . push (ThinModule { shared : shared . clone () , idx : module_index }) ; } std :: mem :: forget (tmp_path) ; (opt_jobs , copy_jobs) }
}

macro_rules! optimize_thin_module_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function optimize_thin_module in module {}", module_path!());
    };
}

mkfn!{
    optimize_thin_module_introspect!();
    pub fn optimize_thin_module (thin_module : ThinModule < GccCodegenBackend > , _cgcx : & CodegenContext < GccCodegenBackend > ,) -> ModuleCodegen < GccContext > { let mut should_combine_object_files = false ; let context = match thin_module . shared . thin_buffers . get (thin_module . idx) { Some (thin_buffer) => Arc :: clone (& thin_buffer . context) , None => { let context = Context :: default () ; let len = thin_module . shared . thin_buffers . len () ; let module = & thin_module . shared . serialized_modules [thin_module . idx - len] ; match * module { SerializedModule :: Local (ref module_buffer) => { let path = module_buffer . 0 . to_str () . expect ("path") ; context . add_driver_option (path) ; should_combine_object_files = true ; } SerializedModule :: FromRlib (_) => unimplemented ! ("from rlib") , SerializedModule :: FromUncompressedFile (_) => { unimplemented ! ("from uncompressed file") } } Arc :: new (SyncContext :: new (context)) } } ; let module = ModuleCodegen :: new_regular (thin_module . name () . to_string () , GccContext { context , should_combine_object_files , relocation_model : RelocModel :: Pic , temp_dir : None , } ,) ; # [allow (clippy :: let_and_return)] module }
}
mkitem!{mkstruct!{pub struct ThinBuffer { context : Arc < SyncContext > , }}}
mkitem!{mkimpl!{impl ThinBuffer { pub (crate) fn new (context : & Arc < SyncContext >) -> Self { Self { context : Arc :: clone (context) } } }}}
mkitem!{mkimpl!{impl ThinBufferMethods for ThinBuffer { fn data (& self) -> & [u8] { & [] } }}}
mkitem!{mkstruct!{pub struct ThinData ;}}

macro_rules! module_name_to_str_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function module_name_to_str in module {}", module_path!());
    };
}

mkfn!{
    module_name_to_str_introspect!();
    fn module_name_to_str (c_str : & CStr) -> & str { c_str . to_str () . unwrap_or_else (| e | { bug ! ("Encountered non-utf8 GCC module name `{}`: {}" , c_str . to_string_lossy () , e) }) }
}