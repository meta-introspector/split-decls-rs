macro_rules! deps {
    () => {
        GenericDefId!();
    };
}

macro_rules! TypeOrConstParamId {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash , PartialOrd , Ord)] pub struct TypeOrConstParamId { pub parent : GenericDefId , pub local_id : LocalTypeOrConstParamId , }
    };
}

TypeOrConstParamId!()