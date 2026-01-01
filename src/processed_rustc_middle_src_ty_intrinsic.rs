/* FP:intrinsic.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_intrinsic_USE_0001
/* FP:intrinsic.rs-0002 */ use rustc_macros :: { Decodable , Encodable , HashStable } ;
/* FP:intrinsic.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_intrinsic_USE_0002
/* FP:intrinsic.rs-0004 */ use crate :: rustc_complete :: Symbol ;
/* FP:intrinsic.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_intrinsic_USE_0003
/* FP:intrinsic.rs-0006 */ use crate :: rustc_complete :: def_id :: DefId ;
/* FP:intrinsic.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_intrinsic_USE_0004
/* FP:intrinsic.rs-0008 */ use super :: TyCtxt ;
/* FP:intrinsic.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_intrinsic_STRUCT_0005
/* FP:intrinsic.rs-0010 */ # [derive (Copy , Clone , Debug , Decodable , Encodable , HashStable)] pub struct IntrinsicDef { pub name : Symbol , # [doc = " Whether the intrinsic has no meaningful body and all backends need to shim all calls to it."] pub must_be_overridden : bool , # [doc = " Whether the intrinsic can be invoked from stable const fn"] pub const_stable : bool , }
/* FP:intrinsic.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_intrinsic_IMPL_0006
/* FP:intrinsic.rs-0012 */ impl TyCtxt < '_ > { pub fn is_intrinsic (self , def_id : DefId , name : Symbol) -> bool { let Some (i) = self . intrinsic (def_id) else { return false } ; i . name == name } }