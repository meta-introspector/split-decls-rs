macro_rules! deps {
    () => {
        VerifyBound!();
        GenericKind!();
        SubregionOrigin!();
    };
}

macro_rules! TypeOutlivesDelegate {
    () => {
        deps!();
        pub trait TypeOutlivesDelegate < 'tcx > { fn push_sub_region_constraint (& mut self , origin : SubregionOrigin < 'tcx > , a : ty :: Region < 'tcx > , b : ty :: Region < 'tcx > , constraint_category : ConstraintCategory < 'tcx > ,) ; fn push_verify (& mut self , origin : SubregionOrigin < 'tcx > , kind : GenericKind < 'tcx > , a : ty :: Region < 'tcx > , bound : VerifyBound < 'tcx > ,) ; }
    };
}

TypeOutlivesDelegate!();