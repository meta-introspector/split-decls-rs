macro_rules! deps {
    () => {
        HasAttrs!();
    };
}

macro_rules! impl_has_attrs {
    () => {
        deps!();
        macro_rules ! impl_has_attrs { ($ (($ def : ident , $ def_id : ident) ,) *) => { $ (impl HasAttrs for $ def { fn attrs (self , db : & dyn HirDatabase) -> AttrsWithOwner { let def = AttrDefId ::$ def_id (self . into ()) ; AttrsWithOwner :: new (db , def) } fn attr_id (self) -> AttrDefId { AttrDefId ::$ def_id (self . into ()) } }) * } ; }
    };
}

impl_has_attrs!()