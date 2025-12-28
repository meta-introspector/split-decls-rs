macro_rules! deps {
    () => {
        Stage!();
        NoArgsAttributeParser!();
        WithoutArgs!();
    };
}

macro_rules! impl_252 {
    () => {
        deps!();
        impl < T : NoArgsAttributeParser < S > , S : Stage > Default for WithoutArgs < T , S > { fn default () -> Self { Self (Default :: default ()) } }
    };
}

impl_252!();