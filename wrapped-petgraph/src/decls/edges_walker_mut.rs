macro_rules! deps {
    () => {
        IndexType!();
        Direction!();
        EdgeIndex!();
        Edge!();
        EdgesWalkerMut!();
    };
}

macro_rules! edges_walker_mut {
    () => {
        deps!();
        fn edges_walker_mut < E , Ix > (edges : & mut [Edge < E , Ix >] , next : EdgeIndex < Ix > , dir : Direction ,) -> EdgesWalkerMut < '_ , E , Ix > where Ix : IndexType , { EdgesWalkerMut { edges , next , dir } }
    };
}

edges_walker_mut!();