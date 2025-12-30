// Generated macro for impl_398 (impl)
macro_rules! Depcrate_interpret_callimpl_398 {
() => {
// Module: crate::interpret::call
// Provides: {"impl_398"}
// Dependencies: {}
impl < 'tcx , Prov : Provenance > FnArg < 'tcx , Prov > { pub fn layout (& self) -> & TyAndLayout < 'tcx > { match self { FnArg :: Copy (op) => & op . layout , FnArg :: InPlace (mplace) => & mplace . layout , } } }
};
}
