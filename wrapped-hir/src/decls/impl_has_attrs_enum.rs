macro_rules! deps {
    () => {
        HasAttrs!();
    };
}

macro_rules! impl_has_attrs_enum {
    () => {
        deps!();
        macro_rules ! impl_has_attrs_enum { ($ ($ variant : ident) ,* for $ enum : ident) => { $ (impl HasAttrs for $ variant { fn attrs (self , db : & dyn HirDatabase) -> AttrsWithOwner { $ enum ::$ variant (self) . attrs (db) } fn attr_id (self) -> AttrDefId { $ enum ::$ variant (self) . attr_id () } }) * } ; }
    };
}

impl_has_attrs_enum!();