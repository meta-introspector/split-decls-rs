macro_rules! deps {
    () => {
        AssocItemId!();
        ModuleDefId!();
    };
}

macro_rules! impl_708 {
    () => {
        deps!();
        impl From < AssocItemId > for ModuleDefId { fn from (item : AssocItemId) -> Self { match item { AssocItemId :: FunctionId (f) => f . into () , AssocItemId :: ConstId (c) => c . into () , AssocItemId :: TypeAliasId (t) => t . into () , } } }
    };
}

impl_708!()