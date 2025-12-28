macro_rules! deps {
    () => {
        UnalignedIter!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        impl < 'a , T > Clone for UnalignedIter < 'a , T > { fn clone (& self) -> Self { * self } }
    };
}

impl_76!();