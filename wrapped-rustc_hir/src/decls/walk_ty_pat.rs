macro_rules! deps {
    () => {
        TyPat!();
        TyPatKind!();
        Visitor!();
    };
}

macro_rules! walk_ty_pat {
    () => {
        deps!();
        pub fn walk_ty_pat < 'v , V : Visitor < 'v > > (visitor : & mut V , pattern : & 'v TyPat < 'v >) -> V :: Result { let TyPat { kind , hir_id , span : _ } = pattern ; try_visit ! (visitor . visit_id (* hir_id)) ; match * kind { TyPatKind :: Range (lower_bound , upper_bound) => { try_visit ! (visitor . visit_const_arg_unambig (lower_bound)) ; try_visit ! (visitor . visit_const_arg_unambig (upper_bound)) ; } TyPatKind :: Or (patterns) => walk_list ! (visitor , visit_pattern_type_pattern , patterns) , TyPatKind :: Err (_) => () , } V :: Result :: output () }
    };
}

walk_ty_pat!()