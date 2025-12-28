macro_rules! deps {
    () => {
        Visitor!();
        PatField!();
    };
}

macro_rules! walk_pat_field {
    () => {
        deps!();
        pub fn walk_pat_field < 'v , V : Visitor < 'v > > (visitor : & mut V , field : & 'v PatField < 'v >) -> V :: Result { let PatField { hir_id , ident , pat , is_shorthand : _ , span : _ } = field ; try_visit ! (visitor . visit_id (* hir_id)) ; try_visit ! (visitor . visit_ident (* ident)) ; visitor . visit_pat (* pat) }
    };
}

walk_pat_field!();