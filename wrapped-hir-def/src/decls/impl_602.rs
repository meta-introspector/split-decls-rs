macro_rules! deps {
    () => {
        ModuleId!();
        DefDatabase!();
        ItemLoc!();
        HasModule!();
    };
}

macro_rules! impl_602 {
    () => {
        deps!();
        impl < N : AstIdNode > HasModule for ItemLoc < N > { # [inline] fn module (& self , _db : & dyn DefDatabase) -> ModuleId { self . container } }
    };
}

impl_602!()