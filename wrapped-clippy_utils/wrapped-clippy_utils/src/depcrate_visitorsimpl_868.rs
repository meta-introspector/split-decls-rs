// Generated macro for impl_868 (impl)
macro_rules! Depcrate_visitorsimpl_868 {
() => {
// Module: crate::visitors
// Provides: {"impl_868"}
// Dependencies: {}
impl < 'tcx , T > Visitable < 'tcx > for & 'tcx [T] where & 'tcx T : Visitable < 'tcx > , { fn visit < V : Visitor < 'tcx > > (self , visitor : & mut V) -> V :: Result { for x in self { try_visit ! (x . visit (visitor)) ; } V :: Result :: output () } }
};
}
