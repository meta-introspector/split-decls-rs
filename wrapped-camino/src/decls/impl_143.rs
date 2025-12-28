macro_rules! deps {
    () => {
        Utf8Path!();
        Iter!();
    };
}

macro_rules! impl_143 {
    () => {
        deps!();
        impl < 'a > IntoIterator for & 'a Utf8Path { type Item = & 'a str ; type IntoIter = Iter < 'a > ; # [inline] fn into_iter (self) -> Iter < 'a > { self . iter () } }
    };
}

impl_143!();