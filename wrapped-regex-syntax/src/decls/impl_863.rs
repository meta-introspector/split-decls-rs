macro_rules! deps {
    () => {
        Utf8Sequence!();
        Utf8Range!();
    };
}

macro_rules! impl_863 {
    () => {
        deps!();
        impl < 'a > IntoIterator for & 'a Utf8Sequence { type IntoIter = slice :: Iter < 'a , Utf8Range > ; type Item = & 'a Utf8Range ; fn into_iter (self) -> Self :: IntoIter { self . as_slice () . iter () } }
    };
}

impl_863!();