macro_rules! deps {
    () => {
        CapacityError!();
    };
}

macro_rules! impl_392 {
    () => {
        deps!();
        impl defmt :: Format for CapacityError { fn format (& self , fmt : Formatter < '_ >) { defmt :: write ! (fmt , "CapacityError") ; } }
    };
}

impl_392!();