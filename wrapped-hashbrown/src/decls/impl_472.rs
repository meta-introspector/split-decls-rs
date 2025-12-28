macro_rules! deps {
    () => {
        IntoIter!();
        Iter!();
        HashTable!();
    };
}

macro_rules! impl_472 {
    () => {
        deps!();
        impl < 'a , T , A > IntoIterator for & 'a HashTable < T , A > where A : Allocator , { type Item = & 'a T ; type IntoIter = Iter < 'a , T > ; fn into_iter (self) -> Iter < 'a , T > { self . iter () } }
    };
}

impl_472!()