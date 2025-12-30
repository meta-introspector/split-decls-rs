// Generated macro for impl_77 (impl)
macro_rules! Depcrate_apiimpl_77 {
() => {
// Module: crate::api
// Provides: {"impl_77"}
// Dependencies: {}
impl < L : Language > SyntaxNodeChildren < L > { pub fn by_kind (self , matcher : impl Fn (L :: Kind) -> bool) -> impl Iterator < Item = SyntaxNode < L > > { self . raw . by_kind (move | raw_kind | matcher (L :: kind_from_raw (raw_kind))) . map (SyntaxNode :: from) } }
};
}
