// Generated macro for CoverageInfoBuilderMethods (trait)
macro_rules! Depcrate_traits_coverageinfoCoverageInfoBuilderMethods {
() => {
// Module: crate::traits::coverageinfo
// Provides: {"CoverageInfoBuilderMethods"}
// Dependencies: {}
pub trait CoverageInfoBuilderMethods < 'tcx > { # [doc = " Handle the MIR coverage info in a backend-specific way."] # [doc = ""] # [doc = " This can potentially be a no-op in backends that don't support"] # [doc = " coverage instrumentation."] fn add_coverage (& mut self , instance : Instance < 'tcx > , kind : & CoverageKind) ; }
};
}
