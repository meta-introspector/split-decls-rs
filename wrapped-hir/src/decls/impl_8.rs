macro_rules! deps {
    () => {
        Crate!();
        HasAttrs!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl HasAttrs for crate :: Crate { fn attrs (self , db : & dyn HirDatabase) -> AttrsWithOwner { let def = AttrDefId :: ModuleId (self . root_module () . id) ; AttrsWithOwner :: new (db , def) } fn attr_id (self) -> AttrDefId { AttrDefId :: ModuleId (self . root_module () . id) } }
    };
}

impl_8!();