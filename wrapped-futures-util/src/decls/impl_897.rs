macro_rules! deps {
    () => {
        SelectAll!();
        IntoIter!();
    };
}

macro_rules! impl_897 {
    () => {
        deps!();
        impl < St : Stream + Unpin > IntoIterator for SelectAll < St > { type Item = St ; type IntoIter = IntoIter < St > ; fn into_iter (self) -> Self :: IntoIter { IntoIter (self . inner . into_iter ()) } }
    };
}

impl_897!()