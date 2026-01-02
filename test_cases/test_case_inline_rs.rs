// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_mir_transform/src/inline.rs
// Error: expected square brackets
// Problematic line: line 33

const HISTORY_DEPTH_LIMIT: usize = 20;
const TOP_DOWN_DEPTH_LIMIT: usize = 5;

#[derive(Clone, Debug)]
struct CallSite<'tcx> {
    callee: Instance<'tcx>,
    fn_sig: ty::PolyFnSig<'tcx>,
