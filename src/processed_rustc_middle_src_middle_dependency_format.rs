/* FP:dependency_format.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_middle_dependency_format_USE_0001
/* FP:dependency_format.rs-0002 */ use crate :: rustc_data_structures :: fx :: FxIndexMap ;
/* FP:dependency_format.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_middle_dependency_format_USE_0002
/* FP:dependency_format.rs-0004 */ use crate :: rustc_complete :: def_id :: CrateNum ;
/* FP:dependency_format.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_middle_dependency_format_USE_0003
/* FP:dependency_format.rs-0006 */ use crate :: rustc_index :: IndexVec ;
/* FP:dependency_format.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_middle_dependency_format_USE_0004
/* FP:dependency_format.rs-0008 */ use rustc_macros :: { Decodable , Encodable , HashStable } ;
/* FP:dependency_format.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_middle_dependency_format_USE_0005
/* FP:dependency_format.rs-0010 */ use crate :: rustc_complete :: config :: CrateType ;
/* FP:dependency_format.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_middle_dependency_format_TYPE_0006
/* FP:dependency_format.rs-0012 */ # [doc = " A list of dependencies for a certain crate type."] pub type DependencyList = IndexVec < CrateNum , Linkage > ;
/* FP:dependency_format.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_middle_dependency_format_TYPE_0007
/* FP:dependency_format.rs-0014 */ # [doc = " A mapping of all required dependencies for a particular flavor of output."] # [doc = ""] # [doc = " This is local to the tcx, and is generally relevant to one session."] pub type Dependencies = FxIndexMap < CrateType , DependencyList > ;
/* FP:dependency_format.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_middle_dependency_format_ENUM_0008
/* FP:dependency_format.rs-0016 */ # [derive (Copy , Clone , PartialEq , Debug , HashStable , Encodable , Decodable)] pub enum Linkage { NotLinked , IncludedFromDylib , Static , Dynamic , }