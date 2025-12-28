macro_rules! node_set {
    () => {
        fn node_set < 'q > (query : & 'q DepGraphQuery , filter : & DepNodeFilter ,) -> Option < FxIndexSet < & 'q DepNode > > { debug ! ("node_set(filter={:?})" , filter) ; if filter . accepts_all () { return None ; } Some (query . nodes () . into_iter () . filter (| n | filter . test (n)) . collect ()) }
    };
}

node_set!();