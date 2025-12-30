// Generated macro for impl_14 (impl)
macro_rules! Depcrate_operandsimpl_14 {
() => {
// Module: crate::operands
// Provides: {"impl_14"}
// Dependencies: {}
impl From < & Decimal > for PluralOperands { # [doc = " Converts a [`fixed_decimal::Decimal`] to [`PluralOperands`]. Retains at most 18"] # [doc = " digits each from the integer and fraction parts."] fn from (dec : & Decimal) -> Self { Self :: from_significand_and_exponent (dec , 0) } }
};
}
