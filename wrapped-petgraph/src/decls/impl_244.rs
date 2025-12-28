macro_rules! deps {
    () => {
        IndexType!();
        EdgeType!();
        FromElements!();
        Graph!();
        Element!();
    };
}

macro_rules! impl_244 {
    () => {
        deps!();
        impl < N , E , Ty , Ix > FromElements for Graph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { fn from_elements < I > (iterable : I) -> Self where Self : Sized , I : IntoIterator < Item = Element < Self :: NodeWeight , Self :: EdgeWeight > > , { from_elements_indexable (iterable) } }
    };
}

impl_244!();