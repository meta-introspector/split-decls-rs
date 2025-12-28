macro_rules! deps {
    () => {
        PoloniusLocationTable!();
    };
}

macro_rules! FactRow {
    () => {
        deps!();
        trait FactRow { fn write (& self , out : & mut dyn Write , location_table : & PoloniusLocationTable ,) -> Result < () , Box < dyn Error > > ; }
    };
}

FactRow!()