macro_rules! deps {
    () => {
        ItemLoc!();
    };
}

macro_rules! impl_597 {
    () => {
        deps!();
        impl < N : AstIdNode > Clone for ItemLoc < N > { fn clone (& self) -> Self { * self } }
    };
}

impl_597!()