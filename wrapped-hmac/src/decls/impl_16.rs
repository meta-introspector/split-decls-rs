macro_rules! deps {
    () => {
        HmacResetCore!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl < D : EagerHash > KeySizeUser for HmacResetCore < D > { type KeySize = < < D as EagerHash > :: Core as BlockSizeUser > :: BlockSize ; }
    };
}

impl_16!()