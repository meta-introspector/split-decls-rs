macro_rules! deps {
    () => {
        NaiveTime!();
    };
}

macro_rules! impl_520 {
    () => {
        deps!();
        # [cfg (feature = "defmt")] impl defmt :: Format for NaiveTime { fn format (& self , fmt : defmt :: Formatter) { let (hour , min , sec) = self . hms () ; let (sec , nano) = if self . frac >= 1_000_000_000 { (sec + 1 , self . frac - 1_000_000_000) } else { (sec , self . frac) } ; let (hour , min , sec) = (hour as u8 , min as u8 , sec as u8) ; defmt :: write ! (fmt , "{:02}:{:02}:{:02}" , hour , min , sec) ; if nano == 0 { return ; } else if nano % 1_000_000 == 0 { defmt :: write ! (fmt , ".{:03}" , nano / 1_000_000) ; } else if nano % 1_000 == 0 { defmt :: write ! (fmt , ".{:06}" , nano / 1_000) ; } else { defmt :: write ! (fmt , ".{:09}" , nano) ; } } }
    };
}

impl_520!()