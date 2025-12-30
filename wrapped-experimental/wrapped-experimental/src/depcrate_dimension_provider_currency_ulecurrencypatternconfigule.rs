// Generated macro for CurrencyPatternConfigULE (struct)
macro_rules! Depcrate_dimension_provider_currency_uleCurrencyPatternConfigULE {
() => {
// Module: crate::dimension::provider::currency::ule
// Provides: {"CurrencyPatternConfigULE"}
// Dependencies: {}
# [doc = " [`CurrencyPatternConfigULE`] is a type optimized for efficient storing and"] # [doc = " deserialization of [`CurrencyPatternConfig`] using the `ZeroVec` model."] # [doc = ""] # [doc = " The serialization model packages the pattern item in three bytes."] # [doc = ""] # [doc = " The first bit (b7) is used to determine the short_pattern_selection. If the bit is `0`, then, the value will be `Standard`."] # [doc = " If the bit is `1`, then, the value will be `StandardAlphaNextToNumber`."] # [doc = ""] # [doc = " The second bit (b6) is used to determine the narrow_pattern_selection. If the bit is `0`, then, the value will be `Standard`."] # [doc = " If the bit is `1`, then, the value will be `StandardAlphaNextToNumber`."] # [doc = ""] # [doc = " The next three bits (b5, b4 & b3) with the second byte is used to determine the short_placeholder_value."] # [doc = " The next three bits (b2, b1 & b0) with the third byte is used to determine the narrow_placeholder_value."] # [derive (Copy , Clone , Debug , PartialEq)] # [repr (transparent)] pub struct CurrencyPatternConfigULE ([u8 ; 3]) ;
};
}
