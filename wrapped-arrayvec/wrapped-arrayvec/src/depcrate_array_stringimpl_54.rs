// Generated macro for impl_54 (impl)
macro_rules! Depcrate_array_stringimpl_54 {
() => {
// Module: crate::array_string
// Provides: {"impl_54"}
// Dependencies: {}
impl < 'a , const CAP : usize > TryFrom < fmt :: Arguments < 'a > > for ArrayString < CAP > { type Error = CapacityError < fmt :: Error > ; fn try_from (f : fmt :: Arguments < 'a >) -> Result < Self , Self :: Error > { use fmt :: Write ; let mut v = Self :: new () ; v . write_fmt (f) . map_err (| e | CapacityError :: new (e)) ? ; Ok (v) } }
};
}
