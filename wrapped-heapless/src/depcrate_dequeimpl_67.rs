// Generated macro for impl_67 (impl)
macro_rules! Depcrate_dequeimpl_67 {
() => {
// Module: crate::deque
// Provides: {"impl_67"}
// Dependencies: {}
impl < T , const N : usize > Clone for Deque < T , N > where T : Clone , { fn clone (& self) -> Self { let mut res = Self :: new () ; for i in self { unsafe { res . push_back_unchecked (i . clone ()) } } res } }
};
}
