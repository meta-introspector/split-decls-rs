macro_rules! impl_7 {
    () => {
        impl < K , V , S > LiteMap < K , V , S > { # [doc = " Construct a new [`LiteMap`] using the given values"] # [doc = ""] # [doc = " The store must be sorted and have no duplicate keys."] pub const fn from_sorted_store_unchecked (values : S) -> Self { Self { values , _key_type : PhantomData , _value_type : PhantomData , } } }
    };
}

impl_7!();