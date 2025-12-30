// Generated macro for impl_59 (impl)
macro_rules! Depcrateimpl_59 {
() => {
// Module: crate
// Provides: {"impl_59"}
// Dependencies: {}
impl fmt :: Display for TranslationBundleError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { TranslationBundleError :: ReadFtl (e) => write ! (f , "could not read ftl file: {e}") , TranslationBundleError :: ParseFtl (e) => { write ! (f , "could not parse ftl file: {e}") } TranslationBundleError :: AddResource (e) => write ! (f , "failed to add resource: {e}") , TranslationBundleError :: MissingLocale => write ! (f , "missing locale directory") , TranslationBundleError :: ReadLocalesDir (e) => { write ! (f , "could not read locales dir: {e}") } TranslationBundleError :: ReadLocalesDirEntry (e) => { write ! (f , "could not read locales dir entry: {e}") } TranslationBundleError :: LocaleIsNotDir => { write ! (f , "`$sysroot/share/locales/$locale` is not a directory") } } } }
};
}
