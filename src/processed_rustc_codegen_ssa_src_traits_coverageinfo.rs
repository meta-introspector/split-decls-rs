/* FP:coverageinfo.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_coverageinfo_USE_0001
/* FP:coverageinfo.rs-0002 */ use crate :: rustc_complete :: mir :: coverage :: CoverageKind ;
/* FP:coverageinfo.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_coverageinfo_USE_0002
/* FP:coverageinfo.rs-0004 */ use crate :: rustc_complete :: ty :: Instance ;
/* FP:coverageinfo.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_coverageinfo_TRAIT_0003
/* FP:coverageinfo.rs-0006 */ pub trait CoverageInfoBuilderMethods < 'tcx > { # [doc = " Handle the MIR coverage info in a backend-specific way."] # [doc = ""] # [doc = " This can potentially be a no-op in backends that don't support"] # [doc = " coverage instrumentation."] fn add_coverage (& mut self , instance : Instance < 'tcx > , kind : & CoverageKind) ; }