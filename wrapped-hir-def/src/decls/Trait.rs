macro_rules! deps {
    () => {
        RawVisibilityId!();
    };
}

macro_rules! Trait {
    () => {
        deps!();
        # [derive (Debug , Clone , Eq , PartialEq)] pub struct Trait { pub name : Name , pub (crate) visibility : RawVisibilityId , }
    };
}

Trait!();