macro_rules! deps {
    () => {
        Iter!();
        IntoIter!();
        LinearMapInner!();
    };
}

macro_rules! impl_182 {
    () => {
        deps!();
        impl < 'a , K , V , S : LinearMapStorage < K , V > + ? Sized > IntoIterator for & 'a LinearMapInner < K , V , S > where K : Eq , { type Item = (& 'a K , & 'a V) ; type IntoIter = Iter < 'a , K , V > ; fn into_iter (self) -> Self :: IntoIter { self . iter () } }
    };
}

impl_182!();