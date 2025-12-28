macro_rules! deps {
    () => {
        List!();
        Iter!();
    };
}

macro_rules! impl_106 {
    () => {
        deps!();
        impl < 'a > IntoIterator for & 'a List { type IntoIter = Iter < 'a > ; type Item = & 'a [u8] ; fn into_iter (self) -> Iter < 'a > { self . iter () } }
    };
}

impl_106!();