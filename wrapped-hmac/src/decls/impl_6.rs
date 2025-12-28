macro_rules! deps {
    () => {
        HmacCore!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl < D : EagerHash > OutputSizeUser for HmacCore < D > { type OutputSize = < < D as EagerHash > :: Core as OutputSizeUser > :: OutputSize ; }
    };
}

impl_6!()