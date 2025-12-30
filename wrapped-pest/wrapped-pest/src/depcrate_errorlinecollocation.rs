// Generated macro for LineColLocation (enum)
macro_rules! Depcrate_errorLineColLocation {
() => {
// Module: crate::error
// Provides: {"LineColLocation"}
// Dependencies: {}
# [doc = " Line/column where an `Error` has occurred."] # [derive (Clone , Debug , Eq , Hash , PartialEq)] pub enum LineColLocation { # [doc = " Line/column pair if `Error` was created by `Error::new_from_pos`"] Pos ((usize , usize)) , # [doc = " Line/column pairs if `Error` was created by `Error::new_from_span`"] Span ((usize , usize) , (usize , usize)) , }
};
}
