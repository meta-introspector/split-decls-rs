// Generated macro for run (function)
macro_rules! Depcraterun {
() => {
// Module: crate
// Provides: {"run"}
// Dependencies: {}
# [doc = " Run the application loop. This is where you would handle events and update the application"] # [doc = " state. This example exits when the user presses 'q'. Other styles of application loops are"] # [doc = " possible, for example, you could have multiple application states and switch between them based"] # [doc = " on events, or you could have a single application state and update it based on events."] fn run (terminal : & mut DefaultTerminal) -> Result < () > { loop { terminal . draw (render) ? ; if should_quit () ? { break ; } } Ok (()) }
};
}
