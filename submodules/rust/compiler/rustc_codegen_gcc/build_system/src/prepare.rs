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
mkuse!{use std :: fs ;}
mkuse!{use std :: path :: { Path , PathBuf } ;}
mkuse!{use crate :: rustc_info :: get_rustc_path ;}
mkuse!{use crate :: utils :: { cargo_install , create_dir , get_sysroot_dir , git_clone_root_dir , remove_file , run_command , run_command_with_output , walk_dir , } ;}

macro_rules! prepare_libcore_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function prepare_libcore in module {}", module_path!());
    };
}

mkfn!{
    prepare_libcore_introspect!();
    fn prepare_libcore (sysroot_path : & Path , libgccjit12_patches : bool , cross_compile : bool , sysroot_source : Option < String > ,) -> Result < () , String > { let rustlib_dir : PathBuf ; if let Some (path) = sysroot_source { rustlib_dir = Path :: new (& path) . canonicalize () . map_err (| error | format ! ("Failed to canonicalize path: {error:?}")) ? ; if ! rustlib_dir . is_dir () { return Err (format ! ("Custom sysroot path {rustlib_dir:?} not found")) ; } } else { let rustc_path = match get_rustc_path () { Some (path) => path , None => return Err ("`rustc` path not found" . to_string ()) , } ; let parent = match rustc_path . parent () { Some (path) => path , None => return Err (format ! ("No parent for `{}`" , rustc_path . display ())) , } ; rustlib_dir = parent . join ("../lib/rustlib/src/rust") . canonicalize () . map_err (| error | format ! ("Failed to canonicalize path: {error:?}")) ? ; if ! rustlib_dir . is_dir () { return Err ("Please install `rust-src` component" . to_string ()) ; } } let sysroot_dir = sysroot_path . join ("sysroot_src") ; if sysroot_dir . is_dir () && let Err (error) = fs :: remove_dir_all (& sysroot_dir) { return Err (format ! ("Failed to remove `{}`: {:?}" , sysroot_dir . display () , error ,)) ; } let sysroot_library_dir = sysroot_dir . join ("library") ; create_dir (& sysroot_library_dir) ? ; run_command (& [& "cp" , & "-r" , & rustlib_dir . join ("library") , & sysroot_dir] , None) ? ; println ! ("[GIT] init (cwd): `{}`" , sysroot_dir . display ()) ; run_command (& [& "git" , & "init"] , Some (& sysroot_dir)) ? ; println ! ("[GIT] add (cwd): `{}`" , sysroot_dir . display ()) ; run_command (& [& "git" , & "add" , & "."] , Some (& sysroot_dir)) ? ; println ! ("[GIT] commit (cwd): `{}`" , sysroot_dir . display ()) ; run_command (& [& "git" , & "config" , & "user.email" , & "none@example.com"] , Some (& sysroot_dir)) ? ; run_command (& [& "git" , & "config" , & "user.name" , & "None"] , Some (& sysroot_dir)) ? ; run_command (& [& "git" , & "config" , & "core.autocrlf" , & "false"] , Some (& sysroot_dir)) ? ; run_command (& [& "git" , & "config" , & "commit.gpgSign" , & "false"] , Some (& sysroot_dir)) ? ; run_command (& [& "git" , & "commit" , & "-m" , & "Initial commit" , & "-q"] , Some (& sysroot_dir)) ? ; let mut patches = Vec :: new () ; walk_dir ("patches" , & mut | _ | Ok (()) , & mut | file_path : & Path | { patches . push (file_path . to_path_buf ()) ; Ok (()) } , false ,) ? ; if cross_compile { walk_dir ("patches/cross_patches" , & mut | _ | Ok (()) , & mut | file_path : & Path | { patches . push (file_path . to_path_buf ()) ; Ok (()) } , false ,) ? ; } if libgccjit12_patches { walk_dir ("patches/libgccjit12" , & mut | _ | Ok (()) , & mut | file_path : & Path | { patches . push (file_path . to_path_buf ()) ; Ok (()) } , false ,) ? ; } patches . sort () ; for file_path in patches { println ! ("[GIT] apply `{}`" , file_path . display ()) ; let path = Path :: new ("../../..") . join (file_path) ; run_command_with_output (& [& "git" , & "apply" , & path] , Some (& sysroot_dir)) ? ; run_command_with_output (& [& "git" , & "add" , & "-A"] , Some (& sysroot_dir)) ? ; run_command_with_output (& [& "git" , & "commit" , & "--no-gpg-sign" , & "-m" , & format ! ("Patch {}" , path . display ())] , Some (& sysroot_dir) ,) ? ; } println ! ("Successfully prepared libcore for building") ; Ok (()) }
}

macro_rules! prepare_rand_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function prepare_rand in module {}", module_path!());
    };
}

mkfn!{
    prepare_rand_introspect!();
    fn prepare_rand () -> Result < () , String > { let file_path = "patches/crates/0001-Remove-deny-warnings.patch" ; let rand_dir = Path :: new ("build/rand") ; println ! ("[GIT] apply `{file_path}`") ; let path = Path :: new ("../..") . join (file_path) ; run_command_with_output (& [& "git" , & "apply" , & path] , Some (rand_dir)) ? ; run_command_with_output (& [& "git" , & "add" , & "-A"] , Some (rand_dir)) ? ; run_command_with_output (& [& "git" , & "commit" , & "--no-gpg-sign" , & "-m" , & format ! ("Patch {}" , path . display ())] , Some (rand_dir) ,) ? ; Ok (()) }
}

macro_rules! build_raytracer_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_raytracer in module {}", module_path!());
    };
}

mkfn!{
    build_raytracer_introspect!();
    fn build_raytracer (repo_dir : & Path) -> Result < () , String > { run_command (& [& "cargo" , & "build"] , Some (repo_dir)) ? ; let mv_target = repo_dir . join ("raytracer_cg_llvm") ; if mv_target . is_file () { remove_file (& mv_target) ? ; } run_command (& [& "mv" , & "target/debug/main" , & "raytracer_cg_llvm"] , Some (repo_dir)) ? ; Ok (()) }
}

macro_rules! clone_and_setup_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function clone_and_setup in module {}", module_path!());
    };
}

mkfn!{
    clone_and_setup_introspect!();
    fn clone_and_setup < F > (repo_url : & str , checkout_commit : & str , extra : Option < F >) -> Result < () , String > where F : Fn (& Path) -> Result < () , String > , { let clone_result = git_clone_root_dir (repo_url , Path :: new (crate :: BUILD_DIR) , false) ? ; if ! clone_result . ran_clone { println ! ("`{}` has already been cloned" , clone_result . repo_name) ; } let repo_path = Path :: new (crate :: BUILD_DIR) . join (& clone_result . repo_name) ; run_command (& [& "git" , & "checkout" , & "--" , & "."] , Some (& repo_path)) ? ; run_command (& [& "git" , & "checkout" , & checkout_commit] , Some (& repo_path)) ? ; if let Some (extra) = extra { extra (& repo_path) ? ; } Ok (()) }
}
mkitem!{mkstruct!{struct PrepareArg { cross_compile : bool , only_libcore : bool , libgccjit12_patches : bool , sysroot_source : Option < String > , }}}
mkitem!{mkimpl!{impl PrepareArg { fn new () -> Result < Option < Self > , String > { let mut only_libcore = false ; let mut cross_compile = false ; let mut libgccjit12_patches = false ; let mut sysroot_source = None ; let mut args = std :: env :: args () . skip (2) ; while let Some (arg) = args . next () { match arg . as_str () { "--only-libcore" => only_libcore = true , "--cross" => cross_compile = true , "--libgccjit12-patches" => libgccjit12_patches = true , "--sysroot-source" => { if let Some (path) = args . next () { sysroot_source = Some (path) ; } else { return Err ("Expected a value after `--sysroot-source`, found nothing" . to_string ()) ; } } "--help" => { Self :: usage () ; return Ok (None) ; } a => return Err (format ! ("Unknown argument `{a}`")) , } } Ok (Some (Self { cross_compile , only_libcore , libgccjit12_patches , sysroot_source })) } fn usage () { println ! (r#"
`prepare` command help:

    --only-libcore           : Only setup libcore and don't clone other repositories
    --cross                  : Apply the patches needed to do cross-compilation
    --libgccjit12-patches    : Apply patches needed for libgccjit12
    --sysroot-source         : Specify custom path for sysroot source
    --help                   : Show this help"#) } }}}

macro_rules! run_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function run in module {}", module_path!());
    };
}

mkfn!{
    run_introspect!();
    pub fn run () -> Result < () , String > { let args = match PrepareArg :: new () ? { Some (a) => a , None => return Ok (()) , } ; let sysroot_path = get_sysroot_dir () ; prepare_libcore (& sysroot_path , args . libgccjit12_patches , args . cross_compile , args . sysroot_source ,) ? ; if ! args . only_libcore { cargo_install ("hyperfine") ? ; let to_clone = & [("https://github.com/rust-random/rand.git" , "1f4507a8e1cf8050e4ceef95eeda8f64645b6719" , None ,) , ("https://github.com/rust-lang/regex.git" , "341f207c1071f7290e3f228c710817c280c8dca1" , None ,) , ("https://github.com/ebobby/simple-raytracer" , "804a7a21b9e673a482797aa289a18ed480e4d813" , Some (build_raytracer) ,) ,] ; for (repo_url , checkout_commit , cb) in to_clone { clone_and_setup (repo_url , checkout_commit , * cb) ? ; } prepare_rand () ? ; } println ! ("Successfully ran `prepare`") ; Ok (()) }
}