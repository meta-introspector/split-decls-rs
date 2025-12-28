macro_rules! deps {
    () => {
        NoArgsAttributeParser!();
        WithoutArgs!();
        Stage!();
    };
}

macro_rules! impl_252 {
    () => {
        deps!();
        impl < T : NoArgsAttributeParser < S > , S : Stage > Default for WithoutArgs < T , S > { fn default () -> Self { Self (Default :: default ()) } }
    };
}

impl_252!()