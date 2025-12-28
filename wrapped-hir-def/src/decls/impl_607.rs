macro_rules! deps {
    () => {
        AssocItemLoc!();
    };
}

macro_rules! impl_607 {
    () => {
        deps!();
        impl < N : AstIdNode > Eq for AssocItemLoc < N > { }
    };
}

impl_607!()