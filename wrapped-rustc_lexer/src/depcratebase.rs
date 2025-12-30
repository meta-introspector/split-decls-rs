// Generated macro for Base (enum)
macro_rules! DepcrateBase {
() => {
// Module: crate
// Provides: {"Base"}
// Dependencies: {}
# [doc = " Base of numeric literal encoding according to its prefix."] # [derive (Clone , Copy , Debug , PartialEq , Eq , PartialOrd , Ord)] pub enum Base { # [doc = " Literal starts with \"0b\"."] Binary = 2 , # [doc = " Literal starts with \"0o\"."] Octal = 8 , # [doc = " Literal doesn't contain a prefix."] Decimal = 10 , # [doc = " Literal starts with \"0x\"."] Hexadecimal = 16 , }
};
}
