macro_rules! deps {
    () => {
        HmacResetCore!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl < D : EagerHash > OutputSizeUser for HmacResetCore < D > { type OutputSize = < < D as EagerHash > :: Core as OutputSizeUser > :: OutputSize ; }
    };
}

impl_18!();