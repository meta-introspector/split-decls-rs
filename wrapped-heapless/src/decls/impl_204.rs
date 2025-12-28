macro_rules! deps {
    () => {
        Storage!();
        ViewStorage!();
    };
}

macro_rules! impl_204 {
    () => {
        deps!();
        impl Storage for ViewStorage { }
    };
}

impl_204!();