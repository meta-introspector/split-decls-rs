macro_rules! deps {
    () => {
        VariantId!();
    };
}

macro_rules! DefWithBodyId {
    () => {
        deps!();
        # [doc = " The defs which have a body (have root expressions for type inference)."] # [derive (Debug , PartialOrd , Ord , Clone , Copy , PartialEq , Eq , Hash , salsa_macros :: Supertype)] pub enum DefWithBodyId { FunctionId (FunctionId) , StaticId (StaticId) , ConstId (ConstId) , VariantId (EnumVariantId) , }
    };
}

DefWithBodyId!()