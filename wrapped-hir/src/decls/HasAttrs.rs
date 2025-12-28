macro_rules! HasAttrs {
    () => {
        pub trait HasAttrs { fn attrs (self , db : & dyn HirDatabase) -> AttrsWithOwner ; # [doc (hidden)] fn attr_id (self) -> AttrDefId ; }
    };
}

HasAttrs!();