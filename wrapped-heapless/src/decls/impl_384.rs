macro_rules! deps {
    () => {
        Kind!();
        BinaryHeapInner!();
        IntoIter!();
        Iter!();
    };
}

macro_rules! impl_384 {
    () => {
        deps!();
        impl < 'a , T , K , S > IntoIterator for & 'a BinaryHeapInner < T , K , S > where K : Kind , T : Ord , S : VecStorage < T > + ? Sized , { type Item = & 'a T ; type IntoIter = slice :: Iter < 'a , T > ; fn into_iter (self) -> Self :: IntoIter { self . iter () } }
    };
}

impl_384!()