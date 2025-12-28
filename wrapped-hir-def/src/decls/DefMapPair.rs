macro_rules! deps {
    () => {
        DefMap!();
        LocalDefMap!();
    };
}

macro_rules! DefMapPair {
    () => {
        deps!();
        # [salsa_macros :: tracked] pub (crate) struct DefMapPair < 'db > { # [tracked] # [returns (ref)] pub (crate) def_map : DefMap , # [returns (ref)] pub (crate) local : LocalDefMap , }
    };
}

DefMapPair!()