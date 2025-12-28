macro_rules! anonymize_predicate {
    () => {
        pub fn anonymize_predicate < 'tcx > (tcx : TyCtxt < 'tcx > , pred : ty :: Predicate < 'tcx > ,) -> ty :: Predicate < 'tcx > { let new = tcx . anonymize_bound_vars (pred . kind ()) ; tcx . reuse_or_mk_predicate (pred , new) }
    };
}

anonymize_predicate!();