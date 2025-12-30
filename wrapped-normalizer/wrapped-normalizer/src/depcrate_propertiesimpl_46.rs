// Generated macro for impl_46 (impl)
macro_rules! Depcrate_propertiesimpl_46 {
() => {
// Module: crate::properties
// Provides: {"impl_46"}
// Dependencies: {}
impl CanonicalCompositionBorrowed < 'static > { # [doc = " Cheaply converts a [`CanonicalCompositionBorrowed<'static>`] into a [`CanonicalComposition`]."] # [doc = ""] # [doc = " Note: Due to branching and indirection, using [`CanonicalComposition`] might inhibit some"] # [doc = " compile-time optimizations that are possible with [`CanonicalCompositionBorrowed`]."] pub const fn static_to_owned (self) -> CanonicalComposition { CanonicalComposition { canonical_compositions : DataPayload :: from_static_ref (self . canonical_compositions) , } } # [doc = " Constructs a new `CanonicalComposition` using compiled data."] # [doc = ""] # [doc = " ✨ *Enabled with the `compiled_data` Cargo feature.*"] # [doc = ""] # [doc = " [📚 Help choosing a constructor](icu_provider::constructors)"] # [cfg (feature = "compiled_data")] pub const fn new () -> Self { Self { canonical_compositions : crate :: provider :: Baked :: SINGLETON_NORMALIZER_NFC_V1 , } } }
};
}
