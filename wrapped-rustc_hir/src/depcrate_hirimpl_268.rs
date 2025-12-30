// Generated macro for impl_268 (impl)
macro_rules! Depcrate_hirimpl_268 {
() => {
// Module: crate::hir
// Provides: {"impl_268"}
// Dependencies: {}
impl < 'tcx > MaybeOwner < 'tcx > { pub fn as_owner (self) -> Option < & 'tcx OwnerInfo < 'tcx > > { match self { MaybeOwner :: Owner (i) => Some (i) , MaybeOwner :: NonOwner (_) | MaybeOwner :: Phantom => None , } } pub fn unwrap (self) -> & 'tcx OwnerInfo < 'tcx > { self . as_owner () . unwrap_or_else (| | panic ! ("Not a HIR owner")) } }
};
}
