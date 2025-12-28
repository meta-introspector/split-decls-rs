macro_rules! deps {
    () => {
        SliceCopyIter!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl < 'a , T > Clone for SliceCopyIter < 'a , T > { fn clone (& self) -> Self { * self } }
    };
}

impl_40!()