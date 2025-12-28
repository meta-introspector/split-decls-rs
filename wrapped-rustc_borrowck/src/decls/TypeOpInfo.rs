macro_rules! deps {
    () => {
        MirBorrowckCtxt!();
        RegionElement!();
    };
}

macro_rules! TypeOpInfo {
    () => {
        deps!();
        # [allow (unused_lifetimes)] pub (crate) trait TypeOpInfo < 'tcx > { # [doc = " Returns an error to be reported if rerunning the type op fails to"] # [doc = " recover the error's cause."] fn fallback_error (& self , tcx : TyCtxt < 'tcx > , span : Span) -> Diag < 'tcx > ; fn base_universe (& self) -> ty :: UniverseIndex ; fn nice_error < 'infcx > (& self , mbcx : & mut MirBorrowckCtxt < '_ , 'infcx , 'tcx > , cause : ObligationCause < 'tcx > , placeholder_region : ty :: Region < 'tcx > , error_region : Option < ty :: Region < 'tcx > > ,) -> Option < Diag < 'infcx > > ; # [doc = " Constraints require that `error_element` appear in the"] # [doc = "  values of `placeholder`, but this cannot be proven to"] # [doc = " hold. Report an error."] # [instrument (level = "debug" , skip (self , mbcx))] fn report_erroneous_element (& self , mbcx : & mut MirBorrowckCtxt < '_ , '_ , 'tcx > , placeholder : ty :: PlaceholderRegion , error_element : RegionElement , cause : ObligationCause < 'tcx > ,) { let tcx = mbcx . infcx . tcx ; let base_universe = self . base_universe () ; debug ! (? base_universe) ; let Some (adjusted_universe) = placeholder . universe . as_u32 () . checked_sub (base_universe . as_u32 ()) else { mbcx . buffer_error (self . fallback_error (tcx , cause . span)) ; return ; } ; let placeholder_region = ty :: Region :: new_placeholder (tcx , ty :: Placeholder { universe : adjusted_universe . into () , bound : placeholder . bound } ,) ; let error_region = if let RegionElement :: PlaceholderRegion (error_placeholder) = error_element { let adjusted_universe = error_placeholder . universe . as_u32 () . checked_sub (base_universe . as_u32 ()) ; adjusted_universe . map (| adjusted | { ty :: Region :: new_placeholder (tcx , ty :: Placeholder { universe : adjusted . into () , bound : error_placeholder . bound } ,) }) } else { None } ; debug ! (? placeholder_region) ; let span = cause . span ; let nice_error = self . nice_error (mbcx , cause , placeholder_region , error_region) ; debug ! (? nice_error) ; mbcx . buffer_error (nice_error . unwrap_or_else (| | self . fallback_error (tcx , span))) ; } }
    };
}

TypeOpInfo!();