macro_rules! deps {
    () => {
        FactCell!();
        PoloniusLocationTable!();
    };
}

macro_rules! impl_272 {
    () => {
        deps!();
        impl FactCell for RegionVid { fn to_string (& self , _location_table : & PoloniusLocationTable) -> String { format ! ("{self:?}") } }
    };
}

impl_272!();