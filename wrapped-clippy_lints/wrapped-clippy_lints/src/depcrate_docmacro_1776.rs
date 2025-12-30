// Generated macro for macro_1776 (macro)
macro_rules! Depcrate_docmacro_1776 {
() => {
// Module: crate::doc
// Provides: {"macro_1776"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks if included files in doc comments are included only for `cfg(doc)`."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " These files are not useful for compilation but will still be included."] # [doc = " Also, if any of these non-source code file is updated, it will trigger a"] # [doc = " recompilation."] # [doc = ""] # [doc = " ### Known problems"] # [doc = ""] # [doc = " Excluding this will currently result in the file being left out if"] # [doc = " the item's docs are inlined from another crate. This may be fixed in a"] # [doc = " future version of rustdoc."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```ignore"] # [doc = " #![doc = include_str!(\"some_file.md\")]"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " #![cfg_attr(doc, doc = include_str!(\"some_file.md\"))]"] # [doc = " ```"] # [clippy :: version = "1.85.0"] pub DOC_INCLUDE_WITHOUT_CFG , restriction , "check if files included in documentation are behind `cfg(doc)`" }
};
}
