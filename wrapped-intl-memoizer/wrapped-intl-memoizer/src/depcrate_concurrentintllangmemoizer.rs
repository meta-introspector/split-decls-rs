// Generated macro for IntlLangMemoizer (struct)
macro_rules! Depcrate_concurrentIntlLangMemoizer {
() => {
// Module: crate::concurrent
// Provides: {"IntlLangMemoizer"}
// Dependencies: {}
# [doc = " A thread-safe version of the [`intl_memoizer::IntlLangMemoizer`](super::IntlLangMemoizer)."] # [doc = " See the single-thread version for more documentation."] # [derive (Debug)] pub struct IntlLangMemoizer { lang : LanguageIdentifier , map : Mutex < type_map :: concurrent :: TypeMap > , }
};
}
