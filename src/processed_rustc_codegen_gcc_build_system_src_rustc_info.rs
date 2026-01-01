/* FP:rustc_info.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_build_system_src_rustc_info_USE_0001
/* FP:rustc_info.rs-0002 */ use std :: path :: { Path , PathBuf } ;
/* FP:rustc_info.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_build_system_src_rustc_info_USE_0002
/* FP:rustc_info.rs-0004 */ use crate :: utils :: run_command ;
/* FP:rustc_info.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_build_system_src_rustc_info_FN_0003
/* FP:rustc_info.rs-0006 */ pub fn get_rustc_path () -> Option < PathBuf > { if let Ok (rustc) = std :: env :: var ("RUSTC") { return Some (PathBuf :: from (rustc)) ; } run_command (& [& "rustup" , & "which" , & "rustc"] , None) . ok () . map (| out | Path :: new (String :: from_utf8 (out . stdout) . unwrap () . trim ()) . to_path_buf ()) }