macro_rules! deps {
    () => {
        Mut!();
    };
}

macro_rules! impl_129 {
    () => {
        deps!();
        impl < 'a , T > Copy for Mut < 'a , T > where T : ? Sized { }
    };
}

impl_129!();