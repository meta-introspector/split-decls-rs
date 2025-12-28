macro_rules! deps {
    () => {
        Node!();
        DefaultIx!();
    };
}

macro_rules! NodeIndices {
    () => {
        deps!();
        # [doc = " Iterator over the node indices of a graph."] # [derive (Debug , Clone)] pub struct NodeIndices < 'a , N : 'a , Ix : 'a = DefaultIx > { iter : iter :: Enumerate < slice :: Iter < 'a , Node < Option < N > , Ix > > > , }
    };
}

NodeIndices!()