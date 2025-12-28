macro_rules! deps {
    () => {
        FactRow!();
        PoloniusLocationTable!();
        FactCell!();
    };
}

macro_rules! impl_264 {
    () => {
        deps!();
        impl < A , B > FactRow for (A , B) where A : FactCell , B : FactCell , { fn write (& self , out : & mut dyn Write , location_table : & PoloniusLocationTable ,) -> Result < () , Box < dyn Error > > { write_row (out , location_table , & [& self . 0 , & self . 1]) } }
    };
}

impl_264!()