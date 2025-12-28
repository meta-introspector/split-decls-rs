macro_rules! deps {
    () => {
        FactRow!();
        FactCell!();
        PoloniusLocationTable!();
    };
}

macro_rules! impl_265 {
    () => {
        deps!();
        impl < A , B , C > FactRow for (A , B , C) where A : FactCell , B : FactCell , C : FactCell , { fn write (& self , out : & mut dyn Write , location_table : & PoloniusLocationTable ,) -> Result < () , Box < dyn Error > > { write_row (out , location_table , & [& self . 0 , & self . 1 , & self . 2]) } }
    };
}

impl_265!();