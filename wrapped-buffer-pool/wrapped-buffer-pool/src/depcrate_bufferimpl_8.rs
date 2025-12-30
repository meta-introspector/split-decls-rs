// Generated macro for impl_8 (impl)
macro_rules! Depcrate_bufferimpl_8 {
() => {
// Module: crate::buffer
// Provides: {"impl_8"}
// Dependencies: {}
impl Reuse for ConsumeBuffer { fn reuse (& mut self , val : usize) -> bool { self . inner . clear () ; self . inner . shrink_to (val) ; self . head = 0 ; self . inner . capacity () > 0 } }
};
}
