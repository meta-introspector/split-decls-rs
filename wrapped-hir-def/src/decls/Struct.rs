macro_rules! deps {
    () => {
        FieldsShape!();
        RawVisibilityId!();
    };
}

macro_rules! Struct {
    () => {
        deps!();
        # [derive (Debug , Clone , Eq , PartialEq)] pub struct Struct { pub name : Name , pub (crate) visibility : RawVisibilityId , pub shape : FieldsShape , }
    };
}

Struct!()