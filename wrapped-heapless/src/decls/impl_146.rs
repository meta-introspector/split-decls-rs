macro_rules! deps {
    () => {
        IndexSet!();
        Iter!();
        IntoIter!();
    };
}

macro_rules! impl_146 {
    () => {
        deps!();
        impl < 'a , T , S , const N : usize > IntoIterator for & 'a IndexSet < T , S , N > where T : Eq + Hash , S : BuildHasher , { type Item = & 'a T ; type IntoIter = Iter < 'a , T > ; fn into_iter (self) -> Self :: IntoIter { self . iter () } }
    };
}

impl_146!()