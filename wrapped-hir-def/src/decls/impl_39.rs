macro_rules! deps {
    () => {
        ItemLoc!();
        ModuleId!();
        AstIdLoc!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl < N : AstIdNode > AstIdLoc for ItemLoc < N > { type Container = ModuleId ; type Ast = N ; # [inline] fn ast_id (& self) -> AstId < Self :: Ast > { self . id } # [inline] fn container (& self) -> Self :: Container { self . container } }
    };
}

impl_39!()