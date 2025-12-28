macro_rules! deps {
    () => {
        Visitor!();
        Generics!();
    };
}

macro_rules! walk_generics {
    () => {
        deps!();
        pub fn walk_generics < 'v , V : Visitor < 'v > > (visitor : & mut V , generics : & 'v Generics < 'v >) -> V :: Result { let & Generics { params , predicates , has_where_clause_predicates : _ , where_clause_span : _ , span : _ , } = generics ; walk_list ! (visitor , visit_generic_param , params) ; walk_list ! (visitor , visit_where_predicate , predicates) ; V :: Result :: output () }
    };
}

walk_generics!()