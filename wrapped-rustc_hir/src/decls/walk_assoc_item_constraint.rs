macro_rules! deps {
    () => {
        Ty!();
        Term!();
        AssocItemConstraint!();
        Visitor!();
        AssocItemConstraintKind!();
    };
}

macro_rules! walk_assoc_item_constraint {
    () => {
        deps!();
        pub fn walk_assoc_item_constraint < 'v , V : Visitor < 'v > > (visitor : & mut V , constraint : & 'v AssocItemConstraint < 'v > ,) -> V :: Result { let AssocItemConstraint { hir_id , ident , gen_args , kind : _ , span : _ } = constraint ; try_visit ! (visitor . visit_id (* hir_id)) ; try_visit ! (visitor . visit_ident (* ident)) ; try_visit ! (visitor . visit_generic_args (* gen_args)) ; match constraint . kind { AssocItemConstraintKind :: Equality { ref term } => match term { Term :: Ty (ty) => try_visit ! (visitor . visit_ty_unambig (ty)) , Term :: Const (c) => try_visit ! (visitor . visit_const_arg_unambig (c)) , } , AssocItemConstraintKind :: Bound { bounds } => { walk_list ! (visitor , visit_param_bound , bounds) } } V :: Result :: output () }
    };
}

walk_assoc_item_constraint!()