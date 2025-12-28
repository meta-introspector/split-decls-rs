macro_rules! deps {
    () => {
        UnalignedIter!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl < 'a , T > Copy for UnalignedIter < 'a , T > { }
    };
}

impl_75!()