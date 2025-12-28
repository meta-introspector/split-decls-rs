macro_rules! deps {
    () => {
        IndexMap!();
        Bucket!();
        Iter!();
    };
}

macro_rules! Values {
    () => {
        deps!();
        # [doc = " An iterator over the values of a [`IndexMap`]."] # [doc = ""] # [doc = " This `struct` is created by the [`values`](IndexMap::values) method on [`IndexMap`]. See its"] # [doc = " documentation for more."] pub struct Values < 'a , K , V > { iter : slice :: Iter < 'a , Bucket < K , V > > , }
    };
}

Values!();