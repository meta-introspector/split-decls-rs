macro_rules! deps {
    () => {
        RawParIter!();
        HashMap!();
    };
}

macro_rules! ParKeys {
    () => {
        deps!();
        # [doc = " Parallel iterator over shared references to keys in a map."] # [doc = ""] # [doc = " This iterator is created by the [`par_keys`] method on [`HashMap`]."] # [doc = " See its documentation for more."] # [doc = ""] # [doc = " [`par_keys`]: /hashbrown/struct.HashMap.html#method.par_keys"] # [doc = " [`HashMap`]: /hashbrown/struct.HashMap.html"] pub struct ParKeys < 'a , K , V > { inner : RawParIter < (K , V) > , marker : PhantomData < (& 'a K , & 'a V) > , }
    };
}

ParKeys!()