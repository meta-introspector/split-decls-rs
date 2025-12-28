macro_rules! deps {
    () => {
        Closure!();
        Visitor!();
        FnKind!();
    };
}

macro_rules! walk_fn_kind {
    () => {
        deps!();
        pub fn walk_fn_kind < 'v , V : Visitor < 'v > > (visitor : & mut V , function_kind : FnKind < 'v >) -> V :: Result { match function_kind { FnKind :: ItemFn (_ , generics , ..) => { try_visit ! (visitor . visit_generics (generics)) ; } FnKind :: Closure | FnKind :: Method (..) => { } } V :: Result :: output () }
    };
}

walk_fn_kind!()