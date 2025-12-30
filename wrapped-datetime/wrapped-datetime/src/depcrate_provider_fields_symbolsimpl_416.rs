// Generated macro for impl_416 (impl)
macro_rules! Depcrate_provider_fields_symbolsimpl_416 {
() => {
// Module: crate::provider::fields::symbols
// Provides: {"impl_416"}
// Dependencies: {}
# [cfg (feature = "datagen")] impl LengthType for Month { fn get_length_type (self , length : FieldLength) -> TextOrNumeric { match length { FieldLength :: One => TextOrNumeric :: Numeric , FieldLength :: NumericOverride (_) => TextOrNumeric :: Numeric , FieldLength :: Two => TextOrNumeric :: Numeric , FieldLength :: Three => TextOrNumeric :: Text , FieldLength :: Four => TextOrNumeric :: Text , FieldLength :: Five => TextOrNumeric :: Text , FieldLength :: Six => TextOrNumeric :: Text , } } }
};
}
