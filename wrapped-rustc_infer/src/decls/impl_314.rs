macro_rules! deps {
    () => {
        Obligation!();
    };
}

macro_rules! impl_314 {
    () => {
        deps!();
        impl < T : Eq > Eq for Obligation < '_ , T > { }
    };
}

impl_314!();