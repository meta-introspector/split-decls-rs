macro_rules! deps {
    () => {
        ParseWeekdayError!();
    };
}

macro_rules! impl_706 {
    () => {
        deps!();
        # [cfg (feature = "defmt")] impl defmt :: Format for ParseWeekdayError { fn format (& self , fmt : defmt :: Formatter) { defmt :: write ! (fmt , "ParseWeekdayError {{ .. }}") } }
    };
}

impl_706!()