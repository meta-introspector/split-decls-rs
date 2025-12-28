macro_rules! deps {
    () => {
        ValueEnum!();
        EnumValueParser!();
    };
}

macro_rules! impl_282 {
    () => {
        deps!();
        impl < E : crate :: ValueEnum + Clone + Send + Sync + 'static > EnumValueParser < E > { # [doc = " Parse an [`ValueEnum`][crate::ValueEnum]"] pub fn new () -> Self { let phantom : std :: marker :: PhantomData < E > = Default :: default () ; Self (phantom) } }
    };
}

impl_282!()