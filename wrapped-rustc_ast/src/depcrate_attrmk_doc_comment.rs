// Generated macro for mk_doc_comment (function)
macro_rules! Depcrate_attrmk_doc_comment {
() => {
// Module: crate::attr
// Provides: {"mk_doc_comment"}
// Dependencies: {}
pub fn mk_doc_comment (g : & AttrIdGenerator , comment_kind : CommentKind , style : AttrStyle , data : Symbol , span : Span ,) -> Attribute { Attribute { kind : AttrKind :: DocComment (comment_kind , data) , id : g . mk_attr_id () , style , span } }
};
}
