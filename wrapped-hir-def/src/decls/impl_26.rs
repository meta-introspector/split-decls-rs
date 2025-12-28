macro_rules! deps {
    () => {
        ItemLoc!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl < N : AstIdNode > Copy for ItemLoc < N > { }
    };
}

impl_26!()