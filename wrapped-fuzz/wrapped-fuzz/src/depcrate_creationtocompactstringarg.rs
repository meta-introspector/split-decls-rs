// Generated macro for ToCompactStringArg (enum)
macro_rules! Depcrate_creationToCompactStringArg {
() => {
// Module: crate::creation
// Provides: {"ToCompactStringArg"}
// Dependencies: {}
# [doc = " Types that we're able to convert to a [`CompactString`]"] # [doc = ""] # [doc = " Note: number types, bool, and char all have a special implementation for performance"] # [derive (Arbitrary , Debug)] pub enum ToCompactStringArg { # [doc = " Create from a number type using [`ToCompactString`]"] Num (NumType) , # [doc = " Create from a non-zero number type using [`ToCompactString`]"] NonZeroNum (NonZeroNumType) , # [doc = " Create from a `bool` using [`ToCompactString`]"] Bool (bool) , # [doc = " Create from a `char` using [`ToCompactString`]"] Char (char) , # [doc = " Create  from a string using [`ToCompactString`]"] String (String) , }
};
}
