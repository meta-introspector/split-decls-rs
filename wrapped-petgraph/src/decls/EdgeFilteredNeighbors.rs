macro_rules! deps {
    () => {
        Edges!();
    };
}

macro_rules! EdgeFilteredNeighbors {
    () => {
        deps!();
        # [doc = " A filtered neighbors iterator."] # [derive (Debug , Clone)] pub struct EdgeFilteredNeighbors < 'a , G , F : 'a > where G : IntoEdges , { iter : G :: Edges , f : & 'a F , }
    };
}

EdgeFilteredNeighbors!()