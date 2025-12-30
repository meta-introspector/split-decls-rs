// Generated macro for impl_254 (impl)
macro_rules! Depcrate_parse_rstestimpl_254 {
() => {
// Module: crate::parse::rstest
// Provides: {"impl_254"}
// Dependencies: {}
impl Parse for RsTestItem { fn parse (input : ParseStream) -> syn :: Result < Self > { if input . fork () . parse :: < TestCase > () . is_ok () { input . parse :: < TestCase > () . map (RsTestItem :: TestCase) } else if input . peek2 (Token ! [=>]) { input . parse :: < ValueList > () . map (RsTestItem :: ValueList) } else if input . fork () . parse :: < Fixture > () . is_ok () { input . parse :: < Fixture > () . map (RsTestItem :: Fixture) } else if input . fork () . parse :: < Ident > () . is_ok () { input . parse :: < Ident > () . map (IntoPat :: into_pat) . map (RsTestItem :: CaseArgName) } else { Err (syn :: Error :: new (Span :: call_site () , "Cannot parse it")) } } }
};
}
