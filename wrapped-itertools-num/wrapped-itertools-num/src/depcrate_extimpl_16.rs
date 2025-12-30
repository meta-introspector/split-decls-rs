// Generated macro for impl_16 (impl)
macro_rules! Depcrate_extimpl_16 {
() => {
// Module: crate::ext
// Provides: {"impl_16"}
// Dependencies: {}
impl < I , S > Iterator for Cumsum < I , S > where I : Iterator , S : Add < I :: Item , Output = S > , S : Zero + Clone , { type Item = S ; fn next (& mut self) -> Option < Self :: Item > { let z = & mut self . sum ; self . iter . next () . map (| x | { * z = z . clone () + x ; z . clone () }) } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
};
}
