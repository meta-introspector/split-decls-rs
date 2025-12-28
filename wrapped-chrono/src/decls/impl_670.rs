macro_rules! deps {
    () => {
        Utc!();
    };
}

macro_rules! impl_670 {
    () => {
        deps!();
        # [cfg (feature = "defmt")] impl defmt :: Format for Utc { fn format (& self , fmt : defmt :: Formatter) { defmt :: write ! (fmt , "Z") ; } }
    };
}

impl_670!();