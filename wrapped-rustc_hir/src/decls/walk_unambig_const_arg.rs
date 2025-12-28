macro_rules! deps {
    () => {
        ConstArg!();
        InferKind!();
        Visitor!();
    };
}

macro_rules! walk_unambig_const_arg {
    () => {
        deps!();
        pub fn walk_unambig_const_arg < 'v , V : Visitor < 'v > > (visitor : & mut V , const_arg : & 'v ConstArg < 'v > ,) -> V :: Result { match const_arg . try_as_ambig_ct () { Some (ambig_ct) => visitor . visit_const_arg (ambig_ct) , None => { let ConstArg { hir_id , kind : _ } = const_arg ; visitor . visit_infer (* hir_id , const_arg . span () , InferKind :: Const (const_arg)) } } }
    };
}

walk_unambig_const_arg!();