// Generated macro for vis_offset (function)
macro_rules! Depcrate_utilsvis_offset {
() => {
// Module: crate::utils
// Provides: {"vis_offset"}
// Dependencies: {}
pub (crate) fn vis_offset (node : & SyntaxNode) -> TextSize { node . children_with_tokens () . find (| it | ! matches ! (it . kind () , WHITESPACE | COMMENT | ATTR)) . map (| it | it . text_range () . start ()) . unwrap_or_else (| | node . text_range () . start ()) }
};
}
