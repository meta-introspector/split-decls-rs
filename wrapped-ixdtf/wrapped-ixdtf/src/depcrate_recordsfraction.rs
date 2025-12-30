// Generated macro for Fraction (struct)
macro_rules! Depcrate_recordsFraction {
() => {
// Module: crate::records
// Provides: {"Fraction"}
// Dependencies: {}
# [doc = " A fraction value in nanoseconds or lower value."] # [doc = ""] # [doc = " # Precision note"] # [doc = ""] # [doc = " `ixdtf` parses a fraction value to a precision of 18 digits of precision, but"] # [doc = " preserves the fraction's digit length"] # [derive (Debug , Clone , Copy , PartialEq)] # [allow (clippy :: exhaustive_structs)] pub struct Fraction { pub (crate) digits : NonZeroU8 , pub (crate) value : u64 , }
};
}
