// Generated macro for ShortCurrencyCompact (struct)
macro_rules! Depcrate_dimension_provider_currency_compactShortCurrencyCompact {
() => {
// Module: crate::dimension::provider::currency::compact
// Provides: {"ShortCurrencyCompact"}
// Dependencies: {}
# [doc = " Currency Compact  data struct."] # [derive (Debug , Clone , Default , PartialEq , yoke :: Yokeable , zerofrom :: ZeroFrom)] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_experimental :: dimension :: provider :: currency :: compact))] # [yoke (prove_covariance_manually)] pub struct ShortCurrencyCompact < 'data > { # [doc = " Contains the compact patterns for a compact currency format based on the plural rules."] # [doc = " NOTE:"] # [doc = "       A map keyed on log10 of the CLDR `type` attribute."] # [doc = "       For example:"] # [doc = "         `\"1000-count-one\": \"¤0K\"`"] # [doc = "                 -> key1 = 3, key2 = CompactCount::One, value = \"¤0K\""] # [doc = "         `\"1000-count-one-alt-alphaNextToNumber\": \"¤\u{a0}0K\"`"] # [doc = "                 -> key1 = 3, key2 = CompactCount::OneAlt, value = \"¤\u{a0}0K\""] # [cfg_attr (feature = "serde" , serde (borrow))] pub compact_patterns : ZeroMap < 'data , (i8 , CompactCount) , str > , }
};
}
