// Generated macro for run_app (function)
macro_rules! Depcrate_crosstermrun_app {
() => {
// Module: crate::crossterm
// Provides: {"run_app"}
// Dependencies: {}
fn run_app < B : Backend > (terminal : & mut Terminal < B > , mut app : App , tick_rate : Duration ,) -> Result < () , Box < dyn Error > > where B :: Error : 'static , { let mut last_tick = Instant :: now () ; loop { terminal . draw (| frame | ui :: render (frame , & mut app)) ? ; let timeout = tick_rate . saturating_sub (last_tick . elapsed ()) ; if ! event :: poll (timeout) ? { app . on_tick () ; last_tick = Instant :: now () ; continue ; } if let Some (key) = event :: read () ? . as_key_press_event () { match key . code { KeyCode :: Char ('h') | KeyCode :: Left => app . on_left () , KeyCode :: Char ('j') | KeyCode :: Down => app . on_down () , KeyCode :: Char ('k') | KeyCode :: Up => app . on_up () , KeyCode :: Char ('l') | KeyCode :: Right => app . on_right () , KeyCode :: Char (c) => app . on_key (c) , _ => { } } } if app . should_quit { return Ok (()) ; } } }
};
}
