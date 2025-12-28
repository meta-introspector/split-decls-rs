macro_rules! deps {
    () => {
        MapValueParser!();
        TypedValueParser!();
    };
}

macro_rules! impl_316 {
    () => {
        deps!();
        impl < P , F , T > MapValueParser < P , F > where P : TypedValueParser , P :: Value : Send + Sync + Clone , F : Fn (P :: Value) -> T + Clone , T : Send + Sync + Clone , { fn new (parser : P , func : F) -> Self { Self { parser , func } } }
    };
}

impl_316!();