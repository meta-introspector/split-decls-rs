// Generated macro for input_handling (function)
macro_rules! Depcrateinput_handling {
() => {
// Module: crate
// Provides: {"input_handling"}
// Dependencies: {}
fn input_handling (tx : mpsc :: Sender < Event >) { let tick_rate = Duration :: from_millis (200) ; thread :: spawn (move | | { let mut last_tick = Instant :: now () ; loop { let timeout = tick_rate . saturating_sub (last_tick . elapsed ()) ; if event :: poll (timeout) . unwrap () { match event :: read () . unwrap () { event :: Event :: Key (key) => tx . send (Event :: Input (key)) . unwrap () , event :: Event :: Resize (_ , _) => tx . send (Event :: Resize) . unwrap () , _ => { } } } if last_tick . elapsed () >= tick_rate { tx . send (Event :: Tick) . unwrap () ; last_tick = Instant :: now () ; } } }) ; }
};
}
