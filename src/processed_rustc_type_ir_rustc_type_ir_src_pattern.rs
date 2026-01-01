/* FP:pattern.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_type_ir_src_pattern_USE_0001
/* FP:pattern.rs-0002 */ use derive_where :: derive_where ;
/* FP:pattern.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_type_ir_src_pattern_USE_0002
/* FP:pattern.rs-0004 */ # [cfg (feature = "nightly")] use rustc_macros :: { Decodable_NoContext , Encodable_NoContext , HashStable_NoContext } ;
/* FP:pattern.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_type_ir_src_pattern_USE_0003
/* FP:pattern.rs-0006 */ use rustc_type_ir_macros :: { Lift_Generic , TypeFoldable_Generic , TypeVisitable_Generic } ;
/* FP:pattern.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_type_ir_src_pattern_USE_0004
/* FP:pattern.rs-0008 */ use crate :: Interner ;
/* FP:pattern.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_type_ir_src_pattern_ENUM_0005
/* FP:pattern.rs-0010 */ # [derive_where (Clone , Copy , Hash , PartialEq ; I : Interner)] # [derive (TypeVisitable_Generic , TypeFoldable_Generic , Lift_Generic)] # [cfg_attr (feature = "nightly" , derive (Decodable_NoContext , Encodable_NoContext , HashStable_NoContext))] pub enum PatternKind < I : Interner > { Range { start : I :: Const , end : I :: Const } , Or (I :: PatList) , }
/* FP:pattern.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_type_ir_src_pattern_IMPL_0006
/* FP:pattern.rs-0012 */ impl < I : Interner > Eq for PatternKind < I > { }