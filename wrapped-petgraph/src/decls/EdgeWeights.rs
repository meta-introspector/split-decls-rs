macro_rules! deps {
    () => {
        Edge!();
        DefaultIx!();
        IndexType!();
    };
}

macro_rules! EdgeWeights {
    () => {
        deps!();
        # [doc = " Iterator yielding immutable access to all edge weights."] pub struct EdgeWeights < 'a , E : 'a , Ix : IndexType = DefaultIx > { edges : :: core :: slice :: Iter < 'a , Edge < E , Ix > > , }
    };
}

EdgeWeights!();