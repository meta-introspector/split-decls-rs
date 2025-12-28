macro_rules! deps {
    () => {
        Visitor!();
        VariantData!();
    };
}

macro_rules! walk_struct_def {
    () => {
        deps!();
        pub fn walk_struct_def < 'v , V : Visitor < 'v > > (visitor : & mut V , struct_definition : & 'v VariantData < 'v > ,) -> V :: Result { visit_opt ! (visitor , visit_id , struct_definition . ctor_hir_id ()) ; walk_list ! (visitor , visit_field_def , struct_definition . fields ()) ; V :: Result :: output () }
    };
}

walk_struct_def!()