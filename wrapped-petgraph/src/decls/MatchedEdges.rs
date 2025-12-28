macro_rules! MatchedEdges {
    () => {
        pub struct MatchedEdges < 'a , G : GraphBase > { graph : & 'a G , mate : & 'a [Option < G :: NodeId >] , current : usize , }
    };
}

MatchedEdges!();