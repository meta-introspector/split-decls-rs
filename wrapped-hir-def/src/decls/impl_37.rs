macro_rules! deps {
    () => {
        HasModule!();
        AssocItemLoc!();
        ModuleId!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl < N : AstIdNode > HasModule for AssocItemLoc < N > { # [inline] fn module (& self , db : & dyn DefDatabase) -> ModuleId { self . container . module (db) } }
    };
}

impl_37!()