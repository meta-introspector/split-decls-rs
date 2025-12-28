macro_rules! deps {
    () => {
        AttrStmt!();
        Graph!();
        Node!();
        Edge!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl < A > AttrStmt < A > { pub (crate) fn filter_map_attr < B > (self , f : & dyn Fn (A) -> Option < B >) -> AttrStmt < B > { match self { AttrStmt :: Graph (attr) => AttrStmt :: Graph (attr . filter_map_attr (f)) , AttrStmt :: Node (attr) => AttrStmt :: Node (attr . filter_map_attr (f)) , AttrStmt :: Edge (attr) => AttrStmt :: Edge (attr . filter_map_attr (f)) , } } }
    };
}

impl_37!();