// Generated macro for handle_key_event (function)
macro_rules! Depcratehandle_key_event {
() => {
// Module: crate
// Provides: {"handle_key_event"}
// Dependencies: {}
fn handle_key_event (key : KeyEvent , button_states : & mut [State ; 3] , selected_button : & mut usize ,) -> ControlFlow < () > { if ! key . is_press () { return ControlFlow :: Continue (()) ; } match key . code { KeyCode :: Char ('q') => return ControlFlow :: Break (()) , KeyCode :: Left | KeyCode :: Char ('h') => { button_states [* selected_button] = State :: Normal ; * selected_button = selected_button . saturating_sub (1) ; button_states [* selected_button] = State :: Selected ; } KeyCode :: Right | KeyCode :: Char ('l') => { button_states [* selected_button] = State :: Normal ; * selected_button = selected_button . saturating_add (1) . min (2) ; button_states [* selected_button] = State :: Selected ; } KeyCode :: Char (' ') => { if button_states [* selected_button] == State :: Active { button_states [* selected_button] = State :: Normal ; } else { button_states [* selected_button] = State :: Active ; } } _ => () , } ControlFlow :: Continue (()) }
};
}
