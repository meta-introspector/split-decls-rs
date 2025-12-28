macro_rules! deps {
    () => {
        InstantiateOpaqueType!();
        MirBorrowckCtxt!();
        TypeOpInfo!();
        HigherRankedLifetimeError!();
    };
}

macro_rules! impl_108 {
    () => {
        deps!();
        impl < 'tcx > TypeOpInfo < 'tcx > for crate :: type_check :: InstantiateOpaqueType < 'tcx > { fn fallback_error (& self , tcx : TyCtxt < 'tcx > , span : Span) -> Diag < 'tcx > { tcx . dcx () . create_err (HigherRankedLifetimeError { cause : None , span }) } fn base_universe (& self) -> ty :: UniverseIndex { self . base_universe . unwrap () } fn nice_error < 'infcx > (& self , mbcx : & mut MirBorrowckCtxt < '_ , 'infcx , 'tcx > , _cause : ObligationCause < 'tcx > , placeholder_region : ty :: Region < 'tcx > , error_region : Option < ty :: Region < 'tcx > > ,) -> Option < Diag < 'infcx > > { try_extract_error_from_region_constraints (mbcx . infcx , mbcx . mir_def_id () , placeholder_region , error_region , self . region_constraints . as_ref () . unwrap () , | vid | RegionVariableOrigin :: Nll (mbcx . regioncx . definitions [vid] . origin) , | vid | mbcx . regioncx . definitions [vid] . universe ,) } }
    };
}

impl_108!()