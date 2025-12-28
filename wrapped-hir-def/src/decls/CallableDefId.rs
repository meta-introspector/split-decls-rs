macro_rules! CallableDefId {
    () => {
        # [derive (Debug , PartialOrd , Ord , Clone , Copy , PartialEq , Eq , Hash , salsa_macros :: Supertype)] pub enum CallableDefId { FunctionId (FunctionId) , StructId (StructId) , EnumVariantId (EnumVariantId) , }
    };
}

CallableDefId!()