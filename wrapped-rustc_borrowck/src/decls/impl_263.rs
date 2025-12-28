macro_rules! deps {
    () => {
        PoloniusLocationTable!();
        FactRow!();
    };
}

macro_rules! impl_263 {
    () => {
        deps!();
        impl FactRow for PoloniusRegionVid { fn write (& self , out : & mut dyn Write , location_table : & PoloniusLocationTable ,) -> Result < () , Box < dyn Error > > { write_row (out , location_table , & [self]) } }
    };
}

impl_263!();