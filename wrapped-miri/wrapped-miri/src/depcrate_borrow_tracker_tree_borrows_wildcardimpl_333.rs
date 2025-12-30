// Generated macro for impl_333 (impl)
macro_rules! Depcrate_borrow_tracker_tree_borrows_wildcardimpl_333 {
() => {
// Module: crate::borrow_tracker::tree_borrows::wildcard
// Provides: {"impl_333"}
// Dependencies: {}
impl WildcardAccessRelatedness { pub fn to_relatedness (self) -> Option < AccessRelatedness > { match self { Self :: LocalAccess => Some (AccessRelatedness :: LocalAccess) , Self :: ForeignAccess => Some (AccessRelatedness :: ForeignAccess) , Self :: EitherAccess => None , } } }
};
}
