macro_rules! deps {
    () => {
        TypeParamId!();
        ConstParamId!();
        LifetimeParamId!();
    };
}

macro_rules! GenericParamId {
    () => {
        deps!();
        # [doc = " A generic param"] # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash)] pub enum GenericParamId { TypeParamId (TypeParamId) , ConstParamId (ConstParamId) , LifetimeParamId (LifetimeParamId) , }
    };
}

GenericParamId!()