macro_rules! deps {
    () => {
        AssocItemLoc!();
    };
}

macro_rules! impl_605 {
    () => {
        deps!();
        impl < N : AstIdNode > Copy for AssocItemLoc < N > { }
    };
}

impl_605!();