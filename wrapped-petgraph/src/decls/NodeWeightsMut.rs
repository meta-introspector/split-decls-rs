macro_rules! deps {
    () => {
        IndexType!();
        DefaultIx!();
        Node!();
    };
}

macro_rules! NodeWeightsMut {
    () => {
        deps!();
        # [doc = " Iterator yielding mutable access to all node weights."] # [derive (Debug)] pub struct NodeWeightsMut < 'a , N : 'a , Ix : IndexType = DefaultIx > { nodes : :: core :: slice :: IterMut < 'a , Node < N , Ix > > , }
    };
}

NodeWeightsMut!();