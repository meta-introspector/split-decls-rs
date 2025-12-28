macro_rules! deps {
    () => {
        CLruCache!();
        CLruCacheIterMut!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl < K , V , S > CLruCache < K , V , S > { # [doc = " Returns an iterator visiting all entries in order, giving a mutable reference on V."] # [doc = " The iterator element type is `(&'a K, &'a mut V)`."] pub fn iter_mut (& mut self) -> CLruCacheIterMut < '_ , K , V > { CLruCacheIterMut { iter : self . storage . iter_mut () , } } }
    };
}

impl_11!()