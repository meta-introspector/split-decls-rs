macro_rules! deps {
    () => {
        RawVisibilityId!();
        ModKind!();
    };
}

macro_rules! Mod {
    () => {
        deps!();
        # [derive (Debug , Clone , Eq , PartialEq)] pub struct Mod { pub name : Name , pub (crate) visibility : RawVisibilityId , pub (crate) kind : ModKind , }
    };
}

Mod!()