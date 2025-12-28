macro_rules! deps {
    () => {
        CrateName!();
    };
}

macro_rules! Dependency {
    () => {
        deps!();
        # [derive (Debug , Clone , PartialEq , Eq , Hash)] pub struct Dependency < Id > { pub crate_id : Id , pub name : CrateName , prelude : bool , sysroot : bool , }
    };
}

Dependency!();