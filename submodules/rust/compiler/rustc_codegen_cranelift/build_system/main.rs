mkuse!{use std :: path :: PathBuf ;}
mkuse!{use std :: { env , process } ;}
mkuse!{use self :: utils :: Compiler ;}
mkmod!{abi_cafe, { 
                getname!(abi_cafe);
                getsrc!(abi_cafe);
                getpath!(abi_cafe);
                get_deps!(abi_cafe);
                get_crates!(abi_cafe);
                mkinclude!(abi_cafe);
                 
            }}
mkmod!{bench, { 
                getname!(bench);
                getsrc!(bench);
                getpath!(bench);
                get_deps!(bench);
                get_crates!(bench);
                mkinclude!(bench);
                 
            }}
mkmod!{build_backend, { 
                getname!(build_backend);
                getsrc!(build_backend);
                getpath!(build_backend);
                get_deps!(build_backend);
                get_crates!(build_backend);
                mkinclude!(build_backend);
                 
            }}
mkmod!{build_sysroot, { 
                getname!(build_sysroot);
                getsrc!(build_sysroot);
                getpath!(build_sysroot);
                get_deps!(build_sysroot);
                get_crates!(build_sysroot);
                mkinclude!(build_sysroot);
                 
            }}
mkmod!{config, { 
                getname!(config);
                getsrc!(config);
                getpath!(config);
                get_deps!(config);
                get_crates!(config);
                mkinclude!(config);
                 
            }}
mkmod!{path, { 
                getname!(path);
                getsrc!(path);
                getpath!(path);
                get_deps!(path);
                get_crates!(path);
                mkinclude!(path);
                 
            }}
mkmod!{prepare, { 
                getname!(prepare);
                getsrc!(prepare);
                getpath!(prepare);
                get_deps!(prepare);
                get_crates!(prepare);
                mkinclude!(prepare);
                 
            }}
mkmod!{rustc_info, { 
                getname!(rustc_info);
                getsrc!(rustc_info);
                getpath!(rustc_info);
                get_deps!(rustc_info);
                get_crates!(rustc_info);
                mkinclude!(rustc_info);
                 
            }}
mkmod!{shared_utils, { 
                getname!(shared_utils);
                getsrc!(shared_utils);
                getpath!(shared_utils);
                get_deps!(shared_utils);
                get_crates!(shared_utils);
                mkinclude!(shared_utils);
                 
            }}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                 
            }}
mkmod!{utils, { 
                getname!(utils);
                getsrc!(utils);
                getpath!(utils);
                get_deps!(utils);
                get_crates!(utils);
                mkinclude!(utils);
                 
            }}

macro_rules! usage_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function usage in module {}", module_path!());
    };
}

mkfn!{
    usage_introspect!();
    fn usage () { eprintln ! ("{}" , include_str ! ("usage.txt")) ; }
}
mkitem!{macro_rules ! arg_error { ($ ($ err : tt) *) => { { eprintln ! ($ ($ err) *) ; usage () ; std :: process :: exit (1) ; } } ; }}
mkitem!{mkenum!{# [derive (PartialEq , Debug)] enum Command { Prepare , Build , Test , AbiCafe , Bench , }}}
mkitem!{mkenum!{# [derive (Copy , Clone , Debug)] enum SysrootKind { None , Clif , Llvm , }}}
mkitem!{mkenum!{# [derive (Clone , Debug)] enum CodegenBackend { Local (PathBuf) , Builtin (String) , }}}

macro_rules! main_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function main in module {}", module_path!());
    };
}

mkfn!{
    main_introspect!();
    fn main () { if env :: var_os ("RUST_BACKTRACE") . is_none () { env :: set_var ("RUST_BACKTRACE" , "1") ; } env :: set_var ("CG_CLIF_DISABLE_INCR_CACHE" , "1") ; if env :: var_os ("CARGO_BUILD_INCREMENTAL") . is_none () { env :: set_var ("CARGO_BUILD_INCREMENTAL" , "true") ; } let mut args = env :: args () . skip (1) ; let command = match args . next () . as_deref () { Some ("prepare") => Command :: Prepare , Some ("build") => Command :: Build , Some ("test") => Command :: Test , Some ("abi-cafe") => Command :: AbiCafe , Some ("bench") => Command :: Bench , Some (flag) if flag . starts_with ('-') => arg_error ! ("Expected command found flag {}" , flag) , Some (command) => arg_error ! ("Unknown command {}" , command) , None => { usage () ; process :: exit (0) ; } } ; let mut out_dir = PathBuf :: from (".") ; let mut download_dir = None ; let mut sysroot_kind = SysrootKind :: Clif ; let mut use_unstable_features = true ; let mut frozen = false ; let mut skip_tests = vec ! [] ; let mut use_backend = None ; while let Some (arg) = args . next () . as_deref () { match arg { "--out-dir" => { out_dir = PathBuf :: from (args . next () . unwrap_or_else (| | { arg_error ! ("--out-dir requires argument") ; })) ; } "--download-dir" => { download_dir = Some (PathBuf :: from (args . next () . unwrap_or_else (| | { arg_error ! ("--download-dir requires argument") ; }))) ; } "--sysroot" => { sysroot_kind = match args . next () . as_deref () { Some ("none") => SysrootKind :: None , Some ("clif") => SysrootKind :: Clif , Some ("llvm") => SysrootKind :: Llvm , Some (arg) => arg_error ! ("Unknown sysroot kind {}" , arg) , None => arg_error ! ("--sysroot requires argument") , } } "--no-unstable-features" => use_unstable_features = false , "--frozen" => frozen = true , "--skip-test" => { skip_tests . push (args . next () . unwrap_or_else (| | { arg_error ! ("--skip-test requires argument") ; })) ; } "--use-backend" => { use_backend = Some (match args . next () { Some (name) => name , None => arg_error ! ("--use-backend requires argument") , }) ; } flag if flag . starts_with ("-") => arg_error ! ("Unknown flag {}" , flag) , arg => arg_error ! ("Unexpected argument {}" , arg) , } } let current_dir = std :: env :: current_dir () . unwrap () ; out_dir = current_dir . join (out_dir) ; if command == Command :: Prepare { prepare :: prepare (& path :: Dirs { source_dir : current_dir . clone () , download_dir : download_dir . map (| dir | current_dir . join (dir)) . unwrap_or_else (| | out_dir . join ("download")) , build_dir : PathBuf :: from ("dummy_do_not_use") , dist_dir : PathBuf :: from ("dummy_do_not_use") , frozen , }) ; process :: exit (0) ; } let rustup_toolchain_name = match (env :: var ("CARGO") , env :: var ("RUSTC") , env :: var ("RUSTDOC")) { (Ok (_) , Ok (_) , Ok (_)) => None , (_ , Err (_) , Err (_)) => Some (rustc_info :: get_toolchain_name ()) , vars => { eprintln ! ("If RUSTC or RUSTDOC is set, both need to be set and in addition CARGO needs to be set: {vars:?}") ; process :: exit (1) ; } } ; let bootstrap_host_compiler = { let cargo = rustc_info :: get_cargo_path () ; let rustc = rustc_info :: get_rustc_path () ; let rustdoc = rustc_info :: get_rustdoc_path () ; let triple = std :: env :: var ("HOST_TRIPLE") . unwrap_or_else (| _ | rustc_info :: get_host_triple (& rustc)) ; Compiler { cargo , rustc , rustdoc , rustflags : vec ! [] , rustdocflags : vec ! [] , triple , runner : vec ! [] , } } ; let target_triple = std :: env :: var ("TARGET_TRIPLE") . unwrap_or_else (| _ | bootstrap_host_compiler . triple . clone ()) ; let dirs = path :: Dirs { source_dir : current_dir . clone () , download_dir : download_dir . map (| dir | current_dir . join (dir)) . unwrap_or_else (| | out_dir . join ("download")) , build_dir : out_dir . join ("build") , dist_dir : out_dir . join ("dist") , frozen , } ; std :: fs :: create_dir_all (& dirs . build_dir) . unwrap () ; { let target = dirs . build_dir . join ("target_dir_should_be_set_explicitly") ; env :: set_var ("CARGO_TARGET_DIR" , & target) ; let _ = std :: fs :: remove_file (& target) ; std :: fs :: File :: create (target) . unwrap () ; } env :: set_var ("RUSTC" , "rustc_should_be_set_explicitly") ; env :: set_var ("RUSTDOC" , "rustdoc_should_be_set_explicitly") ; let cg_clif_dylib = if let Some (name) = use_backend { CodegenBackend :: Builtin (name) } else { CodegenBackend :: Local (build_backend :: build_backend (& dirs , & bootstrap_host_compiler , use_unstable_features ,)) } ; match command { Command :: Prepare => { } Command :: Test => { tests :: run_tests (& dirs , sysroot_kind , use_unstable_features , & skip_tests . iter () . map (| test | & * * test) . collect :: < Vec < _ > > () , & cg_clif_dylib , & bootstrap_host_compiler , rustup_toolchain_name . as_deref () , target_triple . clone () ,) ; } Command :: AbiCafe => { if bootstrap_host_compiler . triple != target_triple { eprintln ! ("Abi-cafe doesn't support cross-compilation") ; process :: exit (1) ; } abi_cafe :: run (sysroot_kind , & dirs , & cg_clif_dylib , rustup_toolchain_name . as_deref () , & bootstrap_host_compiler ,) ; } Command :: Build => { build_sysroot :: build_sysroot (& dirs , sysroot_kind , & cg_clif_dylib , & bootstrap_host_compiler , rustup_toolchain_name . as_deref () , target_triple ,) ; } Command :: Bench => { let compiler = build_sysroot :: build_sysroot (& dirs , sysroot_kind , & cg_clif_dylib , & bootstrap_host_compiler , rustup_toolchain_name . as_deref () , target_triple ,) ; bench :: benchmark (& dirs , & compiler) ; } } }
}