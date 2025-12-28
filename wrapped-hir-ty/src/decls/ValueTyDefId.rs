macro_rules! ValueTyDefId {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash , salsa_macros :: Supertype)] pub enum ValueTyDefId { FunctionId (FunctionId) , StructId (StructId) , UnionId (UnionId) , EnumVariantId (EnumVariantId) , ConstId (ConstId) , StaticId (StaticId) , }
    };
}

ValueTyDefId!()