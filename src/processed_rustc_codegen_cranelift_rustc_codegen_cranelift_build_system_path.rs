/* FP:path.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_build_system_path_USE_0001
/* FP:path.rs-0002 */ use std :: path :: PathBuf ;
/* FP:path.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_build_system_path_STRUCT_0002
/* FP:path.rs-0004 */ # [derive (Debug , Clone)] pub (crate) struct Dirs { pub (crate) source_dir : PathBuf , pub (crate) download_dir : PathBuf , pub (crate) build_dir : PathBuf , pub (crate) dist_dir : PathBuf , pub (crate) frozen : bool , }
/* FP:path.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_build_system_path_ENUM_0003
/* FP:path.rs-0006 */ # [doc (hidden)] # [derive (Debug , Copy , Clone)] enum PathBase { Source , Build , }
/* FP:path.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_build_system_path_STRUCT_0004
/* FP:path.rs-0008 */ # [derive (Debug , Copy , Clone)] pub (crate) struct RelPath { base : PathBase , suffix : & 'static str , }
/* FP:path.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_build_system_path_IMPL_0005
/* FP:path.rs-0010 */ impl RelPath { pub (crate) const fn source (suffix : & 'static str) -> RelPath { RelPath { base : PathBase :: Source , suffix } } pub (crate) const fn build (suffix : & 'static str) -> RelPath { RelPath { base : PathBase :: Build , suffix } } pub (crate) fn to_path (& self , dirs : & Dirs) -> PathBuf { match self . base { PathBase :: Source => dirs . source_dir . join (self . suffix) , PathBase :: Build => dirs . build_dir . join (self . suffix) , } } }