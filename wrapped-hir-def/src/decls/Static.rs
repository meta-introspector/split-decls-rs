macro_rules! deps {
    () => {
        RawVisibilityId!();
    };
}

macro_rules! Static {
    () => {
        deps!();
        # [derive (Debug , Clone , Eq , PartialEq)] pub struct Static { pub name : Name , pub (crate) visibility : RawVisibilityId , }
    };
}

Static!();