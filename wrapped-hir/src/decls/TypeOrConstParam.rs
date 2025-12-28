macro_rules! TypeOrConstParam {
    () => {
        # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash)] pub struct TypeOrConstParam { pub (crate) id : TypeOrConstParamId , }
    };
}

TypeOrConstParam!();