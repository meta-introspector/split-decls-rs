// Generated macro for impl_196 (impl)
macro_rules! Depcrate_utilimpl_196 {
() => {
// Module: crate::util
// Provides: {"impl_196"}
// Dependencies: {}
impl < T : Buf > BufList < T > { # [inline] pub (crate) fn push (& mut self , buf : T) { debug_assert ! (buf . has_remaining ()) ; self . bufs . push_back (buf) ; } # [inline] pub (crate) fn pop (& mut self) -> Option < T > { self . bufs . pop_front () } }
};
}
