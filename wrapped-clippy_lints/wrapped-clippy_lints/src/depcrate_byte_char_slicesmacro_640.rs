// Generated macro for macro_640 (macro)
macro_rules! Depcrate_byte_char_slicesmacro_640 {
() => {
// Module: crate::byte_char_slices
// Provides: {"macro_640"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for hard to read slices of byte characters, that could be more easily expressed as a"] # [doc = " byte string."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = ""] # [doc = " Potentially makes the string harder to read."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```ignore"] # [doc = " &[b'H', b'e', b'l', b'l', b'o'];"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```ignore"] # [doc = " b\"Hello\""] # [doc = " ```"] # [clippy :: version = "1.81.0"] pub BYTE_CHAR_SLICES , style , "hard to read byte char slice" }
};
}
