macro_rules! deps {
    () => {
        DequeInner!();
        Iter!();
        IntoIter!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl < 'a , T , S : VecStorage < T > + ? Sized > IntoIterator for & 'a DequeInner < T , S > { type Item = & 'a T ; type IntoIter = Iter < 'a , T > ; fn into_iter (self) -> Self :: IntoIter { self . iter () } }
    };
}

impl_47!()