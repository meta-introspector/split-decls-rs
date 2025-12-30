// Generated macro for impl_48 (impl)
macro_rules! Depcrate_buf_iterimpl_48 {
() => {
// Module: crate::buf::iter
// Provides: {"impl_48"}
// Dependencies: {}
impl < T : Buf > Iterator for IntoIter < T > { type Item = u8 ; fn next (& mut self) -> Option < u8 > { if ! self . inner . has_remaining () { return None ; } let b = self . inner . chunk () [0] ; self . inner . advance (1) ; Some (b) } fn size_hint (& self) -> (usize , Option < usize >) { let rem = self . inner . remaining () ; (rem , Some (rem)) } }
};
}
