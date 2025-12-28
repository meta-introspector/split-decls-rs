macro_rules! TyDefId {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub enum TyDefId { BuiltinType (BuiltinType) , AdtId (AdtId) , TypeAliasId (TypeAliasId) , }
    };
}

TyDefId!();