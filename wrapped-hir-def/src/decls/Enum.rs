macro_rules! deps {
    () => {
        RawVisibilityId!();
    };
}

macro_rules! Enum {
    () => {
        deps!();
        # [derive (Debug , Clone , Eq , PartialEq)] pub struct Enum { pub name : Name , pub (crate) visibility : RawVisibilityId , }
    };
}

Enum!()