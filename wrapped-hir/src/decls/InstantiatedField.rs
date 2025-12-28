macro_rules! deps {
    () => {
        Field!();
    };
}

macro_rules! InstantiatedField {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct InstantiatedField < 'db > { pub (crate) inner : Field , pub (crate) args : GenericArgs < 'db > , }
    };
}

InstantiatedField!()