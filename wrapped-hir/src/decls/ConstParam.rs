macro_rules! ConstParam {
    () => {
        # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash)] pub struct ConstParam { pub (crate) id : ConstParamId , }
    };
}

ConstParam!()