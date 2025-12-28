macro_rules! deps {
    () => {
        AssocItemLoc!();
    };
}

macro_rules! impl_604 {
    () => {
        deps!();
        impl < N : AstIdNode > Clone for AssocItemLoc < N > { fn clone (& self) -> Self { * self } }
    };
}

impl_604!();