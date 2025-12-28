macro_rules! deps {
    () => {
        Own!();
    };
}

macro_rules! impl_121 {
    () => {
        deps!();
        impl < T > Copy for Own < T > where T : ? Sized { }
    };
}

impl_121!();