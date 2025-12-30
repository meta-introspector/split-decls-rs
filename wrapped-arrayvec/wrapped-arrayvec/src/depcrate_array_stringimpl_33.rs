// Generated macro for impl_33 (impl)
macro_rules! Depcrate_array_stringimpl_33 {
() => {
// Module: crate::array_string
// Provides: {"impl_33"}
// Dependencies: {}
impl < const CAP : usize > PartialEq < ArrayString < CAP > > for str { fn eq (& self , rhs : & ArrayString < CAP >) -> bool { self == & * * rhs } }
};
}
