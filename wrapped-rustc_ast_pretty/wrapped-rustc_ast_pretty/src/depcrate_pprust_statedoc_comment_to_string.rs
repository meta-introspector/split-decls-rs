// Generated macro for doc_comment_to_string (function)
macro_rules! Depcrate_pprust_statedoc_comment_to_string {
() => {
// Module: crate::pprust::state
// Provides: {"doc_comment_to_string"}
// Dependencies: {}
pub fn doc_comment_to_string (comment_kind : CommentKind , attr_style : ast :: AttrStyle , data : Symbol ,) -> String { match (comment_kind , attr_style) { (CommentKind :: Line , ast :: AttrStyle :: Outer) => format ! ("///{data}") , (CommentKind :: Line , ast :: AttrStyle :: Inner) => format ! ("//!{data}") , (CommentKind :: Block , ast :: AttrStyle :: Outer) => format ! ("/**{data}*/") , (CommentKind :: Block , ast :: AttrStyle :: Inner) => format ! ("/*!{data}*/") , } }
};
}
