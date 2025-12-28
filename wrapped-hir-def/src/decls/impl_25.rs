macro_rules! deps {
    () => {
        ItemLoc!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl < N : AstIdNode > Clone for ItemLoc < N > { fn clone (& self) -> Self { * self } }
    };
}

impl_25!()