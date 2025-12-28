macro_rules! deps {
    () => {
        HmacCore!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl < D : EagerHash > KeySizeUser for HmacCore < D > { type KeySize = < < D as EagerHash > :: Core as BlockSizeUser > :: BlockSize ; }
    };
}

impl_4!()