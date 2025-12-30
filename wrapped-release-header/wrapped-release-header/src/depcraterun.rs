// Generated macro for run (function)
macro_rules! Depcraterun {
() => {
// Module: crate
// Provides: {"run"}
// Dependencies: {}
fn run (mut terminal : DefaultTerminal) -> color_eyre :: Result < () > { loop { terminal . draw (render) ? ; if event :: read () ? . is_key_press () { break Ok (()) ; } } }
};
}
