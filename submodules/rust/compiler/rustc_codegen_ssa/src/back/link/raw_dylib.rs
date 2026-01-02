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
mkuse!{use std :: fs ;}
mkuse!{use std :: io :: { BufWriter , Write } ;}
mkuse!{use std :: path :: { Path , PathBuf } ;}
mkuse!{use rustc_abi :: Endian ;}
mkuse!{use rustc_data_structures :: base_n :: { CASE_INSENSITIVE , ToBaseN } ;}
mkuse!{use rustc_data_structures :: fx :: { FxHashMap , FxIndexMap } ;}
mkuse!{use rustc_data_structures :: stable_hasher :: StableHasher ;}
mkuse!{use rustc_hashes :: Hash128 ;}
mkuse!{use rustc_hir :: attrs :: NativeLibKind ;}
mkuse!{use rustc_session :: Session ;}
mkuse!{use rustc_session :: cstore :: DllImport ;}
mkuse!{use rustc_span :: Symbol ;}
mkuse!{use crate :: back :: archive :: ImportLibraryItem ;}
mkuse!{use crate :: back :: link :: ArchiveBuilderBuilder ;}
mkuse!{use crate :: errors :: ErrorCreatingImportLibrary ;}
mkuse!{use crate :: { NativeLib , common , errors } ;}

macro_rules! collate_raw_dylibs_windows_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function collate_raw_dylibs_windows in module {}", module_path!());
    };
}

mkfn!{
    collate_raw_dylibs_windows_introspect!();
    # [doc = " Extract all symbols defined in raw-dylib libraries, collated by library name."] # [doc = ""] # [doc = " If we have multiple extern blocks that specify symbols defined in the same raw-dylib library,"] # [doc = " then the CodegenResults value contains one NativeLib instance for each block. However, the"] # [doc = " linker appears to expect only a single import library for each library used, so we need to"] # [doc = " collate the symbols together by library name before generating the import libraries."] fn collate_raw_dylibs_windows < 'a > (sess : & Session , used_libraries : impl IntoIterator < Item = & 'a NativeLib > ,) -> Vec < (String , Vec < DllImport >) > { let mut dylib_table = FxIndexMap :: < String , FxIndexMap < Symbol , & DllImport > > :: default () ; for lib in used_libraries { if lib . kind == NativeLibKind :: RawDylib { let ext = if lib . verbatim { "" } else { ".dll" } ; let name = format ! ("{}{}" , lib . name , ext) ; let imports = dylib_table . entry (name . clone ()) . or_default () ; for import in & lib . dll_imports { if let Some (old_import) = imports . insert (import . name , import) { if import . calling_convention != old_import . calling_convention { sess . dcx () . emit_err (errors :: MultipleExternalFuncDecl { span : import . span , function : import . name , library_name : & name , }) ; } } } } } sess . dcx () . abort_if_errors () ; dylib_table . into_iter () . map (| (name , imports) | { (name , imports . into_iter () . map (| (_ , import) | import . clone ()) . collect ()) }) . collect () }
}

macro_rules! create_raw_dylib_dll_import_libs_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function create_raw_dylib_dll_import_libs in module {}", module_path!());
    };
}

mkfn!{
    create_raw_dylib_dll_import_libs_introspect!();
    pub (super) fn create_raw_dylib_dll_import_libs < 'a > (sess : & Session , archive_builder_builder : & dyn ArchiveBuilderBuilder , used_libraries : impl IntoIterator < Item = & 'a NativeLib > , tmpdir : & Path , is_direct_dependency : bool ,) -> Vec < PathBuf > { collate_raw_dylibs_windows (sess , used_libraries) . into_iter () . map (| (raw_dylib_name , raw_dylib_imports) | { let name_suffix = if is_direct_dependency { "_imports" } else { "_imports_indirect" } ; let output_path = tmpdir . join (format ! ("{raw_dylib_name}{name_suffix}.lib")) ; let mingw_gnu_toolchain = common :: is_mingw_gnu_toolchain (& sess . target) ; let items : Vec < ImportLibraryItem > = raw_dylib_imports . iter () . map (| import : & DllImport | { if sess . target . arch == "x86" { ImportLibraryItem { name : common :: i686_decorated_name (import , mingw_gnu_toolchain , false , false ,) , ordinal : import . ordinal () , symbol_name : import . is_missing_decorations () . then (| | { common :: i686_decorated_name (import , mingw_gnu_toolchain , false , true ,) }) , is_data : ! import . is_fn , } } else { ImportLibraryItem { name : import . name . to_string () , ordinal : import . ordinal () , symbol_name : None , is_data : ! import . is_fn , } } }) . collect () ; archive_builder_builder . create_dll_import_lib (sess , & raw_dylib_name , items , & output_path ,) ; output_path }) . collect () }
}

macro_rules! collate_raw_dylibs_elf_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function collate_raw_dylibs_elf in module {}", module_path!());
    };
}

mkfn!{
    collate_raw_dylibs_elf_introspect!();
    # [doc = " Extract all symbols defined in raw-dylib libraries, collated by library name."] # [doc = ""] # [doc = " If we have multiple extern blocks that specify symbols defined in the same raw-dylib library,"] # [doc = " then the CodegenResults value contains one NativeLib instance for each block. However, the"] # [doc = " linker appears to expect only a single import library for each library used, so we need to"] # [doc = " collate the symbols together by library name before generating the import libraries."] fn collate_raw_dylibs_elf < 'a > (sess : & Session , used_libraries : impl IntoIterator < Item = & 'a NativeLib > ,) -> Vec < (String , Vec < DllImport >) > { let mut dylib_table = FxIndexMap :: < String , FxIndexMap < Symbol , & DllImport > > :: default () ; for lib in used_libraries { if lib . kind == NativeLibKind :: RawDylib { let filename = if lib . verbatim { lib . name . as_str () . to_owned () } else { let ext = sess . target . dll_suffix . as_ref () ; let prefix = sess . target . dll_prefix . as_ref () ; format ! ("{prefix}{}{ext}" , lib . name) } ; let imports = dylib_table . entry (filename . clone ()) . or_default () ; for import in & lib . dll_imports { imports . insert (import . name , import) ; } } } sess . dcx () . abort_if_errors () ; dylib_table . into_iter () . map (| (name , imports) | { (name , imports . into_iter () . map (| (_ , import) | import . clone ()) . collect ()) }) . collect () }
}

macro_rules! create_raw_dylib_elf_stub_shared_objects_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function create_raw_dylib_elf_stub_shared_objects in module {}", module_path!());
    };
}

mkfn!{
    create_raw_dylib_elf_stub_shared_objects_introspect!();
    pub (super) fn create_raw_dylib_elf_stub_shared_objects < 'a > (sess : & Session , used_libraries : impl IntoIterator < Item = & 'a NativeLib > , raw_dylib_so_dir : & Path ,) -> Vec < String > { collate_raw_dylibs_elf (sess , used_libraries) . into_iter () . map (| (load_filename , raw_dylib_imports) | { use std :: hash :: Hash ; let shared_object = create_elf_raw_dylib_stub (sess , & load_filename , & raw_dylib_imports) ; let mut file_name_hasher = StableHasher :: new () ; load_filename . hash (& mut file_name_hasher) ; for raw_dylib in raw_dylib_imports { raw_dylib . name . as_str () . hash (& mut file_name_hasher) ; } let library_filename : Hash128 = file_name_hasher . finish () ; let temporary_lib_name = format ! ("{}{}{}" , sess . target . dll_prefix , library_filename . as_u128 () . to_base_fixed_len (CASE_INSENSITIVE) , sess . target . dll_suffix) ; let link_path = raw_dylib_so_dir . join (& temporary_lib_name) ; let file = match fs :: File :: create_new (& link_path) { Ok (file) => file , Err (error) => sess . dcx () . emit_fatal (ErrorCreatingImportLibrary { lib_name : & load_filename , error : error . to_string () , }) , } ; if let Err (error) = BufWriter :: new (file) . write_all (& shared_object) { sess . dcx () . emit_fatal (ErrorCreatingImportLibrary { lib_name : & load_filename , error : error . to_string () , }) ; } ; temporary_lib_name }) . collect () }
}

macro_rules! create_elf_raw_dylib_stub_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function create_elf_raw_dylib_stub in module {}", module_path!());
    };
}

mkfn!{
    create_elf_raw_dylib_stub_introspect!();
    # [doc = " Create an ELF .so stub file for raw-dylib."] # [doc = " It exports all the provided symbols, but is otherwise empty."] fn create_elf_raw_dylib_stub (sess : & Session , soname : & str , symbols : & [DllImport]) -> Vec < u8 > { use object :: write :: elf as write ; use object :: { AddressSize , Architecture , elf } ; let mut stub_buf = Vec :: new () ; let Some ((arch , sub_arch)) = sess . target . object_architecture (& sess . unstable_target_features) else { sess . dcx () . fatal (format ! ("raw-dylib is not supported for the architecture `{}`" , sess . target . arch)) ; } ; let endianness = match sess . target . options . endian { Endian :: Little => object :: Endianness :: Little , Endian :: Big => object :: Endianness :: Big , } ; let is_64 = match arch . address_size () { Some (AddressSize :: U8 | AddressSize :: U16 | AddressSize :: U32) => false , Some (AddressSize :: U64) => true , _ => sess . dcx () . fatal (format ! ("raw-dylib is not supported for the architecture `{}`" , sess . target . arch)) , } ; let mut stub = write :: Writer :: new (endianness , is_64 , & mut stub_buf) ; let mut vers = Vec :: new () ; let mut vers_map = FxHashMap :: default () ; let mut syms = Vec :: new () ; for symbol in symbols { let symbol_name = symbol . name . as_str () ; if let Some ((name , version_name)) = symbol_name . split_once ('@') { assert ! (! version_name . contains ('@')) ; let dynstr = stub . add_dynamic_string (name . as_bytes ()) ; let ver = if let Some (& ver_id) = vers_map . get (version_name) { ver_id } else { let id = vers . len () ; vers_map . insert (version_name , id) ; let dynstr = stub . add_dynamic_string (version_name . as_bytes ()) ; vers . push ((version_name , dynstr)) ; id } ; syms . push ((name , dynstr , Some (ver))) ; } else { let dynstr = stub . add_dynamic_string (symbol_name . as_bytes ()) ; syms . push ((symbol_name , dynstr , None)) ; } } let soname = stub . add_dynamic_string (soname . as_bytes ()) ; stub . reserve_null_dynamic_symbol_index () ; for _ in syms . iter () { stub . reserve_dynamic_symbol_index () ; } stub . reserve_shstrtab_section_index () ; let text_section_name = stub . add_section_name (".text" . as_bytes ()) ; let text_section = stub . reserve_section_index () ; stub . reserve_dynsym_section_index () ; stub . reserve_dynstr_section_index () ; if ! vers . is_empty () { stub . reserve_gnu_versym_section_index () ; stub . reserve_gnu_verdef_section_index () ; } stub . reserve_dynamic_section_index () ; stub . reserve_file_header () ; stub . reserve_shstrtab () ; stub . reserve_section_headers () ; stub . reserve_dynsym () ; stub . reserve_dynstr () ; let verdef_count = 1 + vers . len () ; let mut dynamic_entries = 2 ; if ! vers . is_empty () { stub . reserve_gnu_versym () ; stub . reserve_gnu_verdef (verdef_count , verdef_count) ; dynamic_entries += 1 ; } stub . reserve_dynamic (dynamic_entries) ; let e_machine = match (arch , sub_arch) { (Architecture :: Aarch64 , None) => elf :: EM_AARCH64 , (Architecture :: Aarch64_Ilp32 , None) => elf :: EM_AARCH64 , (Architecture :: Arm , None) => elf :: EM_ARM , (Architecture :: Avr , None) => elf :: EM_AVR , (Architecture :: Bpf , None) => elf :: EM_BPF , (Architecture :: Csky , None) => elf :: EM_CSKY , (Architecture :: E2K32 , None) => elf :: EM_MCST_ELBRUS , (Architecture :: E2K64 , None) => elf :: EM_MCST_ELBRUS , (Architecture :: I386 , None) => elf :: EM_386 , (Architecture :: X86_64 , None) => elf :: EM_X86_64 , (Architecture :: X86_64_X32 , None) => elf :: EM_X86_64 , (Architecture :: Hexagon , None) => elf :: EM_HEXAGON , (Architecture :: LoongArch32 , None) => elf :: EM_LOONGARCH , (Architecture :: LoongArch64 , None) => elf :: EM_LOONGARCH , (Architecture :: M68k , None) => elf :: EM_68K , (Architecture :: Mips , None) => elf :: EM_MIPS , (Architecture :: Mips64 , None) => elf :: EM_MIPS , (Architecture :: Mips64_N32 , None) => elf :: EM_MIPS , (Architecture :: Msp430 , None) => elf :: EM_MSP430 , (Architecture :: PowerPc , None) => elf :: EM_PPC , (Architecture :: PowerPc64 , None) => elf :: EM_PPC64 , (Architecture :: Riscv32 , None) => elf :: EM_RISCV , (Architecture :: Riscv64 , None) => elf :: EM_RISCV , (Architecture :: S390x , None) => elf :: EM_S390 , (Architecture :: Sbf , None) => elf :: EM_SBF , (Architecture :: Sharc , None) => elf :: EM_SHARC , (Architecture :: Sparc , None) => elf :: EM_SPARC , (Architecture :: Sparc32Plus , None) => elf :: EM_SPARC32PLUS , (Architecture :: Sparc64 , None) => elf :: EM_SPARCV9 , (Architecture :: Xtensa , None) => elf :: EM_XTENSA , _ => { sess . dcx () . fatal (format ! ("raw-dylib is not supported for the architecture `{}`" , sess . target . arch)) ; } } ; stub . write_file_header (& write :: FileHeader { os_abi : crate :: back :: metadata :: elf_os_abi (sess) , abi_version : 0 , e_type : object :: elf :: ET_DYN , e_machine , e_entry : 0 , e_flags : crate :: back :: metadata :: elf_e_flags (arch , sess) , }) . unwrap () ; stub . write_shstrtab () ; stub . write_null_section_header () ; stub . write_shstrtab_section_header () ; stub . write_section_header (& write :: SectionHeader { name : Some (text_section_name) , sh_type : elf :: SHT_PROGBITS , sh_flags : 0 , sh_addr : 0 , sh_offset : 0 , sh_size : 0 , sh_link : 0 , sh_info : 0 , sh_addralign : 1 , sh_entsize : 0 , }) ; stub . write_dynsym_section_header (0 , 1) ; stub . write_dynstr_section_header (0) ; if ! vers . is_empty () { stub . write_gnu_versym_section_header (0) ; stub . write_gnu_verdef_section_header (0) ; } stub . write_dynamic_section_header (0) ; stub . write_null_dynamic_symbol () ; for (_name , dynstr , _ver) in syms . iter () . copied () { stub . write_dynamic_symbol (& write :: Sym { name : Some (dynstr) , st_info : (elf :: STB_GLOBAL << 4) | elf :: STT_NOTYPE , st_other : elf :: STV_DEFAULT , section : Some (text_section) , st_shndx : 0 , st_value : 0 , st_size : 0 , }) ; } stub . write_dynstr () ; if ! vers . is_empty () { stub . write_null_gnu_versym () ; for (_name , _dynstr , ver) in syms . iter () . copied () { stub . write_gnu_versym (if let Some (ver) = ver { assert ! ((2 + ver as u16) < elf :: VERSYM_HIDDEN) ; elf :: VERSYM_HIDDEN | (2 + ver as u16) } else { 1 }) ; } stub . write_align_gnu_verdef () ; stub . write_gnu_verdef (& write :: Verdef { version : elf :: VER_DEF_CURRENT , flags : elf :: VER_FLG_BASE , index : 1 , aux_count : 1 , name : soname , }) ; for (ver , (_name , dynstr)) in vers . into_iter () . enumerate () { stub . write_gnu_verdef (& write :: Verdef { version : elf :: VER_DEF_CURRENT , flags : 0 , index : 2 + ver as u16 , aux_count : 1 , name : dynstr , }) ; } } stub . write_align_dynamic () ; stub . write_dynamic_string (elf :: DT_SONAME , soname) ; if verdef_count > 1 { stub . write_dynamic (elf :: DT_VERDEFNUM , verdef_count as u64) ; } stub . write_dynamic (elf :: DT_NULL , 0) ; stub_buf }
}