macro_rules! deps {
    () => {
        IterMut!();
        HashTable!();
        IntoIter!();
    };
}

macro_rules! impl_473 {
    () => {
        deps!();
        impl < 'a , T , A > IntoIterator for & 'a mut HashTable < T , A > where A : Allocator , { type Item = & 'a mut T ; type IntoIter = IterMut < 'a , T > ; fn into_iter (self) -> IterMut < 'a , T > { self . iter_mut () } }
    };
}

impl_473!();