// Generated macro for impl_63 (impl)
macro_rules! Depcrate_default_expressionimpl_63 {
() => {
// Module: crate::default_expression
// Provides: {"impl_63"}
// Dependencies: {}
impl DefaultExpression { # [doc = " Add the crate root path so the default expression can be emitted"] # [doc = " to a `TokenStream`."] # [doc = ""] # [doc = " This function is needed because the crate root is inherited from the container, so it cannot"] # [doc = " be provided at parse time to [`darling::FromMeta::from_word`] when reading, and [`ToTokens`] does not"] # [doc = " accept any additional parameters, so it annot be provided at emit time."] pub fn with_crate_root < 'a > (& 'a self , crate_root : & 'a syn :: Path) -> impl 'a + ToTokens { DefaultExpressionWithCrateRoot { crate_root , expr : self , } } pub fn span (& self) -> Span { match self { DefaultExpression :: Explicit (block) => block . span () , DefaultExpression :: Trait => Span :: call_site () , } } # [cfg (test)] pub fn explicit < I : Into < BlockContents > > (content : I) -> Self { DefaultExpression :: Explicit (content . into ()) } }
};
}
