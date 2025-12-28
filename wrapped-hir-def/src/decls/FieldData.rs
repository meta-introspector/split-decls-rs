macro_rules! deps {
    () => {
        RawVisibility!();
    };
}

macro_rules! FieldData {
    () => {
        deps!();
        # [doc = " A single field of an enum variant or struct"] # [derive (Debug , Clone , PartialEq , Eq)] pub struct FieldData { pub name : Name , pub type_ref : TypeRefId , pub visibility : RawVisibility , pub is_unsafe : bool , }
    };
}

FieldData!();