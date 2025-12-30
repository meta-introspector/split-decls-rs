// Generated macro for impl_646 (impl)
macro_rules! Depcrate_shared_util_array_strimpl_646 {
() => {
// Module: crate::shared::util::array_str
// Provides: {"impl_646"}
// Dependencies: {}
impl < const N : usize > PartialEq < & str > for ArrayStr < N > { fn eq (& self , rhs : & & str) -> bool { self . as_str () == * rhs } }
};
}
