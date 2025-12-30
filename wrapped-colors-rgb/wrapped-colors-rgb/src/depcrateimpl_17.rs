// Generated macro for impl_17 (impl)
macro_rules! Depcrateimpl_17 {
() => {
// Module: crate
// Provides: {"impl_17"}
// Dependencies: {}
impl App { # [doc = " Run the app"] # [doc = ""] # [doc = " This is the main event loop for the app."] pub fn run (mut self , terminal : & mut DefaultTerminal) -> Result < () > { while self . is_running () { terminal . draw (| frame | frame . render_widget (& mut self , frame . area ())) ? ; self . handle_events () ? ; } Ok (()) } const fn is_running (& self) -> bool { matches ! (self . state , AppState :: Running) } # [doc = " Handle any events that have occurred since the last time the app was rendered."] fn handle_events (& mut self) -> Result < () > { let timeout = Duration :: from_secs_f32 (1.0 / 60.0) ; if ! event :: poll (timeout) ? { return Ok (()) ; } if event :: read () ? . is_key_press () { self . state = AppState :: Quit ; } Ok (()) } }
};
}
