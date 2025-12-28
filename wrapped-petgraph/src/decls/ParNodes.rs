macro_rules! deps {
    () => {
        NodeTrait!();
        CompactDirection!();
    };
}

macro_rules! ParNodes {
    () => {
        deps!();
        # [doc = " A [ParallelIterator] over this graph's nodes."] # [cfg (feature = "rayon")] pub struct ParNodes < 'a , N > where N : NodeTrait + Send + Sync , { iter : ParKeys < 'a , N , Vec < (N , CompactDirection) > > , }
    };
}

ParNodes!();