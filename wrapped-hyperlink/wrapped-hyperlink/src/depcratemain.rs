// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () -> Result < () > { color_eyre :: install () ? ; let text = Line :: from (vec ! ["Example " . into () , "hyperlink" . blue ()]) ; let hyperlink = Hyperlink :: new (text , "https://example.com") ; ratatui :: run (| terminal | { loop { terminal . draw (| frame | frame . render_widget (& hyperlink , frame . area ())) ? ; if event :: read () ? . is_key_press () { break Ok (()) ; } } }) }
};
}
