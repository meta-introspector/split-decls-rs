macro_rules! deps {
    () => {
        ItemLoc!();
    };
}

macro_rules! impl_600 {
    () => {
        deps!();
        impl < N : AstIdNode > Eq for ItemLoc < N > { }
    };
}

impl_600!();