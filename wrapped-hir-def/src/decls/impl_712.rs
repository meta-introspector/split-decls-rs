macro_rules! deps {
    () => {
        GenericDefId!();
        AssocItemId!();
    };
}

macro_rules! impl_712 {
    () => {
        deps!();
        impl From < AssocItemId > for GenericDefId { fn from (item : AssocItemId) -> Self { match item { AssocItemId :: FunctionId (f) => f . into () , AssocItemId :: ConstId (c) => c . into () , AssocItemId :: TypeAliasId (t) => t . into () , } } }
    };
}

impl_712!()