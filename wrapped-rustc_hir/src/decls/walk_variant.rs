macro_rules! deps {
    () => {
        Variant!();
        Visitor!();
    };
}

macro_rules! walk_variant {
    () => {
        deps!();
        pub fn walk_variant < 'v , V : Visitor < 'v > > (visitor : & mut V , variant : & 'v Variant < 'v >) -> V :: Result { let Variant { ident , hir_id , def_id : _ , data , disr_expr , span : _ } = variant ; try_visit ! (visitor . visit_ident (* ident)) ; try_visit ! (visitor . visit_id (* hir_id)) ; try_visit ! (visitor . visit_variant_data (data)) ; visit_opt ! (visitor , visit_anon_const , disr_expr) ; V :: Result :: output () }
    };
}

walk_variant!();