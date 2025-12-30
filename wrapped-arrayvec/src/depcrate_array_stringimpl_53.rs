// Generated macro for impl_53 (impl)
macro_rules! Depcrate_array_stringimpl_53 {
() => {
// Module: crate::array_string
// Provides: {"impl_53"}
// Dependencies: {}
impl < 'a , const CAP : usize > TryFrom < & 'a str > for ArrayString < CAP > { type Error = CapacityError < & 'a str > ; fn try_from (f : & 'a str) -> Result < Self , Self :: Error > { let mut v = Self :: new () ; v . try_push_str (f) ? ; Ok (v) } }
};
}
