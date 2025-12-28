macro_rules! VariantId {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash , salsa_macros :: Supertype)] pub enum VariantId { EnumVariantId (EnumVariantId) , StructId (StructId) , UnionId (UnionId) , }
    };
}

VariantId!()