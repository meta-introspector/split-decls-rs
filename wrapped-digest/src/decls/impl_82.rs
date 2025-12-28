macro_rules! deps {
    () => {
        ExtendableOutput!();
        XofFixedWrapper!();
    };
}

macro_rules! impl_82 {
    () => {
        deps!();
        impl < T : ExtendableOutput + Reset , S : ArraySize > Reset for XofFixedWrapper < T , S > { fn reset (& mut self) { self . hash . reset () ; } }
    };
}

impl_82!();