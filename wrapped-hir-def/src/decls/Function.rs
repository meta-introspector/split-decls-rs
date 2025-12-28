macro_rules! deps {
    () => {
        RawVisibilityId!();
    };
}

macro_rules! Function {
    () => {
        deps!();
        # [derive (Debug , Clone , Eq , PartialEq)] pub struct Function { pub name : Name , pub (crate) visibility : RawVisibilityId , }
    };
}

Function!();