// Generated macro for impl_931 (impl)
macro_rules! Depcrate_mir_localsimpl_931 {
() => {
// Module: crate::mir::locals
// Provides: {"impl_931"}
// Dependencies: {}
impl < 'tcx , V > Locals < 'tcx , V > { pub (super) fn empty () -> Locals < 'tcx , V > { Locals { values : IndexVec :: default () } } pub (super) fn indices (& self) -> impl DoubleEndedIterator < Item = mir :: Local > + Clone + 'tcx { self . values . indices () } }
};
}
