// Generated macro for impl_80 (impl)
macro_rules! Depcrate_apiimpl_80 {
() => {
// Module: crate::api
// Provides: {"impl_80"}
// Dependencies: {}
impl < L : Language > SyntaxElementChildren < L > { pub fn by_kind (self , matcher : impl Fn (L :: Kind) -> bool ,) -> impl Iterator < Item = SyntaxElement < L > > { self . raw . by_kind (move | raw_kind | matcher (L :: kind_from_raw (raw_kind))) . map (NodeOrToken :: from) } }
};
}
