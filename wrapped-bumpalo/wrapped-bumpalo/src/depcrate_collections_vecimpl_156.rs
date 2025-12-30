// Generated macro for impl_156 (impl)
macro_rules! Depcrate_collections_vecimpl_156 {
() => {
// Module: crate::collections::vec
// Provides: {"impl_156"}
// Dependencies: {}
impl < 'a , 'bump , T , F > Drop for DrainFilter < 'a , 'bump , T , F > where F : FnMut (& mut T) -> bool , { fn drop (& mut self) { self . for_each (drop) ; unsafe { self . vec . set_len (self . old_len - self . del) ; } } }
};
}
