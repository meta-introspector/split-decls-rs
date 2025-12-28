macro_rules! deps {
    () => {
        CompletionRelevanceTypeMatch!();
        CompletionContext!();
    };
}

macro_rules! match_types {
    () => {
        deps!();
        fn match_types (ctx : & CompletionContext < '_ > , ty1 : & hir :: Type < '_ > , ty2 : & hir :: Type < '_ > ,) -> Option < CompletionRelevanceTypeMatch > { if ty1 == ty2 { Some (CompletionRelevanceTypeMatch :: Exact) } else if ty1 . could_unify_with (ctx . db , ty2) { Some (CompletionRelevanceTypeMatch :: CouldUnify) } else { None } }
    };
}

match_types!();