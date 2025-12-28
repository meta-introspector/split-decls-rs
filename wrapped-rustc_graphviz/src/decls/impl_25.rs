macro_rules! deps {
    () => {
        Labeller!();
        Node!();
        Edge!();
        Style!();
        LabelledGraph!();
        Id!();
        LabelText!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl < 'a > Labeller < 'a > for LabelledGraph { type Node = Node ; type Edge = & 'a Edge ; fn graph_id (& 'a self) -> Id < 'a > { Id :: new (self . name) . unwrap () } fn node_id (& 'a self , n : & Node) -> Id < 'a > { id_name (n) } fn node_label (& 'a self , n : & Node) -> LabelText < 'a > { match self . node_labels [* n] { Some (l) => LabelStr (l . into ()) , None => LabelStr (id_name (n) . name) , } } fn edge_label (& 'a self , e : & & 'a Edge) -> LabelText < 'a > { LabelStr (e . label . into ()) } fn node_style (& 'a self , n : & Node) -> Style { self . node_styles [* n] } fn edge_style (& 'a self , e : & & 'a Edge) -> Style { e . style } }
    };
}

impl_25!()