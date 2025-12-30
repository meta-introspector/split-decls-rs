// Generated macro for impl_6 (impl)
macro_rules! Depcrate_ast_helperimpl_6 {
() => {
// Module: crate::ast::helper
// Provides: {"impl_6"}
// Dependencies: {}
impl < S > From < CommentDef < S > > for Comment < S > { fn from (input : CommentDef < S >) -> Self { match input { CommentDef :: Single { content } => Self { content : vec ! [content] , } , CommentDef :: Multi { content } => Self { content } , } } }
};
}
