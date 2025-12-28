macro_rules! deps {
    () => {
        HmacResetCore!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl < D : EagerHash > BlockSizeUser for HmacResetCore < D > { type BlockSize = < < D as EagerHash > :: Core as BlockSizeUser > :: BlockSize ; }
    };
}

impl_17!();