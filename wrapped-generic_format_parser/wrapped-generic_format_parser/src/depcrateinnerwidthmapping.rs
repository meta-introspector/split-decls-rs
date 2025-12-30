// Generated macro for InnerWidthMapping (struct)
macro_rules! DepcrateInnerWidthMapping {
() => {
// Module: crate
// Provides: {"InnerWidthMapping"}
// Dependencies: {}
# [doc = " The location and before/after width of a character whose width has changed from its source code"] # [doc = " representation"] # [derive (Copy , Clone , PartialEq , Eq)] pub struct InnerWidthMapping { # [doc = " Index of the character in the source"] pub position : usize , # [doc = " The inner width in characters"] pub before : usize , # [doc = " The transformed width in characters"] pub after : usize , }
};
}
