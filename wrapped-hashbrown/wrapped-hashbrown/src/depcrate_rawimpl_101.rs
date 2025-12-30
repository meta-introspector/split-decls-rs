// Generated macro for impl_101 (impl)
macro_rules! Depcrate_rawimpl_101 {
() => {
// Module: crate::raw
// Provides: {"impl_101"}
// Dependencies: {}
impl < T > RawIter < T > { unsafe fn drop_elements (& mut self) { if T :: NEEDS_DROP && self . items != 0 { for item in self { item . drop () ; } } } }
};
}
