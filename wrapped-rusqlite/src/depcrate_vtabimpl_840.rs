// Generated macro for impl_840 (impl)
macro_rules! Depcrate_vtabimpl_840 {
() => {
// Module: crate::vtab
// Provides: {"impl_840"}
// Dependencies: {}
impl < 'a > Iterator for ValueIter < 'a > { type Item = ValueRef < 'a > ; # [inline] fn next (& mut self) -> Option < ValueRef < 'a > > { self . iter . next () . map (| & raw | unsafe { ValueRef :: from_value (raw) }) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
};
}
