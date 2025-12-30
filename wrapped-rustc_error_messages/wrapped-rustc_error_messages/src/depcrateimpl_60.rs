// Generated macro for impl_60 (impl)
macro_rules! Depcrateimpl_60 {
() => {
// Module: crate
// Provides: {"impl_60"}
// Dependencies: {}
impl Error for TranslationBundleError { fn source (& self) -> Option < & (dyn Error + 'static) > { match self { TranslationBundleError :: ReadFtl (e) => Some (e) , TranslationBundleError :: ParseFtl (e) => Some (e) , TranslationBundleError :: AddResource (e) => Some (e) , TranslationBundleError :: MissingLocale => None , TranslationBundleError :: ReadLocalesDir (e) => Some (e) , TranslationBundleError :: ReadLocalesDirEntry (e) => Some (e) , TranslationBundleError :: LocaleIsNotDir => None , } } }
};
}
