macro_rules! deps {
    () => {
        IntoIter!();
        Allocator!();
        Vec!();
    };
}

macro_rules! impl_168 {
    () => {
        deps!();
        impl < 'a , T , A : Allocator > IntoIterator for & 'a mut Vec < T , A > { type Item = & 'a mut T ; type IntoIter = slice :: IterMut < 'a , T > ; fn into_iter (self) -> Self :: IntoIter { self . iter_mut () } }
    };
}

impl_168!();