// Generated macro for CurrencyPatternsData (struct)
macro_rules! Depcrate_dimension_provider_currency_patternsCurrencyPatternsData {
() => {
// Module: crate::dimension::provider::currency::patterns
// Provides: {"CurrencyPatternsData"}
// Dependencies: {}
# [doc = " Currency Extended data struct."] # [derive (Debug , Clone , PartialEq , yoke :: Yokeable , zerofrom :: ZeroFrom)] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_experimental :: dimension :: provider :: currency :: patterns))] # [yoke (prove_covariance_manually)] pub struct CurrencyPatternsData < 'data > { # [doc = " Contains the unit patterns for a currency based on plural rules."] # [cfg_attr (feature = "serde" , serde (borrow))] pub patterns : PluralElementsPackedCow < 'data , DoublePlaceholderPattern > , }
};
}
