macro_rules! deps {
    () => {
        AssocItemId!();
        AttrDefId!();
    };
}

macro_rules! impl_721 {
    () => {
        deps!();
        impl From < AssocItemId > for AttrDefId { fn from (assoc : AssocItemId) -> Self { match assoc { AssocItemId :: FunctionId (it) => AttrDefId :: FunctionId (it) , AssocItemId :: ConstId (it) => AttrDefId :: ConstId (it) , AssocItemId :: TypeAliasId (it) => AttrDefId :: TypeAliasId (it) , } } }
    };
}

impl_721!()