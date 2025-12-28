macro_rules! deps {
    () => {
        RawParIter!();
        HashMap!();
    };
}

macro_rules! ParValues {
    () => {
        deps!();
        # [doc = " Parallel iterator over shared references to values in a map."] # [doc = ""] # [doc = " This iterator is created by the [`par_values`] method on [`HashMap`]."] # [doc = " See its documentation for more."] # [doc = ""] # [doc = " [`par_values`]: /hashbrown/struct.HashMap.html#method.par_values"] # [doc = " [`HashMap`]: /hashbrown/struct.HashMap.html"] pub struct ParValues < 'a , K , V > { inner : RawParIter < (K , V) > , marker : PhantomData < (& 'a K , & 'a V) > , }
    };
}

ParValues!()