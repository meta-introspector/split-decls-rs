macro_rules! MatchedNodes {
    () => {
        pub struct MatchedNodes < 'a , G : GraphBase > { graph : & 'a G , mate : & 'a [Option < G :: NodeId >] , current : usize , }
    };
}

MatchedNodes!()