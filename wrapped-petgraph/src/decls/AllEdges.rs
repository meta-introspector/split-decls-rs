macro_rules! deps {
    () => {
        NodeTrait!();
    };
}

macro_rules! AllEdges {
    () => {
        deps!();
        # [derive (Debug , Clone)] pub struct AllEdges < 'a , N , E : 'a , Ty > where N : 'a + NodeTrait , { inner : IndexMapIter < 'a , (N , N) , E > , ty : PhantomData < Ty > , }
    };
}

AllEdges!()