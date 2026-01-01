/* FP:generic_arg.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_type_ir_src_generic_arg_USE_0001
/* FP:generic_arg.rs-0002 */ use derive_where :: derive_where ;
/* FP:generic_arg.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_type_ir_src_generic_arg_USE_0002
/* FP:generic_arg.rs-0004 */ # [cfg (feature = "nightly")] use rustc_macros :: { Decodable_NoContext , Encodable_NoContext , HashStable_NoContext } ;
/* FP:generic_arg.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_type_ir_src_generic_arg_USE_0003
/* FP:generic_arg.rs-0006 */ use crate :: Interner ;
/* FP:generic_arg.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_type_ir_src_generic_arg_ENUM_0004
/* FP:generic_arg.rs-0008 */ # [derive_where (Clone , Copy , PartialEq , Debug ; I : Interner)] # [cfg_attr (feature = "nightly" , derive (Decodable_NoContext , Encodable_NoContext , HashStable_NoContext))] pub enum GenericArgKind < I : Interner > { Lifetime (I :: Region) , Type (I :: Ty) , Const (I :: Const) , }
/* FP:generic_arg.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_type_ir_src_generic_arg_IMPL_0005
/* FP:generic_arg.rs-0010 */ impl < I : Interner > Eq for GenericArgKind < I > { }
/* FP:generic_arg.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_type_ir_src_generic_arg_ENUM_0006
/* FP:generic_arg.rs-0012 */ # [derive_where (Clone , Copy , PartialEq , Debug ; I : Interner)] # [cfg_attr (feature = "nightly" , derive (Decodable_NoContext , Encodable_NoContext , HashStable_NoContext))] pub enum TermKind < I : Interner > { Ty (I :: Ty) , Const (I :: Const) , }
/* FP:generic_arg.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_type_ir_src_generic_arg_IMPL_0007
/* FP:generic_arg.rs-0014 */ impl < I : Interner > Eq for TermKind < I > { }