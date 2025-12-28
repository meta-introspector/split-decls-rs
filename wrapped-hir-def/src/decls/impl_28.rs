macro_rules! deps {
    () => {
        ItemLoc!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl < N : AstIdNode > Eq for ItemLoc < N > { }
    };
}

impl_28!()