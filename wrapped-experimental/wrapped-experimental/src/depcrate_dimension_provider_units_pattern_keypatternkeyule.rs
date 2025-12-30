// Generated macro for PatternKeyULE (struct)
macro_rules! Depcrate_dimension_provider_units_pattern_keyPatternKeyULE {
() => {
// Module: crate::dimension::provider::units::pattern_key
// Provides: {"PatternKeyULE"}
// Dependencies: {}
# [doc = " [`PatternKeyULE`] is a type optimized for efficient storage and"] # [doc = " deserialization of [`PatternKey`] using the `ZeroVec` model."] # [doc = ""] # [doc = " The serialization model packages the pattern item in a single byte."] # [doc = ""] # [doc = " The first two bits (b7 & b6) determine the variant of the pattern key:"] # [doc = " - `00`: `Binary`"] # [doc = " - `01`: `Decimal`"] # [doc = " - `10`: `Power`"] # [doc = " - `11`: Forbidden"] # [doc = ""] # [doc = " The next 6 bits (b5 to b0) determine the value of the pattern key:"] # [doc = " - For `Binary`, the value is mapped directly to the pattern value."] # [doc = " - For `Decimal`:"] # [doc = "     - b5 is determining the sign of the value. if b5 is 0, the value is positive. if b5 is 1, the value is negative."] # [doc = "     - b4 to b0 are determining the magnitude of the value."] # [doc = " - For `Power`:"] # [doc = "     - b5 and b4 represent the power value, which can be `10` to represent `Two` and `11` to represent `Three`."] # [doc = "     - b3 to b0 represent the count value, which can be:"] # [doc = "         - `0000`: Zero"] # [doc = "         - `0001`: One"] # [doc = "         - `0010`: Two"] # [doc = "         - `0011`: Few"] # [doc = "         - `0100`: Many"] # [doc = "         - `0101`: Other"] # [doc = "     - Note: In the `Power` case, b3 is always 0, and when b2 is 1, b1 must be 0."] # [derive (Copy , Clone , PartialOrd , Ord , PartialEq , Eq , Debug)] pub struct PatternKeyULE (u8) ;
};
}
