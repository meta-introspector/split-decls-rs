macro_rules! deps {
    () => {
        IeeeFloat!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl < S > Copy for IeeeFloat < S > { }
    };
}

impl_29!();