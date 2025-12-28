macro_rules! deps {
    () => {
        UsageKind!();
    };
}

macro_rules! impl_310 {
    () => {
        deps!();
        impl < 'tcx > UsageKind < 'tcx > { fn merge (& mut self , other : UsageKind < 'tcx >) { match (& * self , & other) { (UsageKind :: HasDefiningUse , _) | (_ , UsageKind :: None) => unreachable ! () , (UsageKind :: None , _) => * self = other , (UsageKind :: NonDefiningUse (..) | UsageKind :: UnconstrainedHiddenType (..) , UsageKind :: NonDefiningUse (..) ,) => { } (UsageKind :: NonDefiningUse (..) | UsageKind :: UnconstrainedHiddenType (..) , UsageKind :: UnconstrainedHiddenType (..) | UsageKind :: HasDefiningUse ,) => * self = other , } } }
    };
}

impl_310!();