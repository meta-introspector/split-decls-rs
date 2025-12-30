// Generated macro for impl_424 (impl)
macro_rules! Depcrate_provider_fields_symbolsimpl_424 {
() => {
// Module: crate::provider::fields::symbols
// Provides: {"impl_424"}
// Dependencies: {}
# [cfg (feature = "datagen")] impl LengthType for Weekday { fn get_length_type (self , length : FieldLength) -> TextOrNumeric { match self { Self :: Format => TextOrNumeric :: Text , Self :: Local | Self :: StandAlone => match length { FieldLength :: One | FieldLength :: Two => TextOrNumeric :: Numeric , _ => TextOrNumeric :: Text , } , } } }
};
}
