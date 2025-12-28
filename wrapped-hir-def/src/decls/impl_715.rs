macro_rules! deps {
    () => {
        CallableDefId!();
        ModuleDefId!();
        AdtId!();
    };
}

macro_rules! impl_715 {
    () => {
        deps!();
        impl From < CallableDefId > for ModuleDefId { fn from (def : CallableDefId) -> ModuleDefId { match def { CallableDefId :: FunctionId (f) => ModuleDefId :: FunctionId (f) , CallableDefId :: StructId (s) => ModuleDefId :: AdtId (AdtId :: StructId (s)) , CallableDefId :: EnumVariantId (e) => ModuleDefId :: EnumVariantId (e) , } } }
    };
}

impl_715!()