macro_rules! deps {
    () => {
        Struct!();
    };
}

macro_rules! InstantiatedStruct {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct InstantiatedStruct < 'db > { pub (crate) inner : Struct , pub (crate) args : GenericArgs < 'db > , }
    };
}

InstantiatedStruct!();