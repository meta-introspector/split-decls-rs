macro_rules! deps {
    () => {
        Edge!();
        Node!();
        Graph!();
        AList!();
        AttrStmt!();
    };
}

macro_rules! impl_112 {
    () => {
        deps!();
        impl < A > From < AstAttrStmt < A > > for Vec < AttrStmt < A > > { fn from (val : AstAttrStmt < A >) -> Self { match val { AstAttrStmt :: Graph (list) => { let alist : AList < A > = list . into () ; alist . into_iter () . map (| attr | AttrStmt :: Graph (attr)) . collect () } AstAttrStmt :: Node (list) => { let alist : AList < A > = list . into () ; alist . into_iter () . map (| attr | AttrStmt :: Node (attr)) . collect () } AstAttrStmt :: Edge (list) => { let alist : AList < A > = list . into () ; alist . into_iter () . map (| attr | AttrStmt :: Edge (attr)) . collect () } } } }
    };
}

impl_112!();