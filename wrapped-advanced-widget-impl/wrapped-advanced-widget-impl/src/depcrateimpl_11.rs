// Generated macro for impl_11 (impl)
macro_rules! Depcrateimpl_11 {
() => {
// Module: crate
// Provides: {"impl_11"}
// Dependencies: {}
impl App { fn run (mut self , terminal : & mut DefaultTerminal) -> Result < () > { while ! self . should_quit { self . render (terminal) ? ; self . handle_events () ? ; } Ok (()) } fn render (& mut self , tui : & mut DefaultTerminal) -> Result < () > { tui . draw (| frame | frame . render_widget (self , frame . area ())) ? ; Ok (()) } fn handle_events (& mut self) -> Result < () > { let timeout = Duration :: from_secs_f64 (1.0 / 50.0) ; if ! event :: poll (timeout) ? { return Ok (()) ; } if event :: read () ? . is_key_press () { self . should_quit = true ; } Ok (()) } }
};
}
