macro_rules! deps {
    () => {
        NaiveDate!();
    };
}

macro_rules! impl_362 {
    () => {
        deps!();
        # [cfg (feature = "defmt")] impl defmt :: Format for NaiveDate { fn format (& self , fmt : defmt :: Formatter) { let year = self . year () ; let mdf = self . mdf () ; if (0 ..= 9999) . contains (& year) { defmt :: write ! (fmt , "{:02}{:02}" , year / 100 , year % 100) ; } else { let sign = ['+' , '-'] [(year < 0) as usize] ; defmt :: write ! (fmt , "{}{:05}" , sign , year . abs ()) ; } defmt :: write ! (fmt , "-{:02}-{:02}" , mdf . month () , mdf . day ()) ; } }
    };
}

impl_362!();