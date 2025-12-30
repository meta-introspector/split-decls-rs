// Generated macro for impl_897 (impl)
macro_rules! Depcrate_visitorsimpl_897 {
() => {
// Module: crate::visitors
// Provides: {"impl_897"}
// Dependencies: {}
impl < 'tcx , A , B > Visitable < 'tcx > for (A , B) where A : Visitable < 'tcx > , B : Visitable < 'tcx > , { fn visit < V : Visitor < 'tcx > > (self , visitor : & mut V) -> V :: Result { let (a , b) = self ; try_visit ! (a . visit (visitor)) ; b . visit (visitor) } }
};
}
