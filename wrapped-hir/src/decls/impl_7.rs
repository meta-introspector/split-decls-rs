macro_rules! deps {
    () => {
        Const!();
        Function!();
        HasAttrs!();
        AssocItem!();
        TypeAlias!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl HasAttrs for AssocItem { fn attrs (self , db : & dyn HirDatabase) -> AttrsWithOwner { match self { AssocItem :: Function (it) => it . attrs (db) , AssocItem :: Const (it) => it . attrs (db) , AssocItem :: TypeAlias (it) => it . attrs (db) , } } fn attr_id (self) -> AttrDefId { match self { AssocItem :: Function (it) => it . attr_id () , AssocItem :: Const (it) => it . attr_id () , AssocItem :: TypeAlias (it) => it . attr_id () , } } }
    };
}

impl_7!();