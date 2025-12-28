macro_rules! deps {
    () => {
        DocAtom!();
        DocExpr!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl From < DocAtom > for DocExpr { fn from (atom : DocAtom) -> Self { DocExpr :: Atom (atom) } }
    };
}

impl_21!();