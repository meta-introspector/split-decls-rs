macro_rules! deps {
    () => {
        PoloniusLocationTable!();
        FactCell!();
    };
}

macro_rules! impl_270 {
    () => {
        deps!();
        impl FactCell for MovePathIndex { fn to_string (& self , _location_table : & PoloniusLocationTable) -> String { format ! ("{self:?}") } }
    };
}

impl_270!();