macro_rules! deps {
    () => {
        FactCell!();
        PoloniusLocationTable!();
    };
}

macro_rules! impl_269 {
    () => {
        deps!();
        impl FactCell for Local { fn to_string (& self , _location_table : & PoloniusLocationTable) -> String { format ! ("{self:?}") } }
    };
}

impl_269!();