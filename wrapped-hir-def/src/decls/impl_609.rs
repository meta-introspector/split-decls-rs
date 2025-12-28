macro_rules! deps {
    () => {
        AssocItemLoc!();
        DefDatabase!();
        ModuleId!();
        HasModule!();
    };
}

macro_rules! impl_609 {
    () => {
        deps!();
        impl < N : AstIdNode > HasModule for AssocItemLoc < N > { # [inline] fn module (& self , db : & dyn DefDatabase) -> ModuleId { self . container . module (db) } }
    };
}

impl_609!()