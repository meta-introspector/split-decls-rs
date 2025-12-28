macro_rules! deps {
    () => {
        SubregionOrigin!();
    };
}

macro_rules! TypeOutlivesConstraint {
    () => {
        deps!();
        # [doc = " See the `region_obligations` field for more information."] # [derive (Clone , Debug)] pub struct TypeOutlivesConstraint < 'tcx > { pub sub_region : ty :: Region < 'tcx > , pub sup_type : Ty < 'tcx > , pub origin : SubregionOrigin < 'tcx > , }
    };
}

TypeOutlivesConstraint!();