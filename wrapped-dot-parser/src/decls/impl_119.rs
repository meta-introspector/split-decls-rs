macro_rules! deps {
    () => {
        Graph!();
    };
}

macro_rules! impl_119 {
    () => {
        deps!();
        impl < A > From < crate :: ast :: Graph < A > > for Graph < A > where A : Clone , { fn from (g : crate :: ast :: Graph < A >) -> Self { Graph { strict : g . strict , is_digraph : g . is_digraph , name : g . name , stmts : g . stmts . into () , } } }
    };
}

impl_119!();