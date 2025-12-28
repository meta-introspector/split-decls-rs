macro_rules! deps {
    () => {
        ConstraintConversion!();
    };
}

macro_rules! impl_426 {
    () => {
        deps!();
        impl < 'a , 'b , 'tcx > TypeOutlivesDelegate < 'tcx > for & 'a mut ConstraintConversion < 'b , 'tcx > { fn push_sub_region_constraint (& mut self , _origin : SubregionOrigin < 'tcx > , a : ty :: Region < 'tcx > , b : ty :: Region < 'tcx > , constraint_category : ConstraintCategory < 'tcx > ,) { let b = self . to_region_vid (b) ; let a = self . to_region_vid (a) ; self . add_outlives (b , a , constraint_category) ; } fn push_verify (& mut self , _origin : SubregionOrigin < 'tcx > , kind : GenericKind < 'tcx > , a : ty :: Region < 'tcx > , bound : VerifyBound < 'tcx > ,) { let kind = self . replace_placeholders_with_nll (kind) ; let bound = self . replace_placeholders_with_nll (bound) ; let type_test = self . verify_to_type_test (kind , a , bound) ; self . add_type_test (type_test) ; } }
    };
}

impl_426!();