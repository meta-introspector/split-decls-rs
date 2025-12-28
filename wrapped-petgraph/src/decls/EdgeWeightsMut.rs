macro_rules! deps {
    () => {
        IndexType!();
        Edge!();
        DefaultIx!();
    };
}

macro_rules! EdgeWeightsMut {
    () => {
        deps!();
        # [doc = " Iterator yielding mutable access to all edge weights."] # [derive (Debug)] pub struct EdgeWeightsMut < 'a , E : 'a , Ix : IndexType = DefaultIx > { edges : :: core :: slice :: IterMut < 'a , Edge < E , Ix > > , }
    };
}

EdgeWeightsMut!()