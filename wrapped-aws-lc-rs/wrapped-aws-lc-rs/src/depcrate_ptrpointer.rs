// Generated macro for Pointer (trait)
macro_rules! Depcrate_ptrPointer {
() => {
// Module: crate::ptr
// Provides: {"Pointer"}
// Dependencies: {}
pub (crate) trait Pointer { type T ; fn free (& mut self) ; fn as_const_ptr (& self) -> * const Self :: T ; fn as_mut_ptr (& mut self) -> * mut Self :: T ; }
};
}
