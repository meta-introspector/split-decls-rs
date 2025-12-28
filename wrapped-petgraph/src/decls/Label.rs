macro_rules! deps {
    () => {
        Edge!();
    };
}

macro_rules! Label {
    () => {
        deps!();
        # [derive (Clone , Copy , Default)] enum Label < G : GraphBase > { # [default] None , Start , Vertex (G :: NodeId) , Edge (G :: EdgeId , [G :: NodeId ; 2]) , Flag (G :: EdgeId) , }
    };
}

Label!()