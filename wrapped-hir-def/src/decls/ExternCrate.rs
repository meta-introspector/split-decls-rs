macro_rules! deps {
    () => {
        RawVisibilityId!();
        ImportAlias!();
    };
}

macro_rules! ExternCrate {
    () => {
        deps!();
        # [derive (Debug , Clone , Eq , PartialEq)] pub struct ExternCrate { pub name : Name , pub alias : Option < ImportAlias > , pub (crate) visibility : RawVisibilityId , }
    };
}

ExternCrate!();