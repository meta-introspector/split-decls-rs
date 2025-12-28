macro_rules! deps {
    () => {
        FactCell!();
        PoloniusLocationTable!();
    };
}

macro_rules! impl_268 {
    () => {
        deps!();
        impl FactCell for BorrowIndex { fn to_string (& self , _location_table : & PoloniusLocationTable) -> String { format ! ("{self:?}") } }
    };
}

impl_268!();