macro_rules! deps {
    () => {
        Edge!();
        DefaultIx!();
    };
}

macro_rules! EdgeIndices {
    () => {
        deps!();
        # [doc = " Iterator over the edge indices of a graph."] # [doc = ""] # [doc = " Note: `EdgeIndices` borrows a graph."] # [derive (Debug , Clone)] pub struct EdgeIndices < 'a , E : 'a , Ix : 'a = DefaultIx > { iter : iter :: Enumerate < slice :: Iter < 'a , Edge < Option < E > , Ix > > > , }
    };
}

EdgeIndices!()