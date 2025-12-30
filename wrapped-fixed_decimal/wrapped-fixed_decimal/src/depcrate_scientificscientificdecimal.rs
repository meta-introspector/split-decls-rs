// Generated macro for ScientificDecimal (struct)
macro_rules! Depcrate_scientificScientificDecimal {
() => {
// Module: crate::scientific
// Provides: {"ScientificDecimal"}
// Dependencies: {}
# [doc = " A struct containing a [`Decimal`] significand together with an exponent, representing a"] # [doc = " number written in scientific notation, such as 1.729×10³."] # [doc = ""] # [doc = " This structure represents any 0s shown in the significand and exponent,"] # [doc = " and an optional sign for both the significand and the exponent."] # [derive (Debug , Clone , PartialEq)] pub struct ScientificDecimal { significand : Decimal , exponent : FixedInteger , }
};
}
