macro_rules! deps {
    () => {
        IntPredicate!();
    };
}

macro_rules! impl_441 {
    () => {
        deps!();
        impl IntPredicate { pub (crate) fn from_generic (intpre : rustc_codegen_ssa :: common :: IntPredicate) -> Self { use rustc_codegen_ssa :: common :: IntPredicate as Common ; match intpre { Common :: IntEQ => Self :: IntEQ , Common :: IntNE => Self :: IntNE , Common :: IntUGT => Self :: IntUGT , Common :: IntUGE => Self :: IntUGE , Common :: IntULT => Self :: IntULT , Common :: IntULE => Self :: IntULE , Common :: IntSGT => Self :: IntSGT , Common :: IntSGE => Self :: IntSGE , Common :: IntSLT => Self :: IntSLT , Common :: IntSLE => Self :: IntSLE , } } }
    };
}

impl_441!();