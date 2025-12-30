// Generated macro for impl_91 (impl)
macro_rules! Depcrate_apiimpl_91 {
() => {
// Module: crate::api
// Provides: {"impl_91"}
// Dependencies: {}
impl < L : Language > From < cursor :: SyntaxElement > for SyntaxElement < L > { fn from (raw : cursor :: SyntaxElement) -> SyntaxElement < L > { match raw { NodeOrToken :: Node (it) => NodeOrToken :: Node (it . into ()) , NodeOrToken :: Token (it) => NodeOrToken :: Token (it . into ()) , } } }
};
}
