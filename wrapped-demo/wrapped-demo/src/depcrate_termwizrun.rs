// Generated macro for run (function)
macro_rules! Depcrate_termwizrun {
() => {
// Module: crate::termwiz
// Provides: {"run"}
// Dependencies: {}
pub fn run (tick_rate : Duration , enhanced_graphics : bool) -> Result < () , Box < dyn Error > > { let backend = TermwizBackend :: new () ? ; let mut terminal = Terminal :: new (backend) ? ; terminal . hide_cursor () ? ; let app = App :: new ("Termwiz Demo" , enhanced_graphics) ; let app_result = run_app (& mut terminal , app , tick_rate) ; terminal . show_cursor () ? ; terminal . flush () ? ; if let Err (err) = app_result { println ! ("{err:?}") ; } Ok (()) }
};
}
