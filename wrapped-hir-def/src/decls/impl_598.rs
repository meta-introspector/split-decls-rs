macro_rules! deps {
    () => {
        ItemLoc!();
    };
}

macro_rules! impl_598 {
    () => {
        deps!();
        impl < N : AstIdNode > Copy for ItemLoc < N > { }
    };
}

impl_598!()