macro_rules! deps {
    () => {
        NodeTrait!();
        Direction!();
        EdgeType!();
        NeighborsDirected!();
    };
}

macro_rules! EdgesDirected {
    () => {
        deps!();
        # [derive (Debug , Clone)] pub struct EdgesDirected < 'a , N , E : 'a , Ty , # [cfg (not (feature = "std"))] S , # [cfg (feature = "std")] S = RandomState , > where N : 'a + NodeTrait , Ty : EdgeType , S : BuildHasher , { from : N , dir : Direction , edges : & 'a IndexMap < (N , N) , E , S > , iter : NeighborsDirected < 'a , N , Ty > , }
    };
}

EdgesDirected!()