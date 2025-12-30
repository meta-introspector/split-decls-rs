// Generated macro for impl_5 (impl)
macro_rules! Depcrate_builderimpl_5 {
() => {
// Module: crate::builder
// Provides: {"impl_5"}
// Dependencies: {}
impl < 'a , K > From < ParsedPatternItem < 'a , K > > for PatternItemCow < 'a , K > { fn from (value : ParsedPatternItem < 'a , K >) -> Self { match value { ParsedPatternItem :: Literal { content , .. } => PatternItemCow :: Literal (content) , ParsedPatternItem :: Placeholder (key) => PatternItemCow :: Placeholder (key) , } } }
};
}
