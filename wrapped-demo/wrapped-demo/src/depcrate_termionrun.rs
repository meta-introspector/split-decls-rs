// Generated macro for run (function)
macro_rules! Depcrate_termionrun {
() => {
// Module: crate::termion
// Provides: {"run"}
// Dependencies: {}
pub fn run (tick_rate : Duration , enhanced_graphics : bool) -> Result < () , Box < dyn Error > > { let stdout = io :: stdout () . into_raw_mode () . unwrap () . into_alternate_screen () . unwrap () ; let stdout = MouseTerminal :: from (stdout) ; let backend = TermionBackend :: new (stdout) ; let mut terminal = Terminal :: new (backend) ? ; let app = App :: new ("Termion demo" , enhanced_graphics) ; run_app (& mut terminal , app , tick_rate) ? ; Ok (()) }
};
}
