macro_rules! deps {
    () => {
        StoreIterable!();
        MapF!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        impl < 'a , K : 'a , V : 'a > StoreIterable < 'a , K , V > for & 'a [(K , V)] { type KeyValueIter = core :: iter :: Map < core :: slice :: Iter < 'a , (K , V) > , MapF < K , V > > ; # [inline] fn lm_iter (& 'a self) -> Self :: KeyValueIter { self . iter () . map (map_f) } }
    };
}

impl_71!();