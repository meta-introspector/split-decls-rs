macro_rules! deps {
    () => {
        StoreIterable!();
        MapF!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl < 'a , K : 'a , V : 'a > StoreIterable < 'a , K , V > for Vec < (K , V) > { type KeyValueIter = core :: iter :: Map < core :: slice :: Iter < 'a , (K , V) > , MapF < K , V > > ; # [inline] fn lm_iter (& 'a self) -> Self :: KeyValueIter { self . as_slice () . iter () . map (map_f) } }
    };
}

impl_84!();