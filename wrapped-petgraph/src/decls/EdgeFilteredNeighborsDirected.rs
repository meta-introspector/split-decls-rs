macro_rules! deps {
    () => {
        EdgesDirected!();
    };
}

macro_rules! EdgeFilteredNeighborsDirected {
    () => {
        deps!();
        # [doc = " A filtered neighbors-directed iterator."] # [derive (Debug , Clone)] pub struct EdgeFilteredNeighborsDirected < 'a , G , F : 'a > where G : IntoEdgesDirected , { iter : G :: EdgesDirected , f : & 'a F , from : G :: NodeId , }
    };
}

EdgeFilteredNeighborsDirected!()