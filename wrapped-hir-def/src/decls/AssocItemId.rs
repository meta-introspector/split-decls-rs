macro_rules! AssocItemId {
    () => {
        # [derive (Debug , Copy , Clone , PartialEq , Eq , Hash , salsa_macros :: Supertype)] pub enum AssocItemId { FunctionId (FunctionId) , ConstId (ConstId) , TypeAliasId (TypeAliasId) , }
    };
}

AssocItemId!();