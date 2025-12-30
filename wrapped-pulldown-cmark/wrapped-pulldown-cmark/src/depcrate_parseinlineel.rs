// Generated macro for InlineEl (struct)
macro_rules! Depcrate_parseInlineEl {
() => {
// Module: crate::parse
// Provides: {"InlineEl"}
// Dependencies: {}
# [derive (Copy , Clone , Debug)] struct InlineEl { # [doc = " offset of tree node"] start : TreeIndex , # [doc = " number of delimiters available for matching"] count : usize , # [doc = " length of the run that these delimiters came from"] run_length : usize , # [doc = " b'*', b'_', or b'~'"] c : u8 , # [doc = " can both open and close"] both : bool , }
};
}
