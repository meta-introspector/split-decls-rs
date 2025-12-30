// Generated macro for impl_870 (impl)
macro_rules! Depcrate_visitorsimpl_870 {
() => {
// Module: crate::visitors
// Provides: {"impl_870"}
// Dependencies: {}
impl < 'tcx , T > Visitable < 'tcx > for Option < T > where T : Visitable < 'tcx > , { fn visit < V : Visitor < 'tcx > > (self , visitor : & mut V) -> V :: Result { if let Some (x) = self { try_visit ! (x . visit (visitor)) ; } V :: Result :: output () } }
};
}
