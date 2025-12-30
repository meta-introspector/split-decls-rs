// Generated macro for impl_13 (impl)
macro_rules! Depcrateimpl_13 {
() => {
// Module: crate
// Provides: {"impl_13"}
// Dependencies: {}
impl App { fn run (mut self , terminal : & mut DefaultTerminal) -> Result < Option < InputForm > > { while self . state == AppState :: Running { terminal . draw (| frame | self . render (frame)) ? ; self . handle_events () ? ; } match self . state { AppState :: Cancelled => Ok (None) , AppState :: Submitted => Ok (Some (self . form)) , AppState :: Running => unreachable ! () , } } fn render (& self , frame : & mut Frame) { self . form . render (frame) ; } fn handle_events (& mut self) -> Result < () > { if let Some (key) = event :: read () ? . as_key_press_event () { match key . code { KeyCode :: Esc => self . state = AppState :: Cancelled , KeyCode :: Enter => self . state = AppState :: Submitted , _ => self . form . on_key_press (key) , } } Ok (()) } }
};
}
