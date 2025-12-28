macro_rules! deps {
    () => {
        Locations!();
    };
}

macro_rules! OutlivesConstraint {
    () => {
        deps!();
        # [derive (Copy , Clone , PartialEq , Eq)] pub struct OutlivesConstraint < 'tcx > { # [doc = " The region SUP must outlive SUB..."] pub sup : RegionVid , # [doc = " Region that must be outlived."] pub sub : RegionVid , # [doc = " Where did this constraint arise?"] pub locations : Locations , # [doc = " The `Span` associated with the creation of this constraint."] # [doc = " This should be used in preference to obtaining the span from"] # [doc = " `locations`, since the `locations` may give a poor span"] # [doc = " in some cases (e.g. converting a constraint from a promoted)."] pub span : Span , # [doc = " What caused this constraint?"] pub category : ConstraintCategory < 'tcx > , # [doc = " Variance diagnostic information"] pub variance_info : VarianceDiagInfo < TyCtxt < 'tcx > > , # [doc = " If this constraint is promoted from closure requirements."] pub from_closure : bool , }
    };
}

OutlivesConstraint!()