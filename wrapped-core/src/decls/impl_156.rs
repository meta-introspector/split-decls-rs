macro_rules! deps {
    () => {
        InterfaceRef!();
    };
}

macro_rules! impl_156 {
    () => {
        deps!();
        impl < I > Copy for InterfaceRef < '_ , I > { }
    };
}

impl_156!();