// Generated macro for impl_37 (impl)
macro_rules! Depcrate_envimpl_37 {
() => {
// Module: crate::env
// Provides: {"impl_37"}
// Dependencies: {}
impl LocalesProvider for Vec < LanguageIdentifier > { type Iter = < Vec < LanguageIdentifier > as IntoIterator > :: IntoIter ; fn locales (& self) -> Self :: Iter { self . clone () . into_iter () } }
};
}
