macro_rules! deps {
    () => {
        PoloniusLocationTable!();
    };
}

macro_rules! FactCell {
    () => {
        deps!();
        trait FactCell { fn to_string (& self , location_table : & PoloniusLocationTable) -> String ; }
    };
}

FactCell!()