// Generated macro for CompactCount (enum)
macro_rules! Depcrate_dimension_provider_currency_compactCompactCount {
() => {
// Module: crate::dimension::provider::currency::compact
// Provides: {"CompactCount"}
// Dependencies: {}
# [derive (Copy , Clone , PartialOrd , Ord , PartialEq , Eq , Debug)] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_experimental :: dimension :: provider :: currency :: cucompact))] # [repr (u8)] pub enum CompactCount { Standard (PluralCategory) , AlphaNextToNumber (PluralCategory) , }
};
}
