macro_rules! deps {
    () => {
        IndexMap!();
        Bucket!();
    };
}

macro_rules! ParValues {
    () => {
        deps!();
        # [doc = " A parallel iterator over the values of an [`IndexMap`]."] # [doc = ""] # [doc = " This `struct` is created by the [`IndexMap::par_values`] method."] # [doc = " See its documentation for more."] pub struct ParValues < 'a , K , V > { entries : & 'a [Bucket < K , V >] , }
    };
}

ParValues!()