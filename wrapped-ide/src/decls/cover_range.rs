macro_rules! cover_range {
    () => {
        fn cover_range (r0 : Option < TextRange > , r1 : Option < TextRange >) -> Option < TextRange > { match (r0 , r1) { (Some (r0) , Some (r1)) => Some (r0 . cover (r1)) , (Some (range) , None) => Some (range) , (None , Some (range)) => Some (range) , (None , None) => None , } }
    };
}

cover_range!()