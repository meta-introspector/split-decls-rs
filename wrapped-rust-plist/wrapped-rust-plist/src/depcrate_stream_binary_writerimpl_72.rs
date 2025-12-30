// Generated macro for impl_72 (impl)
macro_rules! Depcrate_stream_binary_writerimpl_72 {
() => {
// Module: crate::stream::binary_writer
// Provides: {"impl_72"}
// Dependencies: {}
impl ObjectRef { fn zero () -> ObjectRef { ObjectRef (NonZeroUsize :: new (1) . unwrap ()) } fn clone_and_increment_self (& mut self) -> ObjectRef { let current = self . 0 ; self . 0 = NonZeroUsize :: new (current . get () + 1) . unwrap () ; ObjectRef (current) } fn value (& self) -> usize { self . 0 . get () - 1 } }
};
}
