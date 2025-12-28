macro_rules! deps {
    () => {
        FlatSet!();
        Iter!();
    };
}

macro_rules! impl_634 {
    () => {
        deps!();
        impl < 's , T : PartialEq + Eq > IntoIterator for & 's FlatSet < T > { type Item = & 's T ; type IntoIter = std :: slice :: Iter < 's , T > ; fn into_iter (self) -> Self :: IntoIter { self . inner . iter () } }
    };
}

impl_634!();