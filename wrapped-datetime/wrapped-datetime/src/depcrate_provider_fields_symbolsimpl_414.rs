// Generated macro for impl_414 (impl)
macro_rules! Depcrate_provider_fields_symbolsimpl_414 {
() => {
// Module: crate::provider::fields::symbols
// Provides: {"impl_414"}
// Dependencies: {}
# [cfg (feature = "datagen")] impl LengthType for Year { fn get_length_type (self , _length : FieldLength) -> TextOrNumeric { match self { Year :: Cyclic => TextOrNumeric :: Text , _ => TextOrNumeric :: Numeric , } } }
};
}
