macro_rules! deps {
    () => {
        NodeSet!();
        EdgeStmt!();
        EdgeSet!();
    };
}

macro_rules! impl_109 {
    () => {
        deps!();
        impl < A , I > From < (I , & mut NodeSet < A >) > for EdgeSet < A > where I : IntoIterator < Item = EdgeStmt < A > > , A : Clone , { fn from (tuple : (I , & mut NodeSet < A >)) -> Self { let (stmts , nodes) = tuple ; let mut set = EdgeSet :: empty () ; for stmt in stmts { set += (stmt , & mut * nodes) . into () ; } set } }
    };
}

impl_109!();