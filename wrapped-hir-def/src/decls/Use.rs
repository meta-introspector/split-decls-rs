macro_rules! deps {
    () => {
        RawVisibilityId!();
        UseTree!();
    };
}

macro_rules! Use {
    () => {
        deps!();
        # [derive (Debug , Clone , Eq , PartialEq)] pub struct Use { pub (crate) visibility : RawVisibilityId , pub (crate) use_tree : UseTree , }
    };
}

Use!()