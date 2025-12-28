macro_rules! deps {
    () => {
        DefaultIx!();
        Node!();
        IndexType!();
    };
}

macro_rules! NodeWeights {
    () => {
        deps!();
        # [doc = " Iterator yielding immutable access to all node weights."] pub struct NodeWeights < 'a , N : 'a , Ix : IndexType = DefaultIx > { nodes : :: core :: slice :: Iter < 'a , Node < N , Ix > > , }
    };
}

NodeWeights!()