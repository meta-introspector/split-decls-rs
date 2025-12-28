macro_rules! deps {
    () => {
        PredicateQuery!();
        HigherRankedErrorCause!();
        TypeOpInfo!();
        HigherRankedLifetimeError!();
        MirBorrowckCtxt!();
    };
}

macro_rules! impl_101 {
    () => {
        deps!();
        impl < 'tcx > TypeOpInfo < 'tcx > for PredicateQuery < 'tcx > { fn fallback_error (& self , tcx : TyCtxt < 'tcx > , span : Span) -> Diag < 'tcx > { tcx . dcx () . create_err (HigherRankedLifetimeError { cause : Some (HigherRankedErrorCause :: CouldNotProve { predicate : self . canonical_query . canonical . value . value . predicate . to_string () , }) , span , }) } fn base_universe (& self) -> ty :: UniverseIndex { self . base_universe } fn nice_error < 'infcx > (& self , mbcx : & mut MirBorrowckCtxt < '_ , 'infcx , 'tcx > , cause : ObligationCause < 'tcx > , placeholder_region : ty :: Region < 'tcx > , error_region : Option < ty :: Region < 'tcx > > ,) -> Option < Diag < 'infcx > > { let (infcx , key , _) = mbcx . infcx . tcx . infer_ctxt () . build_with_canonical (cause . span , & self . canonical_query) ; let ocx = ObligationCtxt :: new (& infcx) ; type_op_prove_predicate_with_cause (& ocx , key , cause) ; let diag = try_extract_error_from_fulfill_cx (& ocx , mbcx . mir_def_id () , placeholder_region , error_region ,) ? . with_dcx (mbcx . dcx ()) ; Some (diag) } }
    };
}

impl_101!()