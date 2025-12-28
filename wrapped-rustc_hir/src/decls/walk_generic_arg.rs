macro_rules! deps {
    () => {
        InferKind!();
        Visitor!();
        InferArg!();
        Lifetime!();
        GenericArg!();
    };
}

macro_rules! walk_generic_arg {
    () => {
        deps!();
        pub fn walk_generic_arg < 'v , V : Visitor < 'v > > (visitor : & mut V , generic_arg : & 'v GenericArg < 'v > ,) -> V :: Result { match generic_arg { GenericArg :: Lifetime (lt) => visitor . visit_lifetime (lt) , GenericArg :: Type (ty) => visitor . visit_ty (ty) , GenericArg :: Const (ct) => visitor . visit_const_arg (ct) , GenericArg :: Infer (inf) => { let InferArg { hir_id , span } = inf ; visitor . visit_infer (* hir_id , * span , InferKind :: Ambig (inf)) } } }
    };
}

walk_generic_arg!();