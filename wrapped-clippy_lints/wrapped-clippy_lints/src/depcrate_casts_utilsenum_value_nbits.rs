// Generated macro for enum_value_nbits (function)
macro_rules! Depcrate_casts_utilsenum_value_nbits {
() => {
// Module: crate::casts::utils
// Provides: {"enum_value_nbits"}
// Dependencies: {}
pub (super) fn enum_value_nbits (value : EnumValue) -> u64 { match value { EnumValue :: Unsigned (x) => 128 - x . leading_zeros () , EnumValue :: Signed (x) if x < 0 => 128 - (- (x + 1)) . leading_zeros () + 1 , EnumValue :: Signed (x) => 128 - x . leading_zeros () , } . into () }
};
}
