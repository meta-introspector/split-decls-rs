macro_rules! deps {
    () => {
        ConstParamId!();
        BindingId!();
    };
}

macro_rules! ValueNs {
    () => {
        deps!();
        # [derive (Debug , Copy , Clone , PartialEq , Eq , Hash)] pub enum ValueNs { ImplSelf (ImplId) , LocalBinding (BindingId) , FunctionId (FunctionId) , ConstId (ConstId) , StaticId (StaticId) , StructId (StructId) , EnumVariantId (EnumVariantId) , GenericParam (ConstParamId) , }
    };
}

ValueNs!();