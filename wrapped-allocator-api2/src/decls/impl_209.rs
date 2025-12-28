macro_rules! deps {
    () => {
        Unique!();
    };
}

macro_rules! impl_209 {
    () => {
        deps!();
        impl < T : ? Sized > Copy for Unique < T > { }
    };
}

impl_209!();