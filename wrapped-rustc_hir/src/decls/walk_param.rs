macro_rules! deps {
    () => {
        Visitor!();
        Param!();
    };
}

macro_rules! walk_param {
    () => {
        deps!();
        pub fn walk_param < 'v , V : Visitor < 'v > > (visitor : & mut V , param : & 'v Param < 'v >) -> V :: Result { let Param { hir_id , pat , ty_span : _ , span : _ } = param ; try_visit ! (visitor . visit_id (* hir_id)) ; visitor . visit_pat (pat) }
    };
}

walk_param!()