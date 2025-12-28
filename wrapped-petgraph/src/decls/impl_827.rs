macro_rules! deps {
    () => {
        Edges!();
        IndexType!();
        Edge!();
        EdgeReference!();
        EdgeType!();
    };
}

macro_rules! impl_827 {
    () => {
        deps!();
        impl < 'a , E , Ty , Ix > Iterator for Edges < 'a , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type Item = EdgeReference < 'a , E , Ix > ; fn next (& mut self) -> Option < Self :: Item > { let (iterate_over , reverse) = if Ty :: is_directed () { (Some (self . direction) , None) } else { (None , Some (self . direction . opposite ())) } ; if iterate_over . unwrap_or (Outgoing) == Outgoing { let i = self . next [0] . index () ; if let Some (Edge { node , weight : Some (weight) , next , }) = self . edges . get (i) { self . next [0] = next [0] ; return Some (EdgeReference { index : edge_index (i) , node : if reverse == Some (Outgoing) { swap_pair (* node) } else { * node } , weight , }) ; } } if iterate_over . unwrap_or (Incoming) == Incoming { while let Some (Edge { node , weight , next }) = self . edges . get (self . next [1] . index ()) { debug_assert ! (weight . is_some ()) ; let edge_index = self . next [1] ; self . next [1] = next [1] ; if iterate_over . is_none () && node [0] == self . skip_start { continue ; } return Some (EdgeReference { index : edge_index , node : if reverse == Some (Incoming) { swap_pair (* node) } else { * node } , weight : weight . as_ref () . unwrap () , }) ; } } None } }
    };
}

impl_827!();