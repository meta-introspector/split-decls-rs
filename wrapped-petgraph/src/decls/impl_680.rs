macro_rules! deps {
    () => {
        Undirected!();
        Directed!();
        IndexType!();
        EdgeType!();
        NoPretty!();
        DebugMap!();
        Graph!();
    };
}

macro_rules! impl_680 {
    () => {
        deps!();
        impl < N , E , Ty , Ix > fmt :: Debug for Graph < N , E , Ty , Ix > where N : fmt :: Debug , E : fmt :: Debug , Ty : EdgeType , Ix : IndexType , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let etype = if self . is_directed () { "Directed" } else { "Undirected" } ; let mut fmt_struct = f . debug_struct ("Graph") ; fmt_struct . field ("Ty" , & etype) ; fmt_struct . field ("node_count" , & self . node_count ()) ; fmt_struct . field ("edge_count" , & self . edge_count ()) ; if self . edge_count () > 0 { fmt_struct . field ("edges" , & self . edges . iter () . map (| e | NoPretty ((e . source () . index () , e . target () . index ()))) . format (", ") ,) ; } if size_of :: < N > () != 0 { fmt_struct . field ("node weights" , & DebugMap (| | self . nodes . iter () . map (| n | & n . weight) . enumerate ()) ,) ; } if size_of :: < E > () != 0 { fmt_struct . field ("edge weights" , & DebugMap (| | self . edges . iter () . map (| n | & n . weight) . enumerate ()) ,) ; } fmt_struct . finish () } }
    };
}

impl_680!();