macro_rules! deps {
    () => {
        ConstArg!();
        Visitor!();
        AmbigArg!();
        ConstArgKind!();
        Path!();
    };
}

macro_rules! walk_const_arg {
    () => {
        deps!();
        pub fn walk_const_arg < 'v , V : Visitor < 'v > > (visitor : & mut V , const_arg : & 'v ConstArg < 'v , AmbigArg > ,) -> V :: Result { let ConstArg { hir_id , kind } = const_arg ; try_visit ! (visitor . visit_id (* hir_id)) ; match kind { ConstArgKind :: Path (qpath) => visitor . visit_qpath (qpath , * hir_id , qpath . span ()) , ConstArgKind :: Anon (anon) => visitor . visit_anon_const (* anon) , } }
    };
}

walk_const_arg!();