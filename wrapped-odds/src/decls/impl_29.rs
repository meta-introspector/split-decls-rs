macro_rules! deps {
    () => {
        BlockedIter!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl < 'a , B , T > Clone for BlockedIter < 'a , B , T > { fn clone (& self) -> Self { * self } }
    };
}

impl_29!()