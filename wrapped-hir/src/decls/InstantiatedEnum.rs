macro_rules! deps {
    () => {
        Enum!();
    };
}

macro_rules! InstantiatedEnum {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct InstantiatedEnum < 'db > { pub (crate) inner : Enum , pub (crate) args : GenericArgs < 'db > , }
    };
}

InstantiatedEnum!()