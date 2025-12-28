macro_rules! deps {
    () => {
        RevSlice!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        impl < 'a , T > IntoIterator for & 'a RevSlice < T > { type Item = & 'a T ; type IntoIter = Rev < Iter < 'a , T > > ; fn into_iter (self) -> Self :: IntoIter { self . 0 . iter () . rev () } }
    };
}

impl_63!();