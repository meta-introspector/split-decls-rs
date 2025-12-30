// Generated macro for run (function)
macro_rules! Depcraterun {
() => {
// Module: crate
// Provides: {"run"}
// Dependencies: {}
fn run (mut terminal : DefaultTerminal) -> Result < () > { let mut selected_button : usize = 0 ; let mut button_states = [State :: Selected , State :: Normal , State :: Normal] ; loop { terminal . draw (| frame | render (frame , button_states)) ? ; if ! event :: poll (Duration :: from_millis (100)) ? { continue ; } match event :: read () ? { Event :: Key (key) => { if handle_key_event (key , & mut button_states , & mut selected_button) . is_break () { break ; } } Event :: Mouse (mouse) => { handle_mouse_event (mouse , & mut button_states , & mut selected_button) ; } _ => () , } } Ok (()) }
};
}
