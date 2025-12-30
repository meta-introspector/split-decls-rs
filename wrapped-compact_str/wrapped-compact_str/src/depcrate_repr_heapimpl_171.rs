// Generated macro for impl_171 (impl)
macro_rules! Depcrate_repr_heapimpl_171 {
() => {
// Module: crate::repr::heap
// Provides: {"impl_171"}
// Dependencies: {}
impl Clone for HeapBuffer { fn clone (& self) -> Self { let mut new = Self :: with_capacity (self . capacity ()) . unwrap_with_msg () ; unsafe { new . ptr . as_ptr () . copy_from_nonoverlapping (self . ptr . as_ptr () , self . len) } ; unsafe { new . set_len (self . len) } ; new } }
};
}
