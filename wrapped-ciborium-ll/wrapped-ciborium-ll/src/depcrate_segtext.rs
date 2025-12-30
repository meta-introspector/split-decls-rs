// Generated macro for Text (struct)
macro_rules! Depcrate_segText {
() => {
// Module: crate::seg
// Provides: {"Text"}
// Dependencies: {}
# [doc = " A text parser"] # [doc = ""] # [doc = " This parser converts the input bytes to a `str`. This parser preserves"] # [doc = " trailing invalid UTF-8 sequences in the case that chunking fell in the"] # [doc = " middle of a valid UTF-8 character."] # [derive (Default)] pub struct Text { stored : usize , buffer : [u8 ; 3] , }
};
}
