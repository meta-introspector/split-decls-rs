// Generated macro for ClassRange (struct)
macro_rules! DepcrateClassRange {
() => {
// Module: crate
// Provides: {"ClassRange"}
// Dependencies: {}
# [doc = " A single inclusive range in a character class."] # [doc = ""] # [doc = " Since range boundaries are defined by Unicode scalar values, the boundaries"] # [doc = " can never be in the open interval `(0xD7FF, 0xE000)`. However, a range may"] # [doc = " *cover* codepoints that are not scalar values."] # [doc = ""] # [doc = " Note that this has a few convenient impls on `PartialEq` and `PartialOrd`"] # [doc = " for testing whether a character is contained inside a given range."] # [derive (Clone , Copy , Debug , PartialEq , PartialOrd , Eq , Ord)] pub struct ClassRange { # [doc = " The start character of the range."] # [doc = ""] # [doc = " This must be less than or equal to `end`."] pub start : char , # [doc = " The end character of the range."] # [doc = ""] # [doc = " This must be greater than or equal to `start`."] pub end : char , }
};
}
