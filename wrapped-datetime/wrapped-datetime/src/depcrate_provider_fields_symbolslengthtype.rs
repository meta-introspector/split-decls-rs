// Generated macro for LengthType (trait)
macro_rules! Depcrate_provider_fields_symbolsLengthType {
() => {
// Module: crate::provider::fields::symbols
// Provides: {"LengthType"}
// Dependencies: {}
# [doc = " [`FieldSymbols`](FieldSymbol) can be either text or numeric. This categorization is important"] # [doc = " when matching skeletons with a components [`Bag`](crate::options::components::Bag)."] # [cfg (feature = "datagen")] pub (crate) trait LengthType { fn get_length_type (self , length : FieldLength) -> TextOrNumeric ; }
};
}
