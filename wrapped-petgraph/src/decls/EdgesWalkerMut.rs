macro_rules! deps {
    () => {
        DefaultIx!();
        IndexType!();
        EdgeIndex!();
        Direction!();
        Edge!();
    };
}

macro_rules! EdgesWalkerMut {
    () => {
        deps!();
        struct EdgesWalkerMut < 'a , E : 'a , Ix : IndexType = DefaultIx > { edges : & 'a mut [Edge < E , Ix >] , next : EdgeIndex < Ix > , dir : Direction , }
    };
}

EdgesWalkerMut!();