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
mkuse!{use std :: path :: { Path , PathBuf } ;}
mkuse!{use std :: process :: Command ;}
mkuse!{use std :: { env , fs } ;}
mkuse!{use crate :: path :: { Dirs , RelPath } ;}
mkuse!{use crate :: prepare :: apply_patches ;}
mkuse!{use crate :: rustc_info :: { get_default_sysroot , get_file_name } ;}
mkuse!{use crate :: utils :: { CargoProject , Compiler , LogGroup , ensure_empty_dir , spawn_and_wait , try_hard_link , } ;}
mkuse!{use crate :: { CodegenBackend , SysrootKind , config } ;}

macro_rules! build_sysroot_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_sysroot in module {}", module_path!());
    };
}

mkfn!{
    build_sysroot_introspect!();
    pub (crate) fn build_sysroot (dirs : & Dirs , sysroot_kind : SysrootKind , cg_clif_dylib_src : & CodegenBackend , bootstrap_host_compiler : & Compiler , rustup_toolchain_name : Option < & str > , target_triple : String ,) -> Compiler { let _guard = LogGroup :: guard ("Build sysroot") ; eprintln ! ("[BUILD] sysroot {:?}" , sysroot_kind) ; let dist_dir = & dirs . dist_dir ; ensure_empty_dir (dist_dir) ; fs :: create_dir_all (dist_dir . join ("bin")) . unwrap () ; fs :: create_dir_all (dist_dir . join ("lib")) . unwrap () ; let is_native = bootstrap_host_compiler . triple == target_triple ; let cg_clif_dylib_path = match cg_clif_dylib_src { CodegenBackend :: Local (src_path) => { let cg_clif_dylib_path = dist_dir . join ("lib") . join (src_path . file_name () . unwrap ()) ; try_hard_link (src_path , & cg_clif_dylib_path) ; CodegenBackend :: Local (cg_clif_dylib_path) } CodegenBackend :: Builtin (name) => CodegenBackend :: Builtin (name . clone ()) , } ; let wrapper_base_name = get_file_name (& bootstrap_host_compiler . rustc , "____" , "bin") ; for wrapper in ["rustc-clif" , "rustdoc-clif" , "cargo-clif"] { let wrapper_name = wrapper_base_name . replace ("____" , wrapper) ; let mut build_cargo_wrapper_cmd = Command :: new (& bootstrap_host_compiler . rustc) ; let wrapper_path = dist_dir . join (& wrapper_name) ; build_cargo_wrapper_cmd . arg (dirs . source_dir . join ("scripts") . join (& format ! ("{wrapper}.rs"))) . arg ("-o") . arg (& wrapper_path) . arg ("-Cstrip=debuginfo") ; if let Some (rustup_toolchain_name) = & rustup_toolchain_name { build_cargo_wrapper_cmd . env ("TOOLCHAIN_NAME" , rustup_toolchain_name) . env_remove ("CARGO") . env_remove ("RUSTC") . env_remove ("RUSTDOC") ; } else { build_cargo_wrapper_cmd . env_remove ("TOOLCHAIN_NAME") . env ("CARGO" , & bootstrap_host_compiler . cargo) . env ("RUSTC" , & bootstrap_host_compiler . rustc) . env ("RUSTDOC" , & bootstrap_host_compiler . rustdoc) ; } if let CodegenBackend :: Builtin (name) = cg_clif_dylib_src { build_cargo_wrapper_cmd . env ("BUILTIN_BACKEND" , name) ; } spawn_and_wait (build_cargo_wrapper_cmd) ; try_hard_link (wrapper_path , dist_dir . join ("bin") . join (wrapper_name)) ; } let host = build_sysroot_for_triple (dirs , bootstrap_host_compiler . clone () , & cg_clif_dylib_path , sysroot_kind ,) ; host . install_into_sysroot (dist_dir) ; if ! is_native { build_sysroot_for_triple (dirs , { let mut bootstrap_target_compiler = bootstrap_host_compiler . clone () ; bootstrap_target_compiler . triple = target_triple . clone () ; bootstrap_target_compiler . set_cross_linker_and_runner () ; bootstrap_target_compiler } , & cg_clif_dylib_path , sysroot_kind ,) . install_into_sysroot (dist_dir) ; } let mut target_compiler = Compiler { cargo : bootstrap_host_compiler . cargo . clone () , rustc : dist_dir . join (wrapper_base_name . replace ("____" , "rustc-clif")) , rustdoc : dist_dir . join (wrapper_base_name . replace ("____" , "rustdoc-clif")) , rustflags : vec ! [] , rustdocflags : vec ! [] , triple : target_triple , runner : vec ! [] , } ; if ! is_native { target_compiler . set_cross_linker_and_runner () ; } target_compiler }
}
mkitem!{mkstruct!{#[must_use] struct SysrootTarget { triple : String , libs : Vec < PathBuf > , }}}
mkitem!{mkimpl!{impl SysrootTarget { fn install_into_sysroot (& self , sysroot : & Path) { if self . libs . is_empty () { return ; } let target_rustlib_lib = sysroot . join ("lib") . join ("rustlib") . join (& self . triple) . join ("lib") ; fs :: create_dir_all (& target_rustlib_lib) . unwrap () ; for lib in & self . libs { try_hard_link (lib , target_rustlib_lib . join (lib . file_name () . unwrap ())) ; } } }}}
mkitem!{static STDLIB_SRC : RelPath = RelPath :: build ("stdlib") ;}
mkitem!{static STANDARD_LIBRARY : CargoProject = CargoProject :: new (& RelPath :: build ("stdlib/library/sysroot") , "stdlib_target") ;}
mkitem!{static RTSTARTUP_SYSROOT : RelPath = RelPath :: build ("rtstartup") ;}

macro_rules! build_sysroot_for_triple_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_sysroot_for_triple in module {}", module_path!());
    };
}

mkfn!{
    build_sysroot_for_triple_introspect!();
    fn build_sysroot_for_triple (dirs : & Dirs , compiler : Compiler , cg_clif_dylib_path : & CodegenBackend , sysroot_kind : SysrootKind ,) -> SysrootTarget { match sysroot_kind { SysrootKind :: None => build_rtstartup (dirs , & compiler) . unwrap_or (SysrootTarget { triple : compiler . triple , libs : vec ! [] }) , SysrootKind :: Llvm => build_llvm_sysroot_for_triple (compiler) , SysrootKind :: Clif => build_clif_sysroot_for_triple (dirs , compiler , cg_clif_dylib_path) , } }
}

macro_rules! build_llvm_sysroot_for_triple_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_llvm_sysroot_for_triple in module {}", module_path!());
    };
}

mkfn!{
    build_llvm_sysroot_for_triple_introspect!();
    fn build_llvm_sysroot_for_triple (compiler : Compiler) -> SysrootTarget { let default_sysroot = crate :: rustc_info :: get_default_sysroot (& compiler . rustc) ; let mut target_libs = SysrootTarget { triple : compiler . triple , libs : vec ! [] } ; for entry in fs :: read_dir (default_sysroot . join ("lib") . join ("rustlib") . join (& target_libs . triple) . join ("lib") ,) . unwrap () { let entry = entry . unwrap () ; if entry . file_type () . unwrap () . is_dir () { continue ; } let file = entry . path () ; let file_name_str = file . file_name () . unwrap () . to_str () . unwrap () ; if (file_name_str . contains ("rustc_") && ! file_name_str . contains ("rustc_std_workspace_") && ! file_name_str . contains ("rustc_demangle") && ! file_name_str . contains ("rustc_literal_escaper")) || file_name_str . contains ("chalk") || file_name_str . contains ("tracing") || file_name_str . contains ("regex") { continue ; } target_libs . libs . push (file) ; } target_libs }
}

macro_rules! build_clif_sysroot_for_triple_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_clif_sysroot_for_triple in module {}", module_path!());
    };
}

mkfn!{
    build_clif_sysroot_for_triple_introspect!();
    fn build_clif_sysroot_for_triple (dirs : & Dirs , mut compiler : Compiler , cg_clif_dylib_path : & CodegenBackend ,) -> SysrootTarget { let mut target_libs = SysrootTarget { triple : compiler . triple . clone () , libs : vec ! [] } ; if let Some (rtstartup_target_libs) = build_rtstartup (dirs , & compiler) { rtstartup_target_libs . install_into_sysroot (& RTSTARTUP_SYSROOT . to_path (dirs)) ; target_libs . libs . extend (rtstartup_target_libs . libs) ; } let build_dir = STANDARD_LIBRARY . target_dir (dirs) . join (& compiler . triple) . join ("release") ; if ! config :: get_bool ("keep_sysroot") { ensure_empty_dir (& build_dir . join ("deps")) ; } let mut rustflags = vec ! ["-Zforce-unstable-if-unmarked" . to_owned () , "-Cpanic=abort" . to_owned ()] ; match cg_clif_dylib_path { CodegenBackend :: Local (path) => { rustflags . push (format ! ("-Zcodegen-backend={}" , path . to_str () . unwrap ())) ; } CodegenBackend :: Builtin (name) => { rustflags . push (format ! ("-Zcodegen-backend={name}")) ; } } ; rustflags . push ("--sysroot" . to_owned ()) ; rustflags . push (RTSTARTUP_SYSROOT . to_path (dirs) . to_str () . unwrap () . to_owned ()) ; rustflags . push ("-Zinline-mir" . to_owned ()) ; if let Some (prefix) = env :: var_os ("CG_CLIF_STDLIB_REMAP_PATH_PREFIX") { rustflags . push ("--remap-path-prefix" . to_owned ()) ; rustflags . push (format ! ("{}={}" , STDLIB_SRC . to_path (dirs) . to_str () . unwrap () , prefix . to_str () . unwrap ())) ; } compiler . rustflags . extend (rustflags) ; let mut build_cmd = STANDARD_LIBRARY . build (& compiler , dirs) ; build_cmd . arg ("--release") ; build_cmd . arg ("--features") . arg ("backtrace panic-unwind") ; build_cmd . arg (format ! ("-Zroot-dir={}" , STDLIB_SRC . to_path (dirs) . display ())) ; build_cmd . env ("CARGO_PROFILE_RELEASE_DEBUG" , "true") ; build_cmd . env ("__CARGO_DEFAULT_LIB_METADATA" , "cg_clif") ; if compiler . triple . contains ("apple") { build_cmd . env ("CARGO_PROFILE_RELEASE_SPLIT_DEBUGINFO" , "packed") ; } spawn_and_wait (build_cmd) ; for entry in fs :: read_dir (build_dir . join ("deps")) . unwrap () { let entry = entry . unwrap () ; if let Some (ext) = entry . path () . extension () { if ext == "rmeta" || ext == "d" || ext == "dSYM" || ext == "clif" { continue ; } } else { continue ; } ; target_libs . libs . push (entry . path ()) ; } target_libs }
}

macro_rules! build_rtstartup_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_rtstartup in module {}", module_path!());
    };
}

mkfn!{
    build_rtstartup_introspect!();
    fn build_rtstartup (dirs : & Dirs , compiler : & Compiler) -> Option < SysrootTarget > { if ! config :: get_bool ("keep_sysroot") { let sysroot_src_orig = get_default_sysroot (& compiler . rustc) . join ("lib/rustlib/src/rust") ; assert ! (sysroot_src_orig . exists ()) ; apply_patches (dirs , "stdlib" , & sysroot_src_orig , & STDLIB_SRC . to_path (dirs)) ; } if ! compiler . triple . ends_with ("windows-gnu") { return None ; } let rtstartup_sysroot = RTSTARTUP_SYSROOT . to_path (dirs) ; ensure_empty_dir (& rtstartup_sysroot) ; let rtstartup_src = STDLIB_SRC . to_path (dirs) . join ("library") . join ("rtstartup") ; let mut target_libs = SysrootTarget { triple : compiler . triple . clone () , libs : vec ! [] } ; for file in ["rsbegin" , "rsend"] { let obj = rtstartup_sysroot . join (format ! ("{file}.o")) ; let mut build_rtstartup_cmd = Command :: new (& compiler . rustc) ; build_rtstartup_cmd . arg ("--target") . arg (& compiler . triple) . arg ("--emit=obj") . arg ("-o") . arg (& obj) . arg (rtstartup_src . join (format ! ("{file}.rs"))) ; spawn_and_wait (build_rtstartup_cmd) ; target_libs . libs . push (obj . clone ()) ; } Some (target_libs) }
}