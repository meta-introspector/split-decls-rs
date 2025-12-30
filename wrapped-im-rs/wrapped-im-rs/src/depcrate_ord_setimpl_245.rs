// Generated macro for impl_245 (impl)
macro_rules! Depcrate_ord_setimpl_245 {
() => {
// Module: crate::ord::set
// Provides: {"impl_245"}
// Dependencies: {}
# [cfg (has_specialisation)] impl < A : Ord > BTreeValue for Value < A > { type Key = A ; fn ptr_eq (& self , _other : & Self) -> bool { false } default fn search_key < BK > (slice : & [Self] , key : & BK) -> Result < usize , usize > where BK : Ord + ? Sized , Self :: Key : Borrow < BK > , { slice . binary_search_by (| value | Self :: Key :: borrow (value) . cmp (key)) } default fn search_value (slice : & [Self] , key : & Self) -> Result < usize , usize > { slice . binary_search_by (| value | value . cmp (key)) } fn cmp_keys < BK > (& self , other : & BK) -> Ordering where BK : Ord + ? Sized , Self :: Key : Borrow < BK > , { Self :: Key :: borrow (self) . cmp (other) } fn cmp_values (& self , other : & Self) -> Ordering { self . cmp (other) } }
};
}
