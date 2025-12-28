macro_rules! deps {
    () => {
        Create!();
        Element!();
        Node!();
        Edge!();
    };
}

macro_rules! from_elements_indexable {
    () => {
        deps!();
        fn from_elements_indexable < G , I > (iterable : I) -> G where G : Create + NodeIndexable , I : IntoIterator < Item = Element < G :: NodeWeight , G :: EdgeWeight > > , { let mut gr = G :: with_capacity (0 , 0) ; let map = | gr : & G , i | gr . from_index (i) ; for element in iterable { match element { Element :: Node { weight } => { gr . add_node (weight) ; } Element :: Edge { source , target , weight , } => { let from = map (& gr , source) ; let to = map (& gr , target) ; gr . add_edge (from , to , weight) ; } } } gr }
    };
}

from_elements_indexable!()