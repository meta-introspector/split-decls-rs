macro_rules! deps {
    () => {
        Graph!();
        Node!();
        AttrStmt!();
        Edge!();
    };
}

macro_rules! impl_111 {
    () => {
        deps!();
        impl < A > AttrStmt < A > { fn filter_map < F , B > (self , f : F) -> Option < AttrStmt < B > > where F : Fn (A) -> Option < B > , { match self { AttrStmt :: Graph (a) => f (a) . map (AttrStmt :: Graph) , AttrStmt :: Node (a) => f (a) . map (AttrStmt :: Node) , AttrStmt :: Edge (a) => f (a) . map (AttrStmt :: Edge) , } } }
    };
}

impl_111!()