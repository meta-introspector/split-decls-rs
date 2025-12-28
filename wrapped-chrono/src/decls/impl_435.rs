macro_rules! deps {
    () => {
        NaiveDateTime!();
    };
}

macro_rules! impl_435 {
    () => {
        deps!();
        # [cfg (feature = "defmt")] impl defmt :: Format for NaiveDateTime { fn format (& self , fmt : defmt :: Formatter) { defmt :: write ! (fmt , "{}T{}" , self . date , self . time) ; } }
    };
}

impl_435!();