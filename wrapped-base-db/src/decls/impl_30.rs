macro_rules! deps {
    () => {
        CrateDisplayName!();
        CrateName!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl From < CrateName > for CrateDisplayName { fn from (crate_name : CrateName) -> CrateDisplayName { let canonical_name = crate_name . 0 . clone () ; CrateDisplayName { crate_name , canonical_name } } }
    };
}

impl_30!();