// Generated macro for impl_43 (impl)
macro_rules! Depcrate_array_stringimpl_43 {
() => {
// Module: crate::array_string
// Provides: {"impl_43"}
// Dependencies: {}
impl < const CAP : usize > Clone for ArrayString < CAP > { fn clone (& self) -> ArrayString < CAP > { * self } fn clone_from (& mut self , rhs : & Self) { self . clear () ; self . try_push_str (rhs) . ok () ; } }
};
}
