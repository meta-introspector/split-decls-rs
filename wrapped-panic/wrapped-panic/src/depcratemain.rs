// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () -> Result < () > { color_eyre :: install () ? ; let mut panic_hook_state = PanicHandlerState :: Enabled ; ratatui :: run (| terminal | { loop { terminal . draw (| frame | render (frame , & panic_hook_state)) ? ; if let Some (key) = event :: read () ? . as_key_press_event () { match key . code { KeyCode :: Char ('p') => panic ! ("intentional demo panic") , KeyCode :: Char ('e') => bail ! ("intentional demo error") , KeyCode :: Char ('h') => { let _ = std :: panic :: take_hook () ; panic_hook_state = PanicHandlerState :: Disabled ; } KeyCode :: Char ('q') => return Ok (()) , _ => { } } } } }) }
};
}
