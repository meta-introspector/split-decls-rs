macro_rules! deps {
    () => {
        ParseMonthError!();
    };
}

macro_rules! impl_734 {
    () => {
        deps!();
        # [cfg (feature = "defmt")] impl defmt :: Format for ParseMonthError { fn format (& self , fmt : defmt :: Formatter) { defmt :: write ! (fmt , "ParseMonthError {{ .. }}") } }
    };
}

impl_734!();