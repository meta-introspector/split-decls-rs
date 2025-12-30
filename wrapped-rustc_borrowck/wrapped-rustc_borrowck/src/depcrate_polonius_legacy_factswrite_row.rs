// Generated macro for write_row (function)
macro_rules! Depcrate_polonius_legacy_factswrite_row {
() => {
// Module: crate::polonius::legacy::facts
// Provides: {"write_row"}
// Dependencies: {}
fn write_row (out : & mut dyn Write , location_table : & PoloniusLocationTable , columns : & [& dyn FactCell] ,) -> Result < () , Box < dyn Error > > { for (index , c) in columns . iter () . enumerate () { let tail = if index == columns . len () - 1 { "\n" } else { "\t" } ; write ! (out , "{:?}{tail}" , c . to_string (location_table)) ? ; } Ok (()) }
};
}
