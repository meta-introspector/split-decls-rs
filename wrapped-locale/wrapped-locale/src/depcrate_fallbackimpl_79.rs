// Generated macro for impl_79 (impl)
macro_rules! Depcrate_fallbackimpl_79 {
() => {
// Module: crate::fallback
// Provides: {"impl_79"}
// Dependencies: {}
impl LocaleFallbackerBorrowed < 'static > { # [doc = " Creates a [`LocaleFallbackerBorrowed`] with compiled fallback data (likely subtags and parent locales)."] # [doc = ""] # [doc = " ✨ *Enabled with the `compiled_data` Cargo feature.*"] # [doc = ""] # [doc = " [📚 Help choosing a constructor](icu_provider::constructors)"] # [cfg (feature = "compiled_data")] # [expect (clippy :: new_without_default)] pub const fn new () -> Self { Self { likely_subtags : crate :: provider :: Baked :: SINGLETON_LOCALE_LIKELY_SUBTAGS_LANGUAGE_V1 , parents : crate :: provider :: Baked :: SINGLETON_LOCALE_PARENTS_V1 , } } # [doc = " Cheaply converts a [`LocaleFallbackerBorrowed<'static>`] into a [`LocaleFallbacker`]."] # [doc = ""] # [doc = " Note: Due to branching and indirection, using [`LocaleFallbacker`] might inhibit some"] # [doc = " compile-time optimizations that are possible with [`LocaleFallbackerBorrowed`]."] pub const fn static_to_owned (self) -> LocaleFallbacker { LocaleFallbacker { likely_subtags : DataPayload :: from_static_ref (self . likely_subtags) , parents : DataPayload :: from_static_ref (self . parents) , } } }
};
}
