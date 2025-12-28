macro_rules! deps {
    () => {
        StoreConstEmpty!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl < K : ? Sized , V : ? Sized , S > LiteMap < K , V , S > where S : StoreConstEmpty < K , V > , { # [doc = " Create a new empty [`LiteMap`]"] pub const fn new () -> Self { Self { values : S :: EMPTY , _key_type : PhantomData , _value_type : PhantomData , } } }
    };
}

impl_9!()