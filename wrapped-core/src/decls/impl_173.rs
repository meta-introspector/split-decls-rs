macro_rules! deps {
    () => {
        OutRef!();
    };
}

macro_rules! impl_173 {
    () => {
        deps!();
        impl < T : Type < T > > Default for OutRef < '_ , T > { fn default () -> Self { unsafe { core :: mem :: zeroed () } } }
    };
}

impl_173!()