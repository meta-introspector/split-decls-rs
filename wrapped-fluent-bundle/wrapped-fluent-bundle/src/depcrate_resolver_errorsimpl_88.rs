// Generated macro for impl_88 (impl)
macro_rules! Depcrate_resolver_errorsimpl_88 {
() => {
// Module: crate::resolver::errors
// Provides: {"impl_88"}
// Dependencies: {}
impl < T > From < & InlineExpression < T > > for ReferenceKind where T : ToString , { fn from (exp : & InlineExpression < T >) -> Self { match exp { InlineExpression :: FunctionReference { id , .. } => Self :: Function { id : id . name . to_string () , } , InlineExpression :: MessageReference { id , attribute } => Self :: Message { id : id . name . to_string () , attribute : attribute . as_ref () . map (| i | i . name . to_string ()) , } , InlineExpression :: TermReference { id , attribute , .. } => Self :: Term { id : id . name . to_string () , attribute : attribute . as_ref () . map (| i | i . name . to_string ()) , } , InlineExpression :: VariableReference { id , .. } => Self :: Variable { id : id . name . to_string () , } , _ => unreachable ! () , } } }
};
}
