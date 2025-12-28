macro_rules! deps {
    () => {
        Renames!();
        Clone!();
    };
}

macro_rules! rename {
    () => {
        deps!();
        # [doc = ""] pub mod rename { # [doc = " Determine how to do rename tracking."] # [derive (Debug , Copy , Clone , Eq , PartialEq)] pub enum Tracking { # [doc = " Do not track renames at all, the fastest option."] Disabled , # [doc = " Track renames."] Renames , # [doc = " Track renames and copies."] # [doc = ""] # [doc = " This is the most expensive option."] RenamesAndCopies , } }
    };
}

rename!()