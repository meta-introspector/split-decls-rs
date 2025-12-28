macro_rules! deps {
    () => {
        IsoWeek!();
    };
}

macro_rules! impl_480 {
    () => {
        deps!();
        # [cfg (feature = "defmt")] impl defmt :: Format for IsoWeek { fn format (& self , fmt : defmt :: Formatter) { let year = self . year () ; let week = self . week () ; if (0 ..= 9999) . contains (& year) { defmt :: write ! (fmt , "{:04}-W{:02}" , year , week) } else { let sign = ['+' , '-'] [(year < 0) as usize] ; defmt :: write ! (fmt , "{}{:05}-W{:02}" , sign , year . abs () , week) } } }
    };
}

impl_480!();