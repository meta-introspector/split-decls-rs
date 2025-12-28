macro_rules! deps {
    () => {
        TypeParam!();
    };
}

macro_rules! has_disaplayable_predicates {
    () => {
        deps!();
        fn has_disaplayable_predicates (db : & dyn HirDatabase , params : & GenericParams , store : & ExpressionStore ,) -> bool { params . where_predicates () . iter () . any (| pred | { ! matches ! (pred , WherePredicate :: TypeBound { target , .. } if matches ! (store [* target] , TypeRef :: TypeParam (id) if db . generic_params (id . parent ()) [id . local_id ()] . name () . is_none ())) }) }
    };
}

has_disaplayable_predicates!();