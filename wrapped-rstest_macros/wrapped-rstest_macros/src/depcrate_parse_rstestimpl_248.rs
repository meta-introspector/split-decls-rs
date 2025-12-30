// Generated macro for impl_248 (impl)
macro_rules! Depcrate_parse_rstestimpl_248 {
() => {
// Module: crate::parse::rstest
// Provides: {"impl_248"}
// Dependencies: {}
impl MaybePat for RsTestItem { fn maybe_pat (& self) -> Option < & syn :: Pat > { match self { RsTestItem :: Fixture (f) => f . maybe_pat () , RsTestItem :: CaseArgName (c) => Some (c) , RsTestItem :: TestCase (_) => None , RsTestItem :: ValueList (vl) => Some (& vl . arg) , } } }
};
}
