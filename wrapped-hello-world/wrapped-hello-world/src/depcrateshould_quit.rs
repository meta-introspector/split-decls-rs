// Generated macro for should_quit (function)
macro_rules! Depcrateshould_quit {
() => {
// Module: crate
// Provides: {"should_quit"}
// Dependencies: {}
# [doc = " Check if the user has pressed 'q'. This is where you would handle events. This example just"] # [doc = " checks if the user has pressed 'q' and returns true if they have. It does not handle any other"] # [doc = " events. There is a 250ms timeout on the event poll to ensure that the terminal is rendered at"] # [doc = " least once every 250ms. This allows you to do other work in the application loop, such as"] # [doc = " updating the application state, without blocking the event loop for too long."] fn should_quit () -> Result < bool > { if event :: poll (Duration :: from_millis (250)) . context ("event poll failed") ? { let q_pressed = event :: read () . context ("event read failed") ? . as_key_press_event () . is_some_and (| key | key . code == KeyCode :: Char ('q')) ; return Ok (q_pressed) ; } Ok (false) }
};
}
