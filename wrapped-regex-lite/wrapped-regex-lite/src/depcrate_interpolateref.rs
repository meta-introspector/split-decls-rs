// Generated macro for Ref (enum)
macro_rules! Depcrate_interpolateRef {
() => {
// Module: crate::interpolate
// Provides: {"Ref"}
// Dependencies: {}
# [doc = " A reference to a capture group in some text."] # [doc = ""] # [doc = " e.g., `$2`, `$foo`, `${foo}`."] # [derive (Clone , Copy , Debug , Eq , PartialEq)] enum Ref < 'a > { Named (& 'a str) , Number (usize) , }
};
}
