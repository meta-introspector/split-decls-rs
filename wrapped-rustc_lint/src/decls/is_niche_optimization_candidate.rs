macro_rules! is_niche_optimization_candidate {
    () => {
        # [doc = " A type is niche-optimization candidate iff:"] # [doc = " - Is a zero-sized type with alignment 1 (a “1-ZST”)."] # [doc = " - Has no fields."] # [doc = " - Does not have the `#[non_exhaustive]` attribute."] fn is_niche_optimization_candidate < 'tcx > (tcx : TyCtxt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > , ty : Ty < 'tcx > ,) -> bool { if tcx . layout_of (typing_env . as_query_input (ty)) . is_ok_and (| layout | ! layout . is_1zst ()) { return false ; } match ty . kind () { ty :: Adt (ty_def , _) => { let non_exhaustive = ty_def . is_variant_list_non_exhaustive () ; let empty = (ty_def . is_struct () && ty_def . all_fields () . next () . is_none ()) || (ty_def . is_enum () && ty_def . variants () . is_empty ()) ; ! non_exhaustive && empty } ty :: Tuple (tys) => tys . is_empty () , _ => false , } }
    };
}

is_niche_optimization_candidate!()