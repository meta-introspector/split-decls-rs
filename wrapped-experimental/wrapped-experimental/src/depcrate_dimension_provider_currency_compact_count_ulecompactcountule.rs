// Generated macro for CompactCountULE (struct)
macro_rules! Depcrate_dimension_provider_currency_compact_count_uleCompactCountULE {
() => {
// Module: crate::dimension::provider::currency::compact_count_ule
// Provides: {"CompactCountULE"}
// Dependencies: {}
# [doc = " [`CompactCountULE`] is a type optimized for efficient storing and"] # [doc = " deserialization of [`CompactCount`] using the `ZeroVec` model."] # [doc = ""] # [doc = " The serialization model packages the pattern item in one byte."] # [doc = ""] # [doc = " The first bit (b7) is used to determine count_type."] # [doc = " If the bit is `0`, then, then the type is `Standard`."] # [doc = " If the bit is `1`, then, then the type is `AlphaNextToNumber`."] # [doc = ""] # [doc = " The last three bits (b2, b1 & b0), are used to determine the count:"] # [doc = "     000 -> Count::Zero"] # [doc = "     001 -> Count::One"] # [doc = "     010 -> Count::Two"] # [doc = "     011 -> Count::Few"] # [doc = "     100 -> Count::Many"] # [doc = "     101 -> Count::Other"] # [doc = ""] # [doc = " The other bits (b6,b5,b4,b3) must always be zeros."] # [derive (Copy , Clone , Debug , PartialEq)] # [repr (transparent)] pub struct CompactCountULE (u8) ;
};
}
