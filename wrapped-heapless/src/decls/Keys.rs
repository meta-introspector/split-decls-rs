macro_rules! deps {
    () => {
        Iter!();
        IndexMap!();
        Bucket!();
    };
}

macro_rules! Keys {
    () => {
        deps!();
        # [doc = " An iterator over the keys of a [`IndexMap`]."] # [doc = ""] # [doc = " This `struct` is created by the [`keys`](IndexMap::keys) method on [`IndexMap`]. See its"] # [doc = " documentation for more."] pub struct Keys < 'a , K , V > { iter : slice :: Iter < 'a , Bucket < K , V > > , }
    };
}

Keys!();