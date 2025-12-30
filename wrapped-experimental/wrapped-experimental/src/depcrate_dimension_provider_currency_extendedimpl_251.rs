// Generated macro for impl_251 (impl)
macro_rules! Depcrate_dimension_provider_currency_extendedimpl_251 {
() => {
// Module: crate::dimension::provider::currency::extended
// Provides: {"impl_251"}
// Dependencies: {}
impl < 'data > CurrencyExtendedData < 'data > { # [doc = " Construct an instance directly from a byte slice."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The bytes must represent a valid [`icu_plurals::provider::PluralElementsPackedULE`]"] pub const unsafe fn from_bytes_unchecked (bytes : & 'data [u8]) -> Self { Self { display_names : icu_plurals :: provider :: PluralElementsPackedCow { elements : alloc :: borrow :: Cow :: Borrowed (icu_plurals :: provider :: PluralElementsPackedULE :: from_bytes_unchecked (bytes) ,) , } , } } }
};
}
