// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () -> Result < () , Box < dyn std :: error :: Error > > { ratatui :: run (| terminal | { loop { terminal . draw (| frame | frame . render_widget ("Hello World!" , frame . area ())) ? ; if crossterm :: event :: read () ? . is_key_press () { break Ok (()) ; } } }) }
};
}
