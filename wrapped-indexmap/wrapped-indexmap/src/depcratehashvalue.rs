// Generated macro for HashValue (struct)
macro_rules! DepcrateHashValue {
() => {
// Module: crate
// Provides: {"HashValue"}
// Dependencies: {}
# [doc = " Hash value newtype. Not larger than usize, since anything larger"] # [doc = " isn't used for selecting position anyway."] # [derive (Clone , Copy , Debug , PartialEq)] struct HashValue (usize) ;
};
}
