// Generated macro for Path (struct)
macro_rules! Depcrate_typesPath {
() => {
// Module: crate::types
// Provides: {"Path"}
// Dependencies: {}
# [doc = " Any value that can be interpreted as a path to a resource on disk."] # [doc = ""] # [doc = " Git represents file paths as byte arrays, modeled here as owned or borrowed byte sequences."] # [derive (Clone , Eq , PartialEq , Ord , PartialOrd , Hash , Debug)] pub struct Path < 'a > { # [doc = " The path string, un-interpolated"] pub value : std :: borrow :: Cow < 'a , bstr :: BStr > , }
};
}
