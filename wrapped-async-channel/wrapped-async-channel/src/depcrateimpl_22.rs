// Generated macro for impl_22 (impl)
macro_rules! Depcrateimpl_22 {
() => {
// Module: crate
// Provides: {"impl_22"}
// Dependencies: {}
impl < T > Drop for Sender < T > { fn drop (& mut self) { if self . channel . sender_count . fetch_sub (1 , Ordering :: AcqRel) == 1 { self . channel . close () ; } } }
};
}
