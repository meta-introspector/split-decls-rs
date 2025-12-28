macro_rules! deps {
    () => {
        SingleAttributeParser!();
        Single!();
        Stage!();
    };
}

macro_rules! impl_245 {
    () => {
        deps!();
        impl < T : SingleAttributeParser < S > , S : Stage > Default for Single < T , S > { fn default () -> Self { Self (Default :: default () , Default :: default ()) } }
    };
}

impl_245!();