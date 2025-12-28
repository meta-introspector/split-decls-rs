macro_rules! deps {
    () => {
        FnCtxt!();
    };
}

macro_rules! NestedObligationsForSelfTy {
    () => {
        deps!();
        struct NestedObligationsForSelfTy < 'a , 'tcx > { fcx : & 'a FnCtxt < 'a , 'tcx > , self_ty : ty :: TyVid , root_cause : & 'a ObligationCause < 'tcx > , obligations_for_self_ty : & 'a mut PredicateObligations < 'tcx > , }
    };
}

NestedObligationsForSelfTy!();