macro_rules! AdtId {
    () => {
        # [doc = " A Data Type"] # [derive (Debug , PartialOrd , Ord , Clone , Copy , PartialEq , Eq , Hash , salsa_macros :: Supertype)] pub enum AdtId { StructId (StructId) , UnionId (UnionId) , EnumId (EnumId) , }
    };
}

AdtId!()