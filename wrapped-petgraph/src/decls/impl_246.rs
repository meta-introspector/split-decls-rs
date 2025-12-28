macro_rules! deps {
    () => {
        NodeTrait!();
        GraphMap!();
        Element!();
        FromElements!();
        EdgeType!();
    };
}

macro_rules! impl_246 {
    () => {
        deps!();
        # [cfg (feature = "graphmap")] impl < N , E , Ty , S > FromElements for GraphMap < N , E , Ty , S > where Ty : EdgeType , N : NodeTrait , S : BuildHasher + Default , { fn from_elements < I > (iterable : I) -> Self where Self : Sized , I : IntoIterator < Item = Element < Self :: NodeWeight , Self :: EdgeWeight > > , { from_elements_indexable (iterable) } }
    };
}

impl_246!();