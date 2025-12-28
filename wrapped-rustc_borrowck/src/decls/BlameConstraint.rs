macro_rules! BlameConstraint {
    () => {
        # [derive (Clone , Debug)] pub (crate) struct BlameConstraint < 'tcx > { pub category : ConstraintCategory < 'tcx > , pub from_closure : bool , pub cause : ObligationCause < 'tcx > , pub variance_info : ty :: VarianceDiagInfo < TyCtxt < 'tcx > > , }
    };
}

BlameConstraint!();