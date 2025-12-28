macro_rules! deps {
    () => {
        OutOfRange!();
    };
}

macro_rules! impl_748 {
    () => {
        deps!();
        # [cfg (feature = "defmt")] impl defmt :: Format for OutOfRange { fn format (& self , fmt : defmt :: Formatter) { defmt :: write ! (fmt , "out of range") ; } }
    };
}

impl_748!();