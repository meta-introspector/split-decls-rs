macro_rules! deps {
    () => {
        IterMut!();
        SelectAll!();
        IntoIter!();
    };
}

macro_rules! impl_899 {
    () => {
        deps!();
        impl < 'a , St : Stream + Unpin > IntoIterator for & 'a mut SelectAll < St > { type Item = & 'a mut St ; type IntoIter = IterMut < 'a , St > ; fn into_iter (self) -> Self :: IntoIter { self . iter_mut () } }
    };
}

impl_899!();