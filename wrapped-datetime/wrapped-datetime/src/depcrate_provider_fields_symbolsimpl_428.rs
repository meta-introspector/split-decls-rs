// Generated macro for impl_428 (impl)
macro_rules! Depcrate_provider_fields_symbolsimpl_428 {
() => {
// Module: crate::provider::fields::symbols
// Provides: {"impl_428"}
// Dependencies: {}
# [cfg (feature = "datagen")] impl LengthType for TimeZone { fn get_length_type (self , _ : FieldLength) -> TextOrNumeric { use TextOrNumeric :: * ; match self { Self :: Iso | Self :: IsoWithZ => Numeric , Self :: LocalizedOffset | Self :: SpecificNonLocation | Self :: GenericNonLocation | Self :: Location => Text , } } }
};
}
