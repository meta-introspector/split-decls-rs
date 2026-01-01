/* FP:temp_dir.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_temp_dir_USE_0001
/* FP:temp_dir.rs-0002 */ use std :: mem :: ManuallyDrop ;
/* FP:temp_dir.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_temp_dir_USE_0002
/* FP:temp_dir.rs-0004 */ use std :: path :: Path ;
/* FP:temp_dir.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_temp_dir_USE_0003
/* FP:temp_dir.rs-0006 */ use tempfile :: TempDir ;
/* FP:temp_dir.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_temp_dir_STRUCT_0004
/* FP:temp_dir.rs-0008 */ # [doc = " This is used to avoid TempDir being dropped on error paths unintentionally."] # [derive (Debug)] pub struct MaybeTempDir { dir : ManuallyDrop < TempDir > , keep : bool , }
/* FP:temp_dir.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_temp_dir_IMPL_0005
/* FP:temp_dir.rs-0010 */ impl Drop for MaybeTempDir { fn drop (& mut self) { let dir = unsafe { ManuallyDrop :: take (& mut self . dir) } ; if self . keep { let _ = dir . keep () ; } } }
/* FP:temp_dir.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_temp_dir_IMPL_0006
/* FP:temp_dir.rs-0012 */ impl AsRef < Path > for MaybeTempDir { fn as_ref (& self) -> & Path { self . dir . path () } }
/* FP:temp_dir.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_temp_dir_IMPL_0007
/* FP:temp_dir.rs-0014 */ impl MaybeTempDir { pub fn new (dir : TempDir , keep_on_drop : bool) -> MaybeTempDir { MaybeTempDir { dir : ManuallyDrop :: new (dir) , keep : keep_on_drop } } }