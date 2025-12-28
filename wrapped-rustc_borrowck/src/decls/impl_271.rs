macro_rules! deps {
    () => {
        PoloniusLocationTable!();
        FactCell!();
    };
}

macro_rules! impl_271 {
    () => {
        deps!();
        impl FactCell for PoloniusRegionVid { fn to_string (& self , _location_table : & PoloniusLocationTable) -> String { format ! ("{self:?}") } }
    };
}

impl_271!()