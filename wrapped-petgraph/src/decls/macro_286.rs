macro_rules! deps {
    () => {
        IndexType!();
        NodeIndex!();
        Neighbors!();
        RowIter!();
        WSuc!();
    };
}

macro_rules! macro_286 {
    () => {
        deps!();
        iterator_wrap ! { impl (Iterator DoubleEndedIterator ExactSizeIterator) for # [doc = " An iterator over the indices of the neighbors of a node."] # [derive (Debug , Clone)] struct Neighbors <'a , E , Ix > where { Ix : IndexType } item : NodeIndex < Ix >, iter : core :: iter :: Map < RowIter <'a , E , Ix >, fn (& WSuc < E , Ix >) -> NodeIndex < Ix >>, }
    };
}

macro_286!();