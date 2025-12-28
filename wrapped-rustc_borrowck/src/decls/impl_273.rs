macro_rules! deps {
    () => {
        FactCell!();
        PoloniusLocationTable!();
    };
}

macro_rules! impl_273 {
    () => {
        deps!();
        impl FactCell for LocationIndex { fn to_string (& self , location_table : & PoloniusLocationTable) -> String { format ! ("{:?}" , location_table . to_rich_location (* self)) } }
    };
}

impl_273!();