macro_rules! deps {
    () => {
        VecInner!();
        IterMut!();
        LenType!();
        IntoIter!();
    };
}

macro_rules! impl_301 {
    () => {
        deps!();
        impl < 'a , T , LenT : LenType , S : VecStorage < T > + ? Sized > IntoIterator for & 'a mut VecInner < T , LenT , S > { type Item = & 'a mut T ; type IntoIter = slice :: IterMut < 'a , T > ; fn into_iter (self) -> Self :: IntoIter { self . iter_mut () } }
    };
}

impl_301!()