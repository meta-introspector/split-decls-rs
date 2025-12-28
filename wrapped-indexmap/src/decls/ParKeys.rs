macro_rules! deps {
    () => {
        IndexMap!();
        Bucket!();
    };
}

macro_rules! ParKeys {
    () => {
        deps!();
        # [doc = " A parallel iterator over the keys of an [`IndexMap`]."] # [doc = ""] # [doc = " This `struct` is created by the [`IndexMap::par_keys`] method."] # [doc = " See its documentation for more."] pub struct ParKeys < 'a , K , V > { entries : & 'a [Bucket < K , V >] , }
    };
}

ParKeys!()