// Generated macro for impl_169 (impl)
macro_rules! Depcrate_ord_mapimpl_169 {
() => {
// Module: crate::ord::map
// Provides: {"impl_169"}
// Dependencies: {}
# [cfg (has_specialisation)] impl < K : Ord + Copy , V > BTreeValue for (K , V) { fn search_key < BK > (slice : & [Self] , key : & BK) -> Result < usize , usize > where BK : Ord + ? Sized , Self :: Key : Borrow < BK > , { linear_search_by (slice , | value | Self :: Key :: borrow (& value . 0) . cmp (key)) } fn search_value (slice : & [Self] , key : & Self) -> Result < usize , usize > { linear_search_by (slice , | value | value . 0 . cmp (& key . 0)) } }
};
}
