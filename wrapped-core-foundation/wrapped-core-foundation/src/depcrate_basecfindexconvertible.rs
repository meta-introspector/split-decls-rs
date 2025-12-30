// Generated macro for CFIndexConvertible (trait)
macro_rules! Depcrate_baseCFIndexConvertible {
() => {
// Module: crate::base
// Provides: {"CFIndexConvertible"}
// Dependencies: {}
pub trait CFIndexConvertible { # [doc = " Always use this method to construct a `CFIndex` value. It performs bounds checking to"] # [doc = " ensure the value is in range."] fn to_CFIndex (self) -> CFIndex ; }
};
}
