macro_rules! deps {
    () => {
        Iter!();
        IntoIter!();
        SelectAll!();
    };
}

macro_rules! impl_898 {
    () => {
        deps!();
        impl < 'a , St : Stream + Unpin > IntoIterator for & 'a SelectAll < St > { type Item = & 'a St ; type IntoIter = Iter < 'a , St > ; fn into_iter (self) -> Self :: IntoIter { self . iter () } }
    };
}

impl_898!()