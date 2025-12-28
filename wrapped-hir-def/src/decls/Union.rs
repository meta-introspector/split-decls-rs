macro_rules! deps {
    () => {
        RawVisibilityId!();
    };
}

macro_rules! Union {
    () => {
        deps!();
        # [derive (Debug , Clone , Eq , PartialEq)] pub struct Union { pub name : Name , pub (crate) visibility : RawVisibilityId , }
    };
}

Union!()