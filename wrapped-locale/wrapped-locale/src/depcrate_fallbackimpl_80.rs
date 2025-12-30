// Generated macro for impl_80 (impl)
macro_rules! Depcrate_fallbackimpl_80 {
() => {
// Module: crate::fallback
// Provides: {"impl_80"}
// Dependencies: {}
impl < 'a > LocaleFallbackerWithConfig < 'a > { # [doc = " Creates an iterator based on a [`DataLocale`]."] # [doc = ""] # [doc = " If you have a [`Locale`](icu_locale_core::Locale), call `.into()` to get a [`DataLocale`]."] # [doc = ""] # [doc = " When first initialized, the locale is normalized according to the fallback algorithm."] pub fn fallback_for (& self , mut locale : DataLocale) -> LocaleFallbackIterator < 'a > { let mut default_script = None ; self . normalize (& mut locale , & mut default_script) ; let max_script = locale . script . or (default_script) ; LocaleFallbackIterator { current : locale , inner : LocaleFallbackIteratorInner { likely_subtags : self . likely_subtags , parents : self . parents , config : self . config , backup_subdivision : None , backup_variant : None , backup_region : None , max_script , } , } } }
};
}
