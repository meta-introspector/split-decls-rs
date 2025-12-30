// Generated macro for impl_61 (impl)
macro_rules! Depcrateimpl_61 {
() => {
// Module: crate
// Provides: {"impl_61"}
// Dependencies: {}
impl From < (FluentResource , Vec < ParserError >) > for TranslationBundleError { fn from ((_ , mut errs) : (FluentResource , Vec < ParserError >)) -> Self { TranslationBundleError :: ParseFtl (errs . pop () . expect ("failed ftl parse with no errors")) } }
};
}
