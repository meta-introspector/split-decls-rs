// SRC: ../rust/compiler/rustc_codegen_gcc/src/coverageinfo.rs
use crate::rustc_codegen_ssa::traits::CoverageInfoBuilderMethods;
use crate::rustc_complete::mir::coverage::CoverageKind;
use crate::rustc_complete::ty::Instance;

use crate::builder::Builder;

impl<'a, 'gcc, 'tcx> CoverageInfoBuilderMethods<'tcx> for Builder<'a, 'gcc, 'tcx> {
    fn add_coverage(&mut self, _instance: Instance<'tcx>, _kind: &CoverageKind) {
        // TODO(antoyo)
    }
}