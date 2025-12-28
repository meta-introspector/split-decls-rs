macro_rules! deps {
    () => {
        CompactDirection!();
        EdgeType!();
        Direction!();
    };
}

macro_rules! NeighborsDirected {
    () => {
        deps!();
        # [derive (Debug , Clone)] pub struct NeighborsDirected < 'a , N , Ty > where N : 'a , Ty : EdgeType , { iter : Iter < 'a , (N , CompactDirection) > , start_node : N , dir : Direction , ty : PhantomData < Ty > , }
    };
}

NeighborsDirected!()