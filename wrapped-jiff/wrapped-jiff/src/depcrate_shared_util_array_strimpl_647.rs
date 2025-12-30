// Generated macro for impl_647 (impl)
macro_rules! Depcrate_shared_util_array_strimpl_647 {
() => {
// Module: crate::shared::util::array_str
// Provides: {"impl_647"}
// Dependencies: {}
impl < const N : usize > PartialEq < ArrayStr < N > > for str { fn eq (& self , rhs : & ArrayStr < N >) -> bool { self == rhs . as_str () } }
};
}
