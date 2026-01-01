/* FP:build_backend.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_build_system_build_backend_USE_0001
/* FP:build_backend.rs-0002 */ use std :: env ;
/* FP:build_backend.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_build_system_build_backend_USE_0002
/* FP:build_backend.rs-0004 */ use std :: path :: PathBuf ;
/* FP:build_backend.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_build_system_build_backend_USE_0003
/* FP:build_backend.rs-0006 */ use crate :: path :: { Dirs , RelPath } ;
/* FP:build_backend.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_build_system_build_backend_USE_0004
/* FP:build_backend.rs-0008 */ use crate :: rustc_info :: get_file_name ;
/* FP:build_backend.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_build_system_build_backend_USE_0005
/* FP:build_backend.rs-0010 */ use crate :: shared_utils :: { rustflags_from_env , rustflags_to_cmd_env } ;
/* FP:build_backend.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_build_system_build_backend_USE_0006
/* FP:build_backend.rs-0012 */ use crate :: utils :: { CargoProject , Compiler , LogGroup } ;
/* FP:build_backend.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_build_system_build_backend_STATIC_0007
/* FP:build_backend.rs-0014 */ static CG_CLIF : CargoProject = CargoProject :: new (& RelPath :: source (".") , "cg_clif") ;
/* FP:build_backend.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_build_system_build_backend_FN_0008
/* FP:build_backend.rs-0016 */ pub (crate) fn build_backend (dirs : & Dirs , bootstrap_host_compiler : & Compiler , use_unstable_features : bool ,) -> PathBuf { let _group = LogGroup :: guard ("Build backend") ; let mut cmd = CG_CLIF . build (& bootstrap_host_compiler , dirs) ; let mut rustflags = rustflags_from_env ("RUSTFLAGS") ; rustflags . push ("-Zallow-features=rustc_private,f16,f128" . to_owned ()) ; rustflags_to_cmd_env (& mut cmd , "RUSTFLAGS" , & rustflags) ; if env :: var ("CG_CLIF_EXPENSIVE_CHECKS") . is_ok () { cmd . env ("CARGO_PROFILE_RELEASE_DEBUG_ASSERTIONS" , "true") ; cmd . env ("CARGO_PROFILE_RELEASE_OVERFLOW_CHECKS" , "true") ; } if use_unstable_features { cmd . arg ("--features") . arg ("unstable-features") ; } cmd . arg ("--release") ; eprintln ! ("[BUILD] rustc_codegen_cranelift") ; crate :: utils :: spawn_and_wait (cmd) ; CG_CLIF . target_dir (dirs) . join (& bootstrap_host_compiler . triple) . join ("release") . join (get_file_name (& bootstrap_host_compiler . rustc , "rustc_codegen_cranelift" , "dylib")) }