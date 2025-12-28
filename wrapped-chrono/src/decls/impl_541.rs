macro_rules! deps {
    () => {
        FixedOffset!();
    };
}

macro_rules! impl_541 {
    () => {
        deps!();
        # [cfg (feature = "defmt")] impl defmt :: Format for FixedOffset { fn format (& self , f : defmt :: Formatter) { let offset = self . local_minus_utc ; let (sign , offset) = if offset < 0 { ('-' , - offset) } else { ('+' , offset) } ; let sec = offset . rem_euclid (60) ; let mins = offset . div_euclid (60) ; let min = mins . rem_euclid (60) ; let hour = mins . div_euclid (60) ; if sec == 0 { defmt :: write ! (f , "{}{:02}:{:02}" , sign , hour , min) } else { defmt :: write ! (f , "{}{:02}:{:02}:{:02}" , sign , hour , min , sec) } } }
    };
}

impl_541!();