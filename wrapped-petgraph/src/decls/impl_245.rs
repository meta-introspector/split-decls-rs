macro_rules! deps {
    () => {
        StableGraph!();
        EdgeType!();
        IndexType!();
        Element!();
        FromElements!();
    };
}

macro_rules! impl_245 {
    () => {
        deps!();
        # [cfg (feature = "stable_graph")] impl < N , E , Ty , Ix > FromElements for StableGraph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { fn from_elements < I > (iterable : I) -> Self where Self : Sized , I : IntoIterator < Item = Element < Self :: NodeWeight , Self :: EdgeWeight > > , { from_elements_indexable (iterable) } }
    };
}

impl_245!();