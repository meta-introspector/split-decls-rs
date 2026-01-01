/* FP:coverageinfo.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_src_coverageinfo_USE_0001
/* FP:coverageinfo.rs-0002 */ use crate :: rustc_codegen_ssa :: traits :: CoverageInfoBuilderMethods ;
/* FP:coverageinfo.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_src_coverageinfo_USE_0002
/* FP:coverageinfo.rs-0004 */ use crate :: rustc_complete :: mir :: coverage :: CoverageKind ;
/* FP:coverageinfo.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_src_coverageinfo_USE_0003
/* FP:coverageinfo.rs-0006 */ use crate :: rustc_complete :: ty :: Instance ;
/* FP:coverageinfo.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_src_coverageinfo_USE_0004
/* FP:coverageinfo.rs-0008 */ use crate :: builder :: Builder ;
/* FP:coverageinfo.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_src_coverageinfo_IMPL_0005
/* FP:coverageinfo.rs-0010 */ impl < 'a , 'gcc , 'tcx > CoverageInfoBuilderMethods < 'tcx > for Builder < 'a , 'gcc , 'tcx > { fn add_coverage (& mut self , _instance : Instance < 'tcx > , _kind : & CoverageKind) { } }