macro_rules! deps {
    () => {
        Storage!();
        OwnedStorage!();
    };
}

macro_rules! impl_201 {
    () => {
        deps!();
        impl < const N : usize > Storage for OwnedStorage < N > { }
    };
}

impl_201!();