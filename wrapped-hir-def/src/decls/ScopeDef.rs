macro_rules! deps {
    () => {
        BindingId!();
        Label!();
        AdtId!();
        GenericParamId!();
        ModuleDefId!();
        LabelId!();
    };
}

macro_rules! ScopeDef {
    () => {
        deps!();
        # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub enum ScopeDef { ModuleDef (ModuleDefId) , Unknown , ImplSelfType (ImplId) , AdtSelfType (AdtId) , GenericParam (GenericParamId) , Local (BindingId) , Label (LabelId) , }
    };
}

ScopeDef!()