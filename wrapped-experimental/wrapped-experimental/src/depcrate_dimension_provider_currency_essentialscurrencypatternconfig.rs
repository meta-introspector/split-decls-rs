// Generated macro for CurrencyPatternConfig (struct)
macro_rules! Depcrate_dimension_provider_currency_essentialsCurrencyPatternConfig {
() => {
// Module: crate::dimension::provider::currency::essentials
// Provides: {"CurrencyPatternConfig"}
// Dependencies: {}
# [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_experimental :: dimension :: provider :: currency :: essentials))] # [derive (Copy , Debug , Clone , Default , PartialEq , PartialOrd , Eq , Ord)] pub struct CurrencyPatternConfig { # [doc = " Indicates which pattern to use for short currency formatting."] pub short_pattern_selection : PatternSelection , # [doc = " Indicates which pattern to use for narrow currency formatting."] pub narrow_pattern_selection : PatternSelection , # [doc = " The index of the short pattern place holder in the place holders list."] # [doc = " If the value is `None`, this means that the short pattern does not have a place holder."] pub short_placeholder_value : Option < PlaceholderValue > , # [doc = " The index of the narrow pattern place holder in the place holders list."] # [doc = " If the value is `None`, this means that the narrow pattern does not have a place holder."] pub narrow_placeholder_value : Option < PlaceholderValue > , }
};
}
