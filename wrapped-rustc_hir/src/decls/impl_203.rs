macro_rules! deps {
    () => {
        Body!();
        BodyId!();
    };
}

macro_rules! impl_203 {
    () => {
        deps!();
        impl < 'hir > Body < 'hir > { pub fn id (& self) -> BodyId { BodyId { hir_id : self . value . hir_id } } }
    };
}

impl_203!();