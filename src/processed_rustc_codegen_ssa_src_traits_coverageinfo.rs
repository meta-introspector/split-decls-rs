use crate::rustc_complete::mir::coverage::CoverageKind;
use crate::rustc_complete::ty::Instance;

pub trait CoverageInfoBuilderMethods<'tcx> {
    /// Handle the MIR coverage info in a backend-specific way.
    ///
    /// This can potentially be a no-op in backends that don't support
    /// coverage instrumentation.
    fn add_coverage(&mut self, instance: Instance<'tcx>, kind: &CoverageKind);
}