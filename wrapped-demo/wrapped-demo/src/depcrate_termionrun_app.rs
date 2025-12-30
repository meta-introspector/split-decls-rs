// Generated macro for run_app (function)
macro_rules! Depcrate_termionrun_app {
() => {
// Module: crate::termion
// Provides: {"run_app"}
// Dependencies: {}
fn run_app < B : Backend > (terminal : & mut Terminal < B > , mut app : App , tick_rate : Duration ,) -> Result < () , Box < dyn Error > > where B :: Error : 'static , { let events = events (tick_rate) ; loop { terminal . draw (| frame | ui :: render (frame , & mut app)) ? ; match events . recv () ? { Event :: Input (key) => match key { Key :: Up | Key :: Char ('k') => app . on_up () , Key :: Down | Key :: Char ('j') => app . on_down () , Key :: Left | Key :: Char ('h') => app . on_left () , Key :: Right | Key :: Char ('l') => app . on_right () , Key :: Char (c) => app . on_key (c) , _ => { } } , Event :: Tick => app . on_tick () , } if app . should_quit { return Ok (()) ; } } }
};
}
