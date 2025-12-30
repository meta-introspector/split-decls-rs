// Generated macro for impl_485 (impl)
macro_rules! Depcrate_merge_joinimpl_485 {
() => {
// Module: crate::merge_join
// Provides: {"impl_485"}
// Dependencies: {}
impl < L , R , F : FnMut (& L , & R) -> Ordering > OrderingOrBool < L , R > for MergeFuncLR < F , Ordering > { type MergeResult = EitherOrBoth < L , R > ; fn left (left : L) -> Self :: MergeResult { EitherOrBoth :: Left (left) } fn right (right : R) -> Self :: MergeResult { EitherOrBoth :: Right (right) } fn merge (& mut self , left : L , right : R) -> (Option < Either < L , R > > , Self :: MergeResult) { match self . 0 (& left , & right) { Ordering :: Equal => (None , EitherOrBoth :: Both (left , right)) , Ordering :: Less => (Some (Either :: Right (right)) , EitherOrBoth :: Left (left)) , Ordering :: Greater => (Some (Either :: Left (left)) , EitherOrBoth :: Right (right)) , } } fn size_hint (left : SizeHint , right : SizeHint) -> SizeHint { let (a_lower , a_upper) = left ; let (b_lower , b_upper) = right ; let lower = :: std :: cmp :: max (a_lower , b_lower) ; let upper = match (a_upper , b_upper) { (Some (x) , Some (y)) => x . checked_add (y) , _ => None , } ; (lower , upper) } }
};
}
