macro_rules! deps {
    () => {
        CompletionItemRefMode!();
        CompletionContext!();
    };
}

macro_rules! compute_ref_match {
    () => {
        deps!();
        fn compute_ref_match (ctx : & CompletionContext < '_ > , completion_ty : & hir :: Type < '_ > ,) -> Option < CompletionItemRefMode > { let expected_type = ctx . expected_type . as_ref () ? ; let expected_without_ref = expected_type . remove_ref () ; let completion_without_ref = completion_ty . remove_ref () ; if expected_type . could_unify_with (ctx . db , completion_ty) { return None ; } if let Some (expected_without_ref) = & expected_without_ref && completion_ty . autoderef (ctx . db) . any (| ty | ty == * expected_without_ref) { cov_mark :: hit ! (suggest_ref) ; let mutability = if expected_type . is_mutable_reference () { hir :: Mutability :: Mut } else { hir :: Mutability :: Shared } ; return Some (CompletionItemRefMode :: Reference (mutability)) ; } if let Some (completion_without_ref) = completion_without_ref && completion_without_ref == * expected_type && completion_without_ref . is_copy (ctx . db) { cov_mark :: hit ! (suggest_deref) ; return Some (CompletionItemRefMode :: Dereference) ; } None }
    };
}

compute_ref_match!();