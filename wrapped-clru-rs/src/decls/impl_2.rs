macro_rules! deps {
    () => {
        CLruCacheConfig!();
        WeightScale!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl < K , V , S : BuildHasher , W : WeightScale < K , V > > CLruCacheConfig < K , V , S , W > { # [doc = " Configure the provided hash builder."] pub fn with_hasher < O : BuildHasher > (self , hash_builder : O) -> CLruCacheConfig < K , V , O , W > { let Self { capacity , reserve , scale , _marker , .. } = self ; CLruCacheConfig { capacity , hash_builder , reserve , scale , _marker , } } # [doc = " Configure the amount of pre-allocated memory in order to hold at least `reserve` elements"] # [doc = " without reallocating."] pub fn with_memory (mut self , reserve : usize) -> Self { self . reserve = Some (reserve) ; self } # [doc = " Configure the provided scale."] pub fn with_scale < O : WeightScale < K , V > > (self , scale : O) -> CLruCacheConfig < K , V , S , O > { let Self { capacity , hash_builder , reserve , .. } = self ; CLruCacheConfig { capacity , hash_builder , reserve , scale , _marker : PhantomData , } } }
    };
}

impl_2!();