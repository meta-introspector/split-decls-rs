// Generated macro for impl_37 (impl)
macro_rules! Depcrate_yielderimpl_37 {
() => {
// Module: crate::yielder
// Provides: {"impl_37"}
// Dependencies: {}
impl < T > Receiver < T > { pub (crate) fn enter < 'a > (& 'a mut self , dst : & 'a mut Option < T >) -> Enter < 'a , T > { let prev = STORE . with (| cell | { let prev = cell . get () ; cell . set (dst as * mut _ as * mut ()) ; prev }) ; Enter { _rx : self , prev } } }
};
}
