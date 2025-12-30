// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () -> Result < () > { color_eyre :: install () ? ; match ratatui :: run (| terminal | App :: default () . run (terminal)) { Ok (Some (form)) => println ! ("{}" , serde_json :: to_string_pretty (& form) ?) , Ok (None) => println ! ("Canceled") , Err (err) => eprintln ! ("{err}") , } Ok (()) }
};
}
