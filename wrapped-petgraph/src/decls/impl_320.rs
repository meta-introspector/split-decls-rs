macro_rules! deps {
    () => {
        IndexType!();
        List!();
        EdgeIndex!();
    };
}

macro_rules! impl_320 {
    () => {
        deps!();
        impl < E , Ix : IndexType > DataMap for List < E , Ix > { fn node_weight (& self , n : Self :: NodeId) -> Option < & () > { if n . index () < self . suc . len () { Some (& ()) } else { None } } # [doc = " Accesses the weight of edge `e`"] # [doc = ""] # [doc = " Computes in **O(1)**"] fn edge_weight (& self , e : EdgeIndex < Ix >) -> Option < & E > { self . get_edge (e) . map (| x | & x . weight) } }
    };
}

impl_320!();