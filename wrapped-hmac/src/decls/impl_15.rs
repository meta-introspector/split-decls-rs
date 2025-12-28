macro_rules! deps {
    () => {
        HmacResetCore!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl < D : EagerHash > BufferKindUser for HmacResetCore < D > { type BufferKind = Eager ; }
    };
}

impl_15!();