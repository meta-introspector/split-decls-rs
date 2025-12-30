// Generated macro for PlaceholderValue (enum)
macro_rules! Depcrate_dimension_provider_currency_essentialsPlaceholderValue {
() => {
// Module: crate::dimension::provider::currency::essentials
// Provides: {"PlaceholderValue"}
// Dependencies: {}
# [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_experimental :: dimension :: provider :: currency :: essentials))] # [derive (Copy , Debug , Clone , PartialEq , PartialOrd , Eq , Ord)] # [repr (u16)] pub enum PlaceholderValue { # [doc = " The index of the place holder in the place holders list."] # [doc = " NOTE: the maximum value is MAX_PLACEHOLDER_INDEX which is 2045 (0b0111_1111_1101)."] Index (u16) , # [doc = " The place holder is the iso code."] ISO , }
};
}
