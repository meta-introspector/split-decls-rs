macro_rules! deps {
    () => {
        WfCheckingCtxt!();
    };
}

macro_rules! legacy_receiver_is_implemented {
    () => {
        deps!();
        fn legacy_receiver_is_implemented < 'tcx > (wfcx : & WfCheckingCtxt < '_ , 'tcx > , legacy_receiver_trait_def_id : DefId , cause : ObligationCause < 'tcx > , receiver_ty : Ty < 'tcx > ,) -> bool { let tcx = wfcx . tcx () ; let trait_ref = ty :: TraitRef :: new (tcx , legacy_receiver_trait_def_id , [receiver_ty]) ; let obligation = Obligation :: new (tcx , cause , wfcx . param_env , trait_ref) ; if wfcx . infcx . predicate_must_hold_modulo_regions (& obligation) { true } else { debug ! ("receiver_is_implemented: type `{:?}` does not implement `LegacyReceiver` trait" , receiver_ty) ; false } }
    };
}

legacy_receiver_is_implemented!()