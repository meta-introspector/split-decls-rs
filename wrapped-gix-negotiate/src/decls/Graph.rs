macro_rules! deps {
    () => {
        Metadata!();
    };
}

macro_rules! Graph {
    () => {
        deps!();
        # [doc = " The graph our callers use to store traversal information, for (re-)use in the negotiation implementation."] pub type Graph < 'find , 'cache > = gix_revwalk :: Graph < 'find , 'cache , gix_revwalk :: graph :: Commit < Metadata > > ;
    };
}

Graph!()