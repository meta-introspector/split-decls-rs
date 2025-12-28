macro_rules! deps {
    () => {
        StringArray!();
        Iter!();
    };
}

macro_rules! impl_154 {
    () => {
        deps!();
        impl < 'a > IntoIterator for & 'a StringArray { type Item = Option < & 'a str > ; type IntoIter = Iter < 'a > ; fn into_iter (self) -> Self :: IntoIter { self . iter () } }
    };
}

impl_154!();