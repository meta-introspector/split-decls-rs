macro_rules! deps {
    () => {
        Entry128!();
        Entry!();
    };
}

macro_rules! impl_146 {
    () => {
        deps!();
        impl From < Entry > for Entry128 { fn from (entry : Entry) -> Entry128 { Entry128 (entry , [0u8 ; 64]) } }
    };
}

impl_146!()