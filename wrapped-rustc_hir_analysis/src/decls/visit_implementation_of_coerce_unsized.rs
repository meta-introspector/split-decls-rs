macro_rules! deps {
    () => {
        Checker!();
    };
}

macro_rules! visit_implementation_of_coerce_unsized {
    () => {
        deps!();
        fn visit_implementation_of_coerce_unsized (checker : & Checker < '_ >) -> Result < () , ErrorGuaranteed > { let tcx = checker . tcx ; let impl_did = checker . impl_def_id ; debug ! ("visit_implementation_of_coerce_unsized: impl_did={:?}" , impl_did) ; tcx . ensure_ok () . coerce_unsized_info (impl_did) }
    };
}

visit_implementation_of_coerce_unsized!()