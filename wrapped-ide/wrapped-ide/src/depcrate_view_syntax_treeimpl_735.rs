// Generated macro for impl_735 (impl)
macro_rules! Depcrate_view_syntax_treeimpl_735 {
() => {
// Module: crate::view_syntax_tree
// Provides: {"impl_735"}
// Dependencies: {}
impl TextPosition { pub (crate) fn new (line_index : & LineIndex , offset : TextSize) -> Self { let LineCol { line , col } = line_index . line_col (offset) ; Self { offset , line , col } } }
};
}
