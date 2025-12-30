// Generated macro for impl_158 (impl)
macro_rules! Depcrate_arcimpl_158 {
() => {
// Module: crate::arc
// Provides: {"impl_158"}
// Dependencies: {}
impl < T : ? Sized + PartialEq > PartialEq for Arc < T > { fn eq (& self , other : & Arc < T >) -> bool { Self :: ptr_eq (self , other) || * (* self) == * (* other) } }
};
}
