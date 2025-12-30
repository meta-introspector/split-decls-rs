// Generated macro for ByteRange (struct)
macro_rules! DepcrateByteRange {
() => {
// Module: crate
// Provides: {"ByteRange"}
// Dependencies: {}
# [doc = " A single inclusive range in a byte class."] # [doc = ""] # [doc = " Note that this has a few convenient impls on `PartialEq` and `PartialOrd`"] # [doc = " for testing whether a byte is contained inside a given range."] # [derive (Clone , Copy , Debug , PartialEq , PartialOrd , Eq , Ord)] pub struct ByteRange { # [doc = " The start byte of the range."] # [doc = ""] # [doc = " This must be less than or equal to `end`."] pub start : u8 , # [doc = " The end byte of the range."] # [doc = ""] # [doc = " This must be greater than or equal to `end`."] pub end : u8 , }
};
}
