// Generated macro for impl_19 (impl)
macro_rules! Depcrateimpl_19 {
() => {
// Module: crate
// Provides: {"impl_19"}
// Dependencies: {}
impl App { fn run (mut self , terminal : & mut DefaultTerminal) -> Result < () > { while self . state != AppState :: Quitting { terminal . draw (| frame | frame . render_widget (& self , frame . area ())) ? ; self . handle_events () ? ; self . update (terminal . size () ? . width) ; } Ok (()) } fn update (& mut self , terminal_width : u16) { if self . state != AppState :: Started { return ; } self . progress_columns = (self . progress_columns + 1) . clamp (0 , terminal_width) ; self . progress1 = self . progress_columns * 100 / terminal_width ; self . progress2 = f64 :: from (self . progress_columns) * 100.0 / f64 :: from (terminal_width) ; self . progress3 = (self . progress3 + 0.1) . clamp (40.0 , 100.0) ; self . progress4 = (self . progress4 + 0.1) . clamp (40.0 , 100.0) ; } fn handle_events (& mut self) -> Result < () > { let timeout = Duration :: from_secs_f32 (1.0 / 20.0) ; if ! event :: poll (timeout) ? { return Ok (()) ; } if let Some (key) = event :: read () ? . as_key_press_event () { match key . code { KeyCode :: Char (' ') | KeyCode :: Enter => self . start () , KeyCode :: Char ('q') | KeyCode :: Esc => self . quit () , _ => { } } } Ok (()) } const fn start (& mut self) { self . state = AppState :: Started ; } const fn quit (& mut self) { self . state = AppState :: Quitting ; } }
};
}
