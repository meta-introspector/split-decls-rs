macro_rules! deps {
    () => {
        IKeyValuePair_Impl!();
    };
}

macro_rules! impl_99 {
    () => {
        deps!();
        impl < K , V > IKeyValuePair_Impl < K , V > for StockKeyValuePair_Impl < K , V > where K : RuntimeType , V : RuntimeType , K :: Default : Clone , V :: Default : Clone , { fn Key (& self) -> Result < K > { K :: from_default (& self . key) } fn Value (& self) -> Result < V > { V :: from_default (& self . value) } }
    };
}

impl_99!();