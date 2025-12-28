macro_rules! deps {
    () => {
        HmacCore!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl < D : EagerHash > BlockSizeUser for HmacCore < D > { type BlockSize = < < D as EagerHash > :: Core as BlockSizeUser > :: BlockSize ; }
    };
}

impl_5!();