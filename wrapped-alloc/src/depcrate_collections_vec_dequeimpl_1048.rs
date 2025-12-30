// Generated macro for impl_1048 (impl)
macro_rules! Depcrate_collections_vec_dequeimpl_1048 {
() => {
// Module: crate::collections::vec_deque
// Provides: {"impl_1048"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T : PartialEq , A : Allocator > PartialEq for VecDeque < T , A > { fn eq (& self , other : & Self) -> bool { if self . len != other . len () { return false ; } let (sa , sb) = self . as_slices () ; let (oa , ob) = other . as_slices () ; if sa . len () == oa . len () { sa == oa && sb == ob } else if sa . len () < oa . len () { let front = sa . len () ; let mid = oa . len () - front ; let (oa_front , oa_mid) = oa . split_at (front) ; let (sb_mid , sb_back) = sb . split_at (mid) ; debug_assert_eq ! (sa . len () , oa_front . len ()) ; debug_assert_eq ! (sb_mid . len () , oa_mid . len ()) ; debug_assert_eq ! (sb_back . len () , ob . len ()) ; sa == oa_front && sb_mid == oa_mid && sb_back == ob } else { let front = oa . len () ; let mid = sa . len () - front ; let (sa_front , sa_mid) = sa . split_at (front) ; let (ob_mid , ob_back) = ob . split_at (mid) ; debug_assert_eq ! (sa_front . len () , oa . len ()) ; debug_assert_eq ! (sa_mid . len () , ob_mid . len ()) ; debug_assert_eq ! (sb . len () , ob_back . len ()) ; sa_front == oa && sa_mid == ob_mid && sb == ob_back } } }
};
}
