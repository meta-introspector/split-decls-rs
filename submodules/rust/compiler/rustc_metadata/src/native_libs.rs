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
mkuse!{use std :: ops :: ControlFlow ;}
mkuse!{use std :: path :: { Path , PathBuf } ;}
mkuse!{use rustc_abi :: ExternAbi ;}
mkuse!{use rustc_ast :: CRATE_NODE_ID ;}
mkuse!{use rustc_attr_parsing :: { ShouldEmit , eval_config_entry } ;}
mkuse!{use rustc_data_structures :: fx :: FxHashSet ;}
mkuse!{use rustc_hir :: attrs :: { AttributeKind , NativeLibKind , PeImportNameType } ;}
mkuse!{use rustc_hir :: find_attr ;}
mkuse!{use rustc_middle :: query :: LocalCrate ;}
mkuse!{use rustc_middle :: ty :: { self , List , Ty , TyCtxt } ;}
mkuse!{use rustc_session :: Session ;}
mkuse!{use rustc_session :: config :: CrateType ;}
mkuse!{use rustc_session :: cstore :: { DllCallingConvention , DllImport , ForeignModule , NativeLib } ;}
mkuse!{use rustc_session :: search_paths :: PathKind ;}
mkuse!{use rustc_span :: Symbol ;}
mkuse!{use rustc_span :: def_id :: { DefId , LOCAL_CRATE } ;}
mkuse!{use rustc_target :: spec :: { BinaryFormat , LinkSelfContainedComponents } ;}
mkuse!{use crate :: errors ;}
mkitem!{mkstruct!{#[doc = " The fallback directories are passed to linker, but not used when rustc does the search,"] #[doc = " because in the latter case the set of fallback directories cannot always be determined"] #[doc = " consistently at the moment."] pub struct NativeLibSearchFallback < 'a > { pub self_contained_components : LinkSelfContainedComponents , pub apple_sdk_root : Option < & 'a Path > , }}}

macro_rules! walk_native_lib_search_dirs_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function walk_native_lib_search_dirs in module {}", module_path!());
    };
}

mkfn!{
    walk_native_lib_search_dirs_introspect!();
    pub fn walk_native_lib_search_dirs < R > (sess : & Session , fallback : Option < NativeLibSearchFallback < '_ > > , mut f : impl FnMut (& Path , bool) -> ControlFlow < R > ,) -> ControlFlow < R > { for search_path in sess . target_filesearch () . cli_search_paths (PathKind :: Native) { f (& search_path . dir , false) ? ; } for search_path in sess . target_filesearch () . cli_search_paths (PathKind :: Framework) { if search_path . kind != PathKind :: All { f (& search_path . dir , true) ? ; } } let Some (NativeLibSearchFallback { self_contained_components , apple_sdk_root }) = fallback else { return ControlFlow :: Continue (()) ; } ; if self_contained_components . intersects (LinkSelfContainedComponents :: LIBC | LinkSelfContainedComponents :: UNWIND | LinkSelfContainedComponents :: MINGW ,) { f (& sess . target_tlib_path . dir . join ("self-contained") , false) ? ; } if sess . target . vendor == "fortanix" || sess . target . os == "linux" || sess . target . os == "fuchsia" || sess . target . is_like_aix || sess . target . is_like_darwin && ! sess . opts . unstable_opts . sanitizer . is_empty () { f (& sess . target_tlib_path . dir , false) ? ; } if let Some (sdk_root) = apple_sdk_root && sess . target . env == "macabi" { f (& sdk_root . join ("System/iOSSupport/usr/lib") , false) ? ; f (& sdk_root . join ("System/iOSSupport/System/Library/Frameworks") , true) ? ; } ControlFlow :: Continue (()) }
}

macro_rules! try_find_native_static_library_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function try_find_native_static_library in module {}", module_path!());
    };
}

mkfn!{
    try_find_native_static_library_introspect!();
    pub fn try_find_native_static_library (sess : & Session , name : & str , verbatim : bool ,) -> Option < PathBuf > { let default = sess . staticlib_components (verbatim) ; let formats = if verbatim { vec ! [default] } else { let unix = ("lib" , ".a") ; if default == unix { vec ! [default] } else { vec ! [default , unix] } } ; walk_native_lib_search_dirs (sess , None , | dir , is_framework | { if ! is_framework { for (prefix , suffix) in & formats { let test = dir . join (format ! ("{prefix}{name}{suffix}")) ; if test . exists () { return ControlFlow :: Break (test) ; } } } ControlFlow :: Continue (()) }) . break_value () }
}

macro_rules! try_find_native_dynamic_library_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function try_find_native_dynamic_library in module {}", module_path!());
    };
}

mkfn!{
    try_find_native_dynamic_library_introspect!();
    pub fn try_find_native_dynamic_library (sess : & Session , name : & str , verbatim : bool ,) -> Option < PathBuf > { let default = sess . staticlib_components (verbatim) ; let formats = if verbatim { vec ! [default] } else { let meson = ("lib" , ".dll.a") ; let mingw = ("lib" , ".a") ; vec ! [default , meson , mingw] } ; walk_native_lib_search_dirs (sess , None , | dir , is_framework | { if ! is_framework { for (prefix , suffix) in & formats { let test = dir . join (format ! ("{prefix}{name}{suffix}")) ; if test . exists () { return ControlFlow :: Break (test) ; } } } ControlFlow :: Continue (()) }) . break_value () }
}

macro_rules! find_native_static_library_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function find_native_static_library in module {}", module_path!());
    };
}

mkfn!{
    find_native_static_library_introspect!();
    pub fn find_native_static_library (name : & str , verbatim : bool , sess : & Session) -> PathBuf { try_find_native_static_library (sess , name , verbatim) . unwrap_or_else (| | sess . dcx () . emit_fatal (errors :: MissingNativeLibrary :: new (name , verbatim))) }
}

macro_rules! find_bundled_library_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function find_bundled_library in module {}", module_path!());
    };
}

mkfn!{
    find_bundled_library_introspect!();
    fn find_bundled_library (name : Symbol , verbatim : Option < bool > , kind : NativeLibKind , has_cfg : bool , tcx : TyCtxt < '_ > ,) -> Option < Symbol > { let sess = tcx . sess ; if let NativeLibKind :: Static { bundle : Some (true) | None , whole_archive } = kind && tcx . crate_types () . iter () . any (| t | matches ! (t , & CrateType :: Rlib | CrateType :: Staticlib)) && (sess . opts . unstable_opts . packed_bundled_libs || has_cfg || whole_archive == Some (true)) { let verbatim = verbatim . unwrap_or (false) ; return find_native_static_library (name . as_str () , verbatim , sess) . file_name () . and_then (| s | s . to_str ()) . map (Symbol :: intern) ; } None }
}

macro_rules! collect_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function collect in module {}", module_path!());
    };
}

mkfn!{
    collect_introspect!();
    pub (crate) fn collect (tcx : TyCtxt < '_ > , LocalCrate : LocalCrate) -> Vec < NativeLib > { let mut collector = Collector { tcx , libs : Vec :: new () } ; if tcx . sess . opts . unstable_opts . link_directives { for module in tcx . foreign_modules (LOCAL_CRATE) . values () { collector . process_module (module) ; } } collector . process_command_line () ; collector . libs }
}

macro_rules! relevant_lib_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function relevant_lib in module {}", module_path!());
    };
}

mkfn!{
    relevant_lib_introspect!();
    pub (crate) fn relevant_lib (sess : & Session , lib : & NativeLib) -> bool { match lib . cfg { Some (ref cfg) => { eval_config_entry (sess , cfg , CRATE_NODE_ID , None , ShouldEmit :: ErrorsAndLints) . as_bool () } None => true , } }
}
mkitem!{mkstruct!{struct Collector < 'tcx > { tcx : TyCtxt < 'tcx > , libs : Vec < NativeLib > , }}}
mkitem!{mkimpl!{impl < 'tcx > Collector < 'tcx > { fn process_module (& mut self , module : & ForeignModule) { let ForeignModule { def_id , abi , ref foreign_items } = * module ; let def_id = def_id . expect_local () ; let sess = self . tcx . sess ; if matches ! (abi , ExternAbi :: Rust) { return ; } for attr in find_attr ! (self . tcx . get_all_attrs (def_id) , AttributeKind :: Link (links , _) => links) . iter () . map (| v | v . iter ()) . flatten () { let dll_imports = match attr . kind { NativeLibKind :: RawDylib => foreign_items . iter () . map (| & child_item | { self . build_dll_import (abi , attr . import_name_type . map (| (import_name_type , _) | import_name_type) , child_item ,) }) . collect () , _ => { for & child_item in foreign_items { if let Some (span) = find_attr ! (self . tcx . get_all_attrs (child_item) , AttributeKind :: LinkOrdinal { span , .. } => * span) { sess . dcx () . emit_err (errors :: LinkOrdinalRawDylib { span }) ; } } Vec :: new () } } ; let filename = find_bundled_library (attr . name , attr . verbatim , attr . kind , attr . cfg . is_some () , self . tcx ,) ; self . libs . push (NativeLib { name : attr . name , filename , kind : attr . kind , cfg : attr . cfg . clone () , foreign_module : Some (def_id . to_def_id ()) , verbatim : attr . verbatim , dll_imports , }) ; } } fn process_command_line (& mut self) { let mut renames = FxHashSet :: default () ; for lib in & self . tcx . sess . opts . libs { if let NativeLibKind :: Framework { .. } = lib . kind && ! self . tcx . sess . target . is_like_darwin { self . tcx . dcx () . emit_err (errors :: LibFrameworkApple) ; } if let Some (ref new_name) = lib . new_name { let any_duplicate = self . libs . iter () . any (| n | n . name . as_str () == lib . name) ; if new_name . is_empty () { self . tcx . dcx () . emit_err (errors :: EmptyRenamingTarget { lib_name : & lib . name }) ; } else if ! any_duplicate { self . tcx . dcx () . emit_err (errors :: RenamingNoLink { lib_name : & lib . name }) ; } else if ! renames . insert (& lib . name) { self . tcx . dcx () . emit_err (errors :: MultipleRenamings { lib_name : & lib . name }) ; } } } for passed_lib in & self . tcx . sess . opts . libs { let mut existing = self . libs . extract_if (.. , | lib | { if lib . name . as_str () == passed_lib . name { if lib . has_modifiers () || passed_lib . has_modifiers () { match lib . foreign_module { Some (def_id) => { self . tcx . dcx () . emit_err (errors :: NoLinkModOverride { span : Some (self . tcx . def_span (def_id)) , }) } None => self . tcx . dcx () . emit_err (errors :: NoLinkModOverride { span : None }) , } ; } if passed_lib . kind != NativeLibKind :: Unspecified { lib . kind = passed_lib . kind ; } if let Some (new_name) = & passed_lib . new_name { lib . name = Symbol :: intern (new_name) ; } lib . verbatim = passed_lib . verbatim ; return true ; } false }) . collect :: < Vec < _ > > () ; if existing . is_empty () { let new_name : Option < & str > = passed_lib . new_name . as_deref () ; let name = Symbol :: intern (new_name . unwrap_or (& passed_lib . name)) ; let filename = find_bundled_library (name , passed_lib . verbatim , passed_lib . kind , false , self . tcx ,) ; self . libs . push (NativeLib { name , filename , kind : passed_lib . kind , cfg : None , foreign_module : None , verbatim : passed_lib . verbatim , dll_imports : Vec :: new () , }) ; } else { self . libs . append (& mut existing) ; } } } fn i686_arg_list_size (& self , item : DefId) -> usize { let argument_types : & List < Ty < '_ > > = self . tcx . instantiate_bound_regions_with_erased (self . tcx . type_of (item) . instantiate_identity () . fn_sig (self . tcx) . inputs () . map_bound (| slice | self . tcx . mk_type_list (slice)) ,) ; argument_types . iter () . map (| ty | { let layout = self . tcx . layout_of (ty :: TypingEnv :: fully_monomorphized () . as_query_input (ty)) . expect ("layout") . layout ; (layout . size () . bytes_usize () + 3) & ! 3 }) . sum () } fn build_dll_import (& self , abi : ExternAbi , import_name_type : Option < PeImportNameType > , item : DefId ,) -> DllImport { let span = self . tcx . def_span (item) ; assert ! (self . tcx . sess . target . is_abi_supported (abi)) ; let calling_convention = if self . tcx . sess . target . arch == "x86" { match abi { ExternAbi :: C { .. } | ExternAbi :: Cdecl { .. } => DllCallingConvention :: C , ExternAbi :: Stdcall { .. } => { DllCallingConvention :: Stdcall (self . i686_arg_list_size (item)) } ExternAbi :: System { .. } => { let c_variadic = self . tcx . type_of (item) . instantiate_identity () . fn_sig (self . tcx) . c_variadic () ; if c_variadic { DllCallingConvention :: C } else { DllCallingConvention :: Stdcall (self . i686_arg_list_size (item)) } } ExternAbi :: Fastcall { .. } => { DllCallingConvention :: Fastcall (self . i686_arg_list_size (item)) } ExternAbi :: Vectorcall { .. } => { DllCallingConvention :: Vectorcall (self . i686_arg_list_size (item)) } _ => { self . tcx . dcx () . emit_fatal (errors :: RawDylibUnsupportedAbi { span }) ; } } } else { match abi { ExternAbi :: C { .. } | ExternAbi :: Win64 { .. } | ExternAbi :: System { .. } => { DllCallingConvention :: C } _ => { self . tcx . dcx () . emit_fatal (errors :: RawDylibUnsupportedAbi { span }) ; } } } ; let codegen_fn_attrs = self . tcx . codegen_fn_attrs (item) ; let import_name_type = codegen_fn_attrs . link_ordinal . map_or (import_name_type , | ord | Some (PeImportNameType :: Ordinal (ord))) ; let name = codegen_fn_attrs . symbol_name . unwrap_or_else (| | self . tcx . item_name (item)) ; if self . tcx . sess . target . binary_format == BinaryFormat :: Elf { let name = name . as_str () ; if name . contains ('\0') { self . tcx . dcx () . emit_err (errors :: RawDylibMalformed { span }) ; } else if let Some ((left , right)) = name . split_once ('@') && (left . is_empty () || right . is_empty () || right . contains ('@')) { self . tcx . dcx () . emit_err (errors :: RawDylibMalformed { span }) ; } } DllImport { name , import_name_type , calling_convention , span , is_fn : self . tcx . def_kind (item) . is_fn_like () , } } }}}