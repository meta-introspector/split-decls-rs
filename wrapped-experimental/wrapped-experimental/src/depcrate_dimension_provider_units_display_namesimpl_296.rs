// Generated macro for impl_296 (impl)
macro_rules! Depcrate_dimension_provider_units_display_namesimpl_296 {
() => {
// Module: crate::dimension::provider::units::display_names
// Provides: {"impl_296"}
// Dependencies: {}
impl < 'data > UnitsDisplayNames < 'data > { # [doc = " Construct an instance directly from a byte slice."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The bytes must represent a valid [`icu_plurals::provider::PluralElementsPackedULE`]"] pub const unsafe fn from_bytes_unchecked (bytes : & 'data [u8]) -> Self { Self { patterns : icu_plurals :: provider :: PluralElementsPackedCow { elements : alloc :: borrow :: Cow :: Borrowed (icu_plurals :: provider :: PluralElementsPackedULE :: from_bytes_unchecked (bytes) ,) , } , } } }
};
}
