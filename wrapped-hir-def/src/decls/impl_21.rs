macro_rules! deps {
    () => {
        DocExpr!();
        DocAtom!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl From < DocAtom > for DocExpr { fn from (atom : DocAtom) -> Self { DocExpr :: Atom (atom) } }
    };
}

impl_21!()