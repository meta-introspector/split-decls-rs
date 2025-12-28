macro_rules! deps {
    () => {
        OwnedStorage!();
        Storage!();
    };
}

macro_rules! impl_201 {
    () => {
        deps!();
        impl < const N : usize > Storage for OwnedStorage < N > { }
    };
}

impl_201!()