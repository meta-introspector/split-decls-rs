macro_rules! deps {
    () => {
        InferCtxt!();
        SubregionOrigin!();
        TypeOutlivesDelegate!();
        VerifyBound!();
        GenericKind!();
    };
}

macro_rules! impl_95 {
    () => {
        deps!();
        impl < 'cx , 'tcx > TypeOutlivesDelegate < 'tcx > for & 'cx InferCtxt < 'tcx > { fn push_sub_region_constraint (& mut self , origin : SubregionOrigin < 'tcx > , a : ty :: Region < 'tcx > , b : ty :: Region < 'tcx > , _constraint_category : ConstraintCategory < 'tcx > ,) { self . sub_regions (origin , a , b) } fn push_verify (& mut self , origin : SubregionOrigin < 'tcx > , kind : GenericKind < 'tcx > , a : ty :: Region < 'tcx > , bound : VerifyBound < 'tcx > ,) { self . verify_generic_bound (origin , kind , a , bound) } }
    };
}

impl_95!();