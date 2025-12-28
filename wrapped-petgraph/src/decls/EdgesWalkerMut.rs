macro_rules! deps {
    () => {
        EdgeIndex!();
        Direction!();
        Edge!();
        IndexType!();
        DefaultIx!();
    };
}

macro_rules! EdgesWalkerMut {
    () => {
        deps!();
        struct EdgesWalkerMut < 'a , E : 'a , Ix : IndexType = DefaultIx > { edges : & 'a mut [Edge < E , Ix >] , next : EdgeIndex < Ix > , dir : Direction , }
    };
}

EdgesWalkerMut!()