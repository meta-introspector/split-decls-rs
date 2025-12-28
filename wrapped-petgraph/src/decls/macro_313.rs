macro_rules! deps {
    () => {
        SomeIter!();
        EdgeReference!();
        IndexType!();
    };
}

macro_rules! macro_313 {
    () => {
        deps!();
        iterator_wrap ! { impl (Iterator) for # [doc = " Iterator over the [`EdgeReference`] of the outgoing edges from a node."] # [derive (Debug , Clone)] struct OutgoingEdgeReferences <'a , E , Ix > where { Ix : IndexType } item : EdgeReference <'a , E , Ix >, iter : SomeIter <'a , E , Ix >, }
    };
}

macro_313!();