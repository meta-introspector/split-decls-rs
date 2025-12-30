// Generated macro for impl_486 (impl)
macro_rules! Depcrate_merge_joinimpl_486 {
() => {
// Module: crate::merge_join
// Provides: {"impl_486"}
// Dependencies: {}
impl < L , R , F : FnMut (& L , & R) -> bool > OrderingOrBool < L , R > for MergeFuncLR < F , bool > { type MergeResult = Either < L , R > ; fn left (left : L) -> Self :: MergeResult { Either :: Left (left) } fn right (right : R) -> Self :: MergeResult { Either :: Right (right) } fn merge (& mut self , left : L , right : R) -> (Option < Either < L , R > > , Self :: MergeResult) { if self . 0 (& left , & right) { (Some (Either :: Right (right)) , Either :: Left (left)) } else { (Some (Either :: Left (left)) , Either :: Right (right)) } } fn size_hint (left : SizeHint , right : SizeHint) -> SizeHint { size_hint :: add (left , right) } }
};
}
