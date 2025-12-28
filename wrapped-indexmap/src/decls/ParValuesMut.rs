macro_rules! deps {
    () => {
        Bucket!();
        IndexMap!();
    };
}

macro_rules! ParValuesMut {
    () => {
        deps!();
        # [doc = " A parallel mutable iterator over the values of an [`IndexMap`]."] # [doc = ""] # [doc = " This `struct` is created by the [`IndexMap::par_values_mut`] method."] # [doc = " See its documentation for more."] pub struct ParValuesMut < 'a , K , V > { entries : & 'a mut [Bucket < K , V >] , }
    };
}

ParValuesMut!();