macro_rules! deps {
    () => {
        Iter!();
        Utf8Path!();
    };
}

macro_rules! impl_126 {
    () => {
        deps!();
        impl < 'a > IntoIterator for & 'a Utf8Path { type Item = & 'a str ; type IntoIter = Iter < 'a > ; # [inline] fn into_iter (self) -> Iter < 'a > { self . iter () } }
    };
}

impl_126!()