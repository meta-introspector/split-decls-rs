/* FP:coverageinfo.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_mir_coverageinfo_USE_0001
/* FP:coverageinfo.rs-0002 */ use crate :: rustc_complete :: mir :: SourceScope ;
/* FP:coverageinfo.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_mir_coverageinfo_USE_0002
/* FP:coverageinfo.rs-0004 */ use crate :: rustc_complete :: mir :: coverage :: CoverageKind ;
/* FP:coverageinfo.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_mir_coverageinfo_USE_0003
/* FP:coverageinfo.rs-0006 */ use super :: FunctionCx ;
/* FP:coverageinfo.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_mir_coverageinfo_USE_0004
/* FP:coverageinfo.rs-0008 */ use crate :: traits :: * ;
/* FP:coverageinfo.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_mir_coverageinfo_IMPL_0005
/* FP:coverageinfo.rs-0010 */ impl < 'a , 'tcx , Bx : BuilderMethods < 'a , 'tcx > > FunctionCx < 'a , 'tcx , Bx > { pub (crate) fn codegen_coverage (& self , bx : & mut Bx , kind : & CoverageKind , scope : SourceScope) { let instance = if let Some (inlined) = scope . inlined_instance (& self . mir . source_scopes) { self . monomorphize (inlined) } else { self . instance } ; bx . add_coverage (instance , kind) ; } }