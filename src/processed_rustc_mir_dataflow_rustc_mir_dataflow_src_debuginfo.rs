/* FP:debuginfo.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_dataflow_src_debuginfo_USE_0001
/* FP:debuginfo.rs-0002 */ use crate :: rustc_index :: bit_set :: DenseBitSet ;
/* FP:debuginfo.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_dataflow_src_debuginfo_USE_0002
/* FP:debuginfo.rs-0004 */ use crate :: rustc_complete :: mir :: visit :: * ;
/* FP:debuginfo.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_dataflow_src_debuginfo_USE_0003
/* FP:debuginfo.rs-0006 */ use crate :: rustc_complete :: mir :: * ;
/* FP:debuginfo.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_dataflow_src_debuginfo_FN_0004
/* FP:debuginfo.rs-0008 */ # [doc = " Return the set of locals that appear in debuginfo."] pub fn debuginfo_locals (body : & Body < '_ >) -> DenseBitSet < Local > { let mut visitor = DebuginfoLocals (DenseBitSet :: new_empty (body . local_decls . len ())) ; for debuginfo in body . var_debug_info . iter () { visitor . visit_var_debug_info (debuginfo) ; } visitor . 0 }
/* FP:debuginfo.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_dataflow_src_debuginfo_STRUCT_0005
/* FP:debuginfo.rs-0010 */ struct DebuginfoLocals (DenseBitSet < Local >) ;
/* FP:debuginfo.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_dataflow_src_debuginfo_IMPL_0006
/* FP:debuginfo.rs-0012 */ impl Visitor < '_ > for DebuginfoLocals { fn visit_local (& mut self , local : Local , _ : PlaceContext , _ : Location) { self . 0 . insert (local) ; } }