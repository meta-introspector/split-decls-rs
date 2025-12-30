// Generated macro for run (function)
macro_rules! Depcrate_crosstermrun {
() => {
// Module: crate::crossterm
// Provides: {"run"}
// Dependencies: {}
pub fn run (tick_rate : Duration , enhanced_graphics : bool) -> Result < () , Box < dyn Error > > { enable_raw_mode () ? ; let mut stdout = io :: stdout () ; execute ! (stdout , EnterAlternateScreen , EnableMouseCapture) ? ; let backend = CrosstermBackend :: new (stdout) ; let mut terminal = Terminal :: new (backend) ? ; let app = App :: new ("Crossterm Demo" , enhanced_graphics) ; let app_result = run_app (& mut terminal , app , tick_rate) ; disable_raw_mode () ? ; execute ! (terminal . backend_mut () , LeaveAlternateScreen , DisableMouseCapture) ? ; terminal . show_cursor () ? ; if let Err (err) = app_result { println ! ("{err:?}") ; } Ok (()) }
};
}
