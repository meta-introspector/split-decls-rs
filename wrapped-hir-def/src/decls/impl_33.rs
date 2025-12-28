macro_rules! deps {
    () => {
        AssocItemLoc!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl < N : AstIdNode > Copy for AssocItemLoc < N > { }
    };
}

impl_33!()