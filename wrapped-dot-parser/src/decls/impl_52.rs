macro_rules! deps {
    () => {
        AList!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl < A > IntoIterator for AList < A > { type Item = A ; type IntoIter = std :: vec :: IntoIter < Self :: Item > ; fn into_iter (self) -> Self :: IntoIter { self . elems . into_iter () } }
    };
}

impl_52!();