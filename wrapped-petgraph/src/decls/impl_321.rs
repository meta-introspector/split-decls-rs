macro_rules! deps {
    () => {
        List!();
        EdgeIndex!();
        IndexType!();
    };
}

macro_rules! impl_321 {
    () => {
        deps!();
        impl < E , Ix : IndexType > DataMapMut for List < E , Ix > { fn node_weight_mut (& mut self , n : Self :: NodeId) -> Option < & mut () > { if n . index () < self . suc . len () { let b = Box :: new (()) ; Some (Box :: leak (b)) } else { None } } # [doc = " Accesses the weight of edge `e`"] # [doc = ""] # [doc = " Computes in **O(1)**"] fn edge_weight_mut (& mut self , e : EdgeIndex < Ix >) -> Option < & mut E > { self . get_edge_mut (e) . map (| x | & mut x . weight) } }
    };
}

impl_321!()