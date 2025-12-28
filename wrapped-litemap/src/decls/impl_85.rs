macro_rules! deps {
    () => {
        StoreIterableMut!();
        MapFMut!();
    };
}

macro_rules! impl_85 {
    () => {
        deps!();
        impl < 'a , K : 'a , V : 'a > StoreIterableMut < 'a , K , V > for Vec < (K , V) > { type KeyValueIterMut = core :: iter :: Map < core :: slice :: IterMut < 'a , (K , V) > , MapFMut < K , V > > ; # [inline] fn lm_iter_mut (& 'a mut self) -> Self :: KeyValueIterMut { self . as_mut_slice () . iter_mut () . map (map_f_mut) } }
    };
}

impl_85!()