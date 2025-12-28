macro_rules! HashMapCache {
    () => {
        # [doc = " [std::collections::HashMap] cache."] pub struct HashMapCache < S = RandomState > { _mark : PhantomData < S > , }
    };
}

HashMapCache!()