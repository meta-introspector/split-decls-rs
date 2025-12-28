macro_rules! deps {
    () => {
        SubregionOrigin!();
    };
}

macro_rules! impl_259 {
    () => {
        deps!();
        impl < 'tcx > SubregionOrigin < 'tcx > { pub fn to_constraint_category (& self) -> ConstraintCategory < 'tcx > { match self { Self :: Subtype (type_trace) => type_trace . cause . to_constraint_category () , Self :: AscribeUserTypeProvePredicate (span) => ConstraintCategory :: Predicate (* span) , _ => ConstraintCategory :: BoringNoLocation , } } }
    };
}

impl_259!()