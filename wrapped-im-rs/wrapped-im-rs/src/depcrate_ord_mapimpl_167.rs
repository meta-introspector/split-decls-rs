// Generated macro for impl_167 (impl)
macro_rules! Depcrate_ord_mapimpl_167 {
() => {
// Module: crate::ord::map
// Provides: {"impl_167"}
// Dependencies: {}
# [cfg (not (has_specialisation))] impl < K : Ord , V > BTreeValue for (K , V) { type Key = K ; fn ptr_eq (& self , _other : & Self) -> bool { false } fn search_key < BK > (slice : & [Self] , key : & BK) -> Result < usize , usize > where BK : Ord + ? Sized , Self :: Key : Borrow < BK > , { slice . binary_search_by (| value | Self :: Key :: borrow (& value . 0) . cmp (key)) } fn search_value (slice : & [Self] , key : & Self) -> Result < usize , usize > { slice . binary_search_by (| value | value . 0 . cmp (& key . 0)) } fn cmp_keys < BK > (& self , other : & BK) -> Ordering where BK : Ord + ? Sized , Self :: Key : Borrow < BK > , { Self :: Key :: borrow (& self . 0) . cmp (other) } fn cmp_values (& self , other : & Self) -> Ordering { self . 0 . cmp (& other . 0) } }
};
}
