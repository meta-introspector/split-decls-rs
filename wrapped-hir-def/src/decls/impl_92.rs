macro_rules! deps {
    () => {
        HasModule!();
        ModuleId!();
        CrateRootModuleId!();
    };
}

macro_rules! impl_92 {
    () => {
        deps!();
        impl HasModule for CrateRootModuleId { # [inline] fn module (& self , _db : & dyn DefDatabase) -> ModuleId { ModuleId { krate : self . krate , block : None , local_id : DefMap :: ROOT } } # [inline] fn krate (& self , _db : & dyn DefDatabase) -> Crate { self . krate } }
    };
}

impl_92!()