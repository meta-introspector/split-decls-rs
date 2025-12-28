macro_rules! deps {
    () => {
        RevSlice!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        impl < 'a , T > IntoIterator for & 'a mut RevSlice < T > { type Item = & 'a mut T ; type IntoIter = Rev < IterMut < 'a , T > > ; fn into_iter (self) -> Self :: IntoIter { self . 0 . iter_mut () . rev () } }
    };
}

impl_64!();