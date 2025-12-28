macro_rules! deps {
    () => {
        TypeOrConstParamId!();
    };
}

macro_rules! ConstParamId {
    () => {
        deps!();
        # [doc = " A TypeOrConstParamId with an invariant that it actually belongs to a const"] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct ConstParamId (TypeOrConstParamId) ;
    };
}

ConstParamId!()