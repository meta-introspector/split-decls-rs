macro_rules! deps {
    () => {
        NodeTrait!();
    };
}

macro_rules! ParAllEdges {
    () => {
        deps!();
        # [doc = " A [ParallelIterator] over this graph's edges."] # [cfg (feature = "rayon")] pub struct ParAllEdges < 'a , N , E , Ty > where N : NodeTrait + Send + Sync , E : Sync , { inner : ParIter < 'a , (N , N) , E > , ty : PhantomData < fn (Ty) > , }
    };
}

ParAllEdges!();