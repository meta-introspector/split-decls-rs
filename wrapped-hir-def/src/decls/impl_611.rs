macro_rules! deps {
    () => {
        ModuleId!();
        AstIdLoc!();
        ItemLoc!();
    };
}

macro_rules! impl_611 {
    () => {
        deps!();
        impl < N : AstIdNode > AstIdLoc for ItemLoc < N > { type Container = ModuleId ; type Ast = N ; # [inline] fn ast_id (& self) -> AstId < Self :: Ast > { self . id } # [inline] fn container (& self) -> Self :: Container { self . container } }
    };
}

impl_611!();