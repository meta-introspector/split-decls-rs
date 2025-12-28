macro_rules! deps {
    () => {
        RawVisibilityId!();
    };
}

macro_rules! TypeAlias {
    () => {
        deps!();
        # [derive (Debug , Clone , PartialEq , Eq)] pub struct TypeAlias { pub name : Name , pub (crate) visibility : RawVisibilityId , }
    };
}

TypeAlias!()