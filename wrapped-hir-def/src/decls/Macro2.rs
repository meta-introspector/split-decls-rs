macro_rules! deps {
    () => {
        RawVisibilityId!();
    };
}

macro_rules! Macro2 {
    () => {
        deps!();
        # [doc = " \"Macros 2.0\" macro definition."] # [derive (Debug , Clone , Eq , PartialEq)] pub struct Macro2 { pub name : Name , pub (crate) visibility : RawVisibilityId , }
    };
}

Macro2!()