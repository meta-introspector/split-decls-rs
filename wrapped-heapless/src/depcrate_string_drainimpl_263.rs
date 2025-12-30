// Generated macro for impl_263 (impl)
macro_rules! Depcrate_string_drainimpl_263 {
() => {
// Module: crate::string::drain
// Provides: {"impl_263"}
// Dependencies: {}
impl < LenT : LenType > Iterator for Drain < '_ , LenT > { type Item = char ; # [inline] fn next (& mut self) -> Option < char > { self . iter . next () } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } # [inline] fn last (mut self) -> Option < char > { self . next_back () } }
};
}
