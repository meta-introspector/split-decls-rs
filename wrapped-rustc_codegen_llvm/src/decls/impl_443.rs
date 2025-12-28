macro_rules! deps {
    () => {
        RealPredicate!();
    };
}

macro_rules! impl_443 {
    () => {
        deps!();
        impl RealPredicate { pub (crate) fn from_generic (realp : rustc_codegen_ssa :: common :: RealPredicate) -> Self { use rustc_codegen_ssa :: common :: RealPredicate as Common ; match realp { Common :: RealPredicateFalse => Self :: RealPredicateFalse , Common :: RealOEQ => Self :: RealOEQ , Common :: RealOGT => Self :: RealOGT , Common :: RealOGE => Self :: RealOGE , Common :: RealOLT => Self :: RealOLT , Common :: RealOLE => Self :: RealOLE , Common :: RealONE => Self :: RealONE , Common :: RealORD => Self :: RealORD , Common :: RealUNO => Self :: RealUNO , Common :: RealUEQ => Self :: RealUEQ , Common :: RealUGT => Self :: RealUGT , Common :: RealUGE => Self :: RealUGE , Common :: RealULT => Self :: RealULT , Common :: RealULE => Self :: RealULE , Common :: RealUNE => Self :: RealUNE , Common :: RealPredicateTrue => Self :: RealPredicateTrue , } } }
    };
}

impl_443!()