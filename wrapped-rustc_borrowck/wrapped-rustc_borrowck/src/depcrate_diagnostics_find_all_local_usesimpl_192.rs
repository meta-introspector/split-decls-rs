// Generated macro for impl_192 (impl)
macro_rules! Depcrate_diagnostics_find_all_local_usesimpl_192 {
() => {
// Module: crate::diagnostics::find_all_local_uses
// Provides: {"impl_192"}
// Dependencies: {}
impl < 'tcx > Visitor < 'tcx > for AllLocalUsesVisitor { fn visit_local (& mut self , local : Local , _context : PlaceContext , location : Location) { if local == self . for_local { self . uses . insert (location) ; } } }
};
}
