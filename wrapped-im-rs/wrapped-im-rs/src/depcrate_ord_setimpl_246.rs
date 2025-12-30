// Generated macro for impl_246 (impl)
macro_rules! Depcrate_ord_setimpl_246 {
() => {
// Module: crate::ord::set
// Provides: {"impl_246"}
// Dependencies: {}
# [cfg (has_specialisation)] impl < A : Ord + Copy > BTreeValue for Value < A > { fn search_key < BK > (slice : & [Self] , key : & BK) -> Result < usize , usize > where BK : Ord + ? Sized , Self :: Key : Borrow < BK > , { linear_search_by (slice , | value | Self :: Key :: borrow (value) . cmp (key)) } fn search_value (slice : & [Self] , key : & Self) -> Result < usize , usize > { linear_search_by (slice , | value | value . cmp (key)) } }
};
}
