macro_rules! deps {
    () => {
        IdStorage!();
        IdIterator!();
    };
}

macro_rules! NodeReferences {
    () => {
        deps!();
        # [doc = " Iterator over all nodes of a graph."] # [doc = ""] # [doc = " Created from a call to [`.node_references()`][1] on a [`MatrixGraph`][2]."] # [doc = ""] # [doc = " [1]: ../visit/trait.IntoNodeReferences.html#tymethod.node_references"] # [doc = " [2]: struct.MatrixGraph.html"] # [derive (Debug , Clone)] pub struct NodeReferences < 'a , N : 'a , Ix , # [cfg (not (feature = "std"))] S , # [cfg (feature = "std")] S = RandomState , > { nodes : & 'a IdStorage < N , S > , iter : IdIterator < 'a , S > , ix : PhantomData < Ix > , }
    };
}

NodeReferences!()