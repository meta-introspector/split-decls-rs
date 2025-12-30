// Generated macro for macro_1762 (macro)
macro_rules! Depcrate_docmacro_1762 {
() => {
// Module: crate::doc
// Provides: {"macro_1762"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for links with code directly adjacent to code text:"] # [doc = " `` [`MyItem`]`<`[`u32`]`>` ``."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It can be written more simply using HTML-style `<code>` tags."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " //! [`first`](x)`second`"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " //! <code>[first](x)second</code>"] # [doc = " ```"] # [clippy :: version = "1.87.0"] pub DOC_LINK_CODE , nursery , "link with code back-to-back with other code" }
};
}
