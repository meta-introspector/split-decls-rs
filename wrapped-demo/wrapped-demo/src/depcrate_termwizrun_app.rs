// Generated macro for run_app (function)
macro_rules! Depcrate_termwizrun_app {
() => {
// Module: crate::termwiz
// Provides: {"run_app"}
// Dependencies: {}
fn run_app (terminal : & mut Terminal < TermwizBackend > , mut app : App , tick_rate : Duration ,) -> Result < () , Box < dyn Error > > { let mut last_tick = Instant :: now () ; loop { terminal . draw (| frame | ui :: render (frame , & mut app)) ? ; let timeout = tick_rate . saturating_sub (last_tick . elapsed ()) ; if let Some (input) = terminal . backend_mut () . buffered_terminal_mut () . terminal () . poll_input (Some (timeout)) ? { match input { InputEvent :: Key (key_code) => match key_code . key { KeyCode :: UpArrow | KeyCode :: Char ('k') => app . on_up () , KeyCode :: DownArrow | KeyCode :: Char ('j') => app . on_down () , KeyCode :: LeftArrow | KeyCode :: Char ('h') => app . on_left () , KeyCode :: RightArrow | KeyCode :: Char ('l') => app . on_right () , KeyCode :: Char (c) => app . on_key (c) , _ => { } } , InputEvent :: Resized { cols , rows } => { terminal . backend_mut () . buffered_terminal_mut () . resize (cols , rows) ; } _ => { } } } if last_tick . elapsed () >= tick_rate { app . on_tick () ; last_tick = Instant :: now () ; } if app . should_quit { return Ok (()) ; } } }
};
}
