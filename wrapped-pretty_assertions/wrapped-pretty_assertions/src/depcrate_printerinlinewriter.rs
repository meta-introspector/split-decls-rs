// Generated macro for InlineWriter (struct)
macro_rules! Depcrate_printerInlineWriter {
() => {
// Module: crate::printer
// Provides: {"InlineWriter"}
// Dependencies: {}
# [doc = " Group character styling for an inline diff, to prevent wrapping each single"] # [doc = " character in terminal styling codes."] # [doc = ""] # [doc = " Styles are applied automatically each time a new style is given in `write_with_style`."] struct InlineWriter < 'a , Writer > { f : & 'a mut Writer , style : Style , }
};
}
