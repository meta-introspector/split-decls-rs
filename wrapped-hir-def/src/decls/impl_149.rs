macro_rules! deps {
    () => {
        AttrDefId!();
        AssocItemId!();
    };
}

macro_rules! impl_149 {
    () => {
        deps!();
        impl From < AssocItemId > for AttrDefId { fn from (assoc : AssocItemId) -> Self { match assoc { AssocItemId :: FunctionId (it) => AttrDefId :: FunctionId (it) , AssocItemId :: ConstId (it) => AttrDefId :: ConstId (it) , AssocItemId :: TypeAliasId (it) => AttrDefId :: TypeAliasId (it) , } } }
    };
}

impl_149!()