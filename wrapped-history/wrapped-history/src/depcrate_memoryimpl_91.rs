// Generated macro for impl_91 (impl)
macro_rules! Depcrate_memoryimpl_91 {
() => {
// Module: crate::memory
// Provides: {"impl_91"}
// Dependencies: {}
impl MemoryHistory { # [doc = " Creates a new [`MemoryHistory`] with a default entry of '/'."] pub fn new () -> Self { Self :: default () } # [doc = " Creates a new [`MemoryHistory`] with entries."] pub fn with_entries < 'a > (entries : impl IntoIterator < Item = impl Into < Cow < 'a , str > > >) -> Self { let self_ = Self :: new () ; for (index , entry) in entries . into_iter () . enumerate () { if index == 0 { self_ . replace (entry) ; } else { self_ . push (entry) ; } } self_ } fn notify_callbacks (& self) { crate :: utils :: notify_callbacks (self . callbacks . clone ()) ; } }
};
}
