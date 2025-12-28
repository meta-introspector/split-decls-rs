macro_rules! deps {
    () => {
        Label!();
        GenericParamId!();
        AdtId!();
        LabelId!();
        ModuleDefId!();
        BindingId!();
    };
}

macro_rules! ScopeDef {
    () => {
        deps!();
        # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub enum ScopeDef { ModuleDef (ModuleDefId) , Unknown , ImplSelfType (ImplId) , AdtSelfType (AdtId) , GenericParam (GenericParamId) , Local (BindingId) , Label (LabelId) , }
    };
}

ScopeDef!();