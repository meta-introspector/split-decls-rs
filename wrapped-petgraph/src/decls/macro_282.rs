macro_rules! deps {
    () => {
        NodeIndex!();
        IndexType!();
        EdgeIndex!();
    };
}

macro_rules! macro_282 {
    () => {
        deps!();
        iterator_wrap ! { impl (Iterator) for # [doc = " An Iterator over the indices of the outgoing edges from a node."] # [doc = ""] # [doc = " It does not borrow the graph during iteration."] # [derive (Debug , Clone)] struct OutgoingEdgeIndices < Ix > where { Ix : IndexType } item : EdgeIndex < Ix >, iter : core :: iter :: Map < core :: iter :: Zip < Range < usize >, core :: iter :: Repeat < NodeIndex < Ix >>>, fn ((usize , NodeIndex < Ix >)) -> EdgeIndex < Ix >>, }
    };
}

macro_282!()