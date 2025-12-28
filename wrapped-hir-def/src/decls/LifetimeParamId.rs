macro_rules! deps {
    () => {
        GenericDefId!();
    };
}

macro_rules! LifetimeParamId {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct LifetimeParamId { pub parent : GenericDefId , pub local_id : LocalLifetimeParamId , }
    };
}

LifetimeParamId!();