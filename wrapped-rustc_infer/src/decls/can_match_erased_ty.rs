macro_rules! deps {
    () => {
        MatchAgainstHigherRankedOutlives!();
    };
}

macro_rules! can_match_erased_ty {
    () => {
        deps!();
        # [doc = " True if a (potentially higher-ranked) outlives"] # [instrument (level = "debug" , skip (tcx))] pub (super) fn can_match_erased_ty < 'tcx > (tcx : TyCtxt < 'tcx > , outlives_predicate : ty :: Binder < 'tcx , ty :: TypeOutlivesPredicate < 'tcx > > , erased_ty : Ty < 'tcx > ,) -> bool { assert ! (! outlives_predicate . has_escaping_bound_vars ()) ; let erased_outlives_predicate = tcx . erase_and_anonymize_regions (outlives_predicate) ; let outlives_ty = erased_outlives_predicate . skip_binder () . 0 ; if outlives_ty == erased_ty { true } else { MatchAgainstHigherRankedOutlives :: new (tcx) . relate (outlives_ty , erased_ty) . is_ok () } }
    };
}

can_match_erased_ty!();