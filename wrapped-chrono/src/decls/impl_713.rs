macro_rules! deps {
    () => {
        WeekdaySet!();
    };
}

macro_rules! impl_713 {
    () => {
        deps!();
        # [cfg (feature = "defmt")] impl defmt :: Format for WeekdaySet { fn format (& self , f : defmt :: Formatter < '_ >) { defmt :: write ! (f , "WeekdaySet({}{}{}{}{}{}{})" , 0x1 & (self . 0 >> 6) , 0x1 & (self . 0 >> 5) , 0x1 & (self . 0 >> 4) , 0x1 & (self . 0 >> 3) , 0x1 & (self . 0 >> 2) , 0x1 & (self . 0 >> 1) , 0x1 & (self . 0 >> 0) ,) } }
    };
}

impl_713!()