// Generated macro for impl_81 (impl)
macro_rules! Depcrate_shared_util_array_strimpl_81 {
() => {
// Module: crate::shared::util::array_str
// Provides: {"impl_81"}
// Dependencies: {}
impl < const N : usize > PartialEq < ArrayStr < N > > for str { fn eq (& self , rhs : & ArrayStr < N >) -> bool { self == rhs . as_str () } }
};
}
