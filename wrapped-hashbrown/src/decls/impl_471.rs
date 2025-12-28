macro_rules! deps {
    () => {
        HashTable!();
        IntoIter!();
    };
}

macro_rules! impl_471 {
    () => {
        deps!();
        impl < T , A > IntoIterator for HashTable < T , A > where A : Allocator , { type Item = T ; type IntoIter = IntoIter < T , A > ; fn into_iter (self) -> IntoIter < T , A > { IntoIter { inner : self . raw . into_iter () , } } }
    };
}

impl_471!();