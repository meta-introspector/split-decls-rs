macro_rules! deps {
    () => {
        CompletionContext!();
        CompletionRelevanceTypeMatch!();
    };
}

macro_rules! compute_type_match {
    () => {
        deps!();
        fn compute_type_match (ctx : & CompletionContext < '_ > , completion_ty : & hir :: Type < '_ > ,) -> Option < CompletionRelevanceTypeMatch > { let expected_type = ctx . expected_type . as_ref () ? ; if expected_type . is_unit () { return None ; } match_types (ctx , expected_type , completion_ty) }
    };
}

compute_type_match!()