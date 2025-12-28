macro_rules! deps {
    () => {
        CrateName!();
        CrateDisplayName!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl CrateDisplayName { pub fn from_canonical_name (canonical_name : & str) -> CrateDisplayName { let crate_name = CrateName :: normalize_dashes (canonical_name) ; CrateDisplayName { crate_name , canonical_name : Symbol :: intern (canonical_name) } } }
    };
}

impl_33!();