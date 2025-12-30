// Generated macro for impl_62 (impl)
macro_rules! Depcrateimpl_62 {
() => {
// Module: crate
// Provides: {"impl_62"}
// Dependencies: {}
impl From < Vec < FluentError > > for TranslationBundleError { fn from (mut errs : Vec < FluentError >) -> Self { TranslationBundleError :: AddResource (errs . pop () . expect ("failed adding resource to bundle with no errors") ,) } }
};
}
