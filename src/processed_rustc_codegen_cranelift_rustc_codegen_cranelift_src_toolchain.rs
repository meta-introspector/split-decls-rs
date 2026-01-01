/* FP:toolchain.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_src_toolchain_USE_0001
/* FP:toolchain.rs-0002 */ use std :: path :: PathBuf ;
/* FP:toolchain.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_src_toolchain_USE_0002
/* FP:toolchain.rs-0004 */ use crate :: rustc_codegen_ssa :: back :: link :: linker_and_flavor ;
/* FP:toolchain.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_src_toolchain_USE_0003
/* FP:toolchain.rs-0006 */ use crate :: rustc_complete :: Session ;
/* FP:toolchain.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_src_toolchain_FN_0004
/* FP:toolchain.rs-0008 */ # [doc = " Tries to infer the path of a binary for the target toolchain from the linker name."] pub (crate) fn get_toolchain_binary (sess : & Session , tool : & str) -> PathBuf { let (mut linker , _linker_flavor) = linker_and_flavor (sess) ; let linker_file_name = linker . file_name () . unwrap () . to_str () . expect ("linker filename should be valid UTF-8") ; if linker_file_name == "ld.lld" { if tool != "ld" { linker . set_file_name (tool) } } else { let tool_file_name = linker_file_name . replace ("ld" , tool) . replace ("gcc" , tool) . replace ("clang" , tool) . replace ("cc" , tool) ; linker . set_file_name (tool_file_name) } linker }