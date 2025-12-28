macro_rules! deps {
    () => {
        TypedValueParser!();
        Error!();
        Result!();
        TryMapValueParser!();
    };
}

macro_rules! impl_319 {
    () => {
        deps!();
        impl < P , F , T , E > TryMapValueParser < P , F > where P : TypedValueParser , P :: Value : Send + Sync + Clone , F : Fn (P :: Value) -> Result < T , E > + Clone + Send + Sync + 'static , T : Send + Sync + Clone , E : Into < Box < dyn std :: error :: Error + Send + Sync + 'static > > , { fn new (parser : P , func : F) -> Self { Self { parser , func } } }
    };
}

impl_319!()