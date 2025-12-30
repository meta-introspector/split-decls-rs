// Generated macro for handle_mouse_event (function)
macro_rules! Depcratehandle_mouse_event {
() => {
// Module: crate
// Provides: {"handle_mouse_event"}
// Dependencies: {}
fn handle_mouse_event (mouse : MouseEvent , button_states : & mut [State ; 3] , selected_button : & mut usize ,) { match mouse . kind { MouseEventKind :: Moved => { let old_selected_button = * selected_button ; * selected_button = match mouse . column { x if x < 15 => 0 , x if x < 30 => 1 , _ => 2 , } ; if old_selected_button != * selected_button { if button_states [old_selected_button] != State :: Active { button_states [old_selected_button] = State :: Normal ; } if button_states [* selected_button] != State :: Active { button_states [* selected_button] = State :: Selected ; } } } MouseEventKind :: Down (MouseButton :: Left) => { if button_states [* selected_button] == State :: Active { button_states [* selected_button] = State :: Normal ; } else { button_states [* selected_button] = State :: Active ; } } _ => () , } }
};
}
