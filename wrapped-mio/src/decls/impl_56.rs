macro_rules! deps {
    () => {
        Events!();
        Iter!();
        Event!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl < 'a > IntoIterator for & 'a Events { type Item = & 'a Event ; type IntoIter = Iter < 'a > ; fn into_iter (self) -> Self :: IntoIter { self . iter () } }
    };
}

impl_56!();