macro_rules! deps {
    () => {
        PoloniusLocationTable!();
        FactCell!();
    };
}

macro_rules! write_row {
    () => {
        deps!();
        fn write_row (out : & mut dyn Write , location_table : & PoloniusLocationTable , columns : & [& dyn FactCell] ,) -> Result < () , Box < dyn Error > > { for (index , c) in columns . iter () . enumerate () { let tail = if index == columns . len () - 1 { "\n" } else { "\t" } ; write ! (out , "{:?}{tail}" , c . to_string (location_table)) ? ; } Ok (()) }
    };
}

write_row!();