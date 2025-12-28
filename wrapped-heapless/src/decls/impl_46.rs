macro_rules! deps {
    () => {
        IntoIter!();
        Deque!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl < T , const N : usize > IntoIterator for Deque < T , N > { type Item = T ; type IntoIter = IntoIter < T , N > ; fn into_iter (self) -> Self :: IntoIter { IntoIter { deque : self } } }
    };
}

impl_46!()