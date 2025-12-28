macro_rules! deps {
    () => {
        XofFixedWrapper!();
        ExtendableOutput!();
    };
}

macro_rules! impl_82 {
    () => {
        deps!();
        impl < T : ExtendableOutput + Reset , S : ArraySize > Reset for XofFixedWrapper < T , S > { fn reset (& mut self) { self . hash . reset () ; } }
    };
}

impl_82!()