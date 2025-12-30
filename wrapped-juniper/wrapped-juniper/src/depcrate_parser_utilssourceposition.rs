// Generated macro for SourcePosition (struct)
macro_rules! Depcrate_parser_utilsSourcePosition {
() => {
// Module: crate::parser::utils
// Provides: {"SourcePosition"}
// Dependencies: {}
# [doc = " A reference to a line and column in an input source file"] # [derive (Clone , Copy , Debug , Display , Eq , Hash , Ord , PartialEq , PartialOrd)] # [display ("{line}:{col}")] pub struct SourcePosition { index : usize , line : usize , col : usize , }
};
}
