macro_rules! deps {
    () => {
        AttrList!();
        AList!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl < A > IntoIterator for AttrList < A > { type Item = AList < A > ; type IntoIter = std :: vec :: IntoIter < Self :: Item > ; fn into_iter (self) -> Self :: IntoIter { self . elems . into_iter () } }
    };
}

impl_43!()