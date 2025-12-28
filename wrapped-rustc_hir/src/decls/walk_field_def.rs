macro_rules! deps {
    () => {
        FieldDef!();
        Visitor!();
    };
}

macro_rules! walk_field_def {
    () => {
        deps!();
        pub fn walk_field_def < 'v , V : Visitor < 'v > > (visitor : & mut V , FieldDef { hir_id , ident , ty , default , span : _ , vis_span : _ , def_id : _ , safety : _ } : & 'v FieldDef < 'v > ,) -> V :: Result { try_visit ! (visitor . visit_id (* hir_id)) ; try_visit ! (visitor . visit_ident (* ident)) ; visit_opt ! (visitor , visit_anon_const , default) ; visitor . visit_ty_unambig (* ty) }
    };
}

walk_field_def!();