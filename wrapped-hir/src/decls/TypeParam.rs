macro_rules! TypeParam {
    () => {
        # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash)] pub struct TypeParam { pub (crate) id : TypeParamId , }
    };
}

TypeParam!();