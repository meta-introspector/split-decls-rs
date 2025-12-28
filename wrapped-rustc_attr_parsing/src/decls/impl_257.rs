macro_rules! deps {
    () => {
        CombineAttributeParser!();
        Stage!();
        Combine!();
    };
}

macro_rules! impl_257 {
    () => {
        deps!();
        impl < T : CombineAttributeParser < S > , S : Stage > Default for Combine < T , S > { fn default () -> Self { Self { phantom : Default :: default () , items : Default :: default () , first_span : Default :: default () , } } }
    };
}

impl_257!()