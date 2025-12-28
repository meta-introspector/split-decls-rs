macro_rules! deps {
    () => {
        Edge!();
        Node!();
        AttrStmt!();
        Graph!();
    };
}

macro_rules! impl_113 {
    () => {
        deps!();
        # [cfg (feature = "display")] impl < A > Display for AttrStmt < A > where A : Display , { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , std :: fmt :: Error > { match self { AttrStmt :: Graph (attr) => write ! (f , "graph [{}]" , attr) , AttrStmt :: Edge (attr) => write ! (f , "edge [{}]" , attr) , AttrStmt :: Node (attr) => write ! (f , "node [{}]" , attr) , } } }
    };
}

impl_113!()