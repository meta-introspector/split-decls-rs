macro_rules! deps {
    () => {
        TypeOrConstParamId!();
    };
}

macro_rules! TypeParamId {
    () => {
        deps!();
        # [doc = " A TypeOrConstParamId with an invariant that it actually belongs to a type"] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct TypeParamId (TypeOrConstParamId) ;
    };
}

TypeParamId!();