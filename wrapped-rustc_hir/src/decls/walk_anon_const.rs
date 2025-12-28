macro_rules! deps {
    () => {
        AnonConst!();
        Visitor!();
    };
}

macro_rules! walk_anon_const {
    () => {
        deps!();
        pub fn walk_anon_const < 'v , V : Visitor < 'v > > (visitor : & mut V , constant : & 'v AnonConst) -> V :: Result { let AnonConst { hir_id , def_id : _ , body , span : _ } = constant ; try_visit ! (visitor . visit_id (* hir_id)) ; visitor . visit_nested_body (* body) }
    };
}

walk_anon_const!();