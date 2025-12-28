macro_rules! deps {
    () => {
        ItemContainerId!();
        AstIdLoc!();
        AssocItemLoc!();
    };
}

macro_rules! impl_612 {
    () => {
        deps!();
        impl < N : AstIdNode > AstIdLoc for AssocItemLoc < N > { type Container = ItemContainerId ; type Ast = N ; # [inline] fn ast_id (& self) -> AstId < Self :: Ast > { self . id } # [inline] fn container (& self) -> Self :: Container { self . container } }
    };
}

impl_612!()