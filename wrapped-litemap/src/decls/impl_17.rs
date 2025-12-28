macro_rules! deps {
    () => {
        Store!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl < K , V , S > Default for LiteMap < K , V , S > where S : Store < K , V > + Default , { fn default () -> Self { Self { values : S :: default () , _key_type : PhantomData , _value_type : PhantomData , } } }
    };
}

impl_17!();