// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () -> Result < () > { color_eyre :: install () ? ; let mut show_popup = false ; ratatui :: run (| terminal | { loop { terminal . draw (| frame | render (frame , show_popup)) ? ; if let Some (key) = event :: read () ? . as_key_press_event () { match key . code { KeyCode :: Char ('q') => return Ok (()) , KeyCode :: Char ('p') => show_popup = ! show_popup , _ => { } } } } }) }
};
}
