macro_rules! deps {
    () => {
        HasModule!();
        ItemLoc!();
        ModuleId!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl < N : AstIdNode > HasModule for ItemLoc < N > { # [inline] fn module (& self , _db : & dyn DefDatabase) -> ModuleId { self . container } }
    };
}

impl_30!()