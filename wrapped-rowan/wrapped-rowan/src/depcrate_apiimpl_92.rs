// Generated macro for impl_92 (impl)
macro_rules! Depcrate_apiimpl_92 {
() => {
// Module: crate::api
// Provides: {"impl_92"}
// Dependencies: {}
impl < L : Language > From < SyntaxElement < L > > for cursor :: SyntaxElement { fn from (element : SyntaxElement < L >) -> cursor :: SyntaxElement { match element { NodeOrToken :: Node (it) => NodeOrToken :: Node (it . into ()) , NodeOrToken :: Token (it) => NodeOrToken :: Token (it . into ()) , } } }
};
}
