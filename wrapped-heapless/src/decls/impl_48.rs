macro_rules! deps {
    () => {
        IterMut!();
        DequeInner!();
        IntoIter!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl < 'a , T , S : VecStorage < T > + ? Sized > IntoIterator for & 'a mut DequeInner < T , S > { type Item = & 'a mut T ; type IntoIter = IterMut < 'a , T > ; fn into_iter (self) -> Self :: IntoIter { self . iter_mut () } }
    };
}

impl_48!();