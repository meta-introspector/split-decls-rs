macro_rules! deps {
    () => {
        Arc!();
    };
}

macro_rules! impl_144 {
    () => {
        deps!();
        impl < T : ? Sized + Eq > Eq for Arc < T > { }
    };
}

impl_144!();