macro_rules! deps {
    () => {
        Variant!();
    };
}

macro_rules! InstantiatedVariant {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct InstantiatedVariant < 'db > { pub (crate) inner : Variant , pub (crate) args : GenericArgs < 'db > , }
    };
}

InstantiatedVariant!();