macro_rules! deps {
    () => {
        HmacCore!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl < D : EagerHash > BufferKindUser for HmacCore < D > { type BufferKind = Eager ; }
    };
}

impl_3!()