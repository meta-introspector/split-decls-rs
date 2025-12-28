macro_rules! deps {
    () => {
        ReferenceKind!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl < T > From < & InlineExpression < T > > for ReferenceKind where T : ToString , { fn from (exp : & InlineExpression < T >) -> Self { match exp { InlineExpression :: FunctionReference { id , .. } => Self :: Function { id : id . name . to_string () , } , InlineExpression :: MessageReference { id , attribute } => Self :: Message { id : id . name . to_string () , attribute : attribute . as_ref () . map (| i | i . name . to_string ()) , } , InlineExpression :: TermReference { id , attribute , .. } => Self :: Term { id : id . name . to_string () , attribute : attribute . as_ref () . map (| i | i . name . to_string ()) , } , InlineExpression :: VariableReference { id , .. } => Self :: Variable { id : id . name . to_string () , } , _ => unreachable ! () , } } }
    };
}

impl_43!()