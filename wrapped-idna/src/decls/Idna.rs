macro_rules! deps {
    () => {
        Config!();
        Uts46!();
    };
}

macro_rules! Idna {
    () => {
        deps!();
        # [doc = " Deprecated. Use the crate-top-level functions or [`Uts46`]."] # [derive (Default)] # [deprecated] pub struct Idna { config : Config , }
    };
}

Idna!();