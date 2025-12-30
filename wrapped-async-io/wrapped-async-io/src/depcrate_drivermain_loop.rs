// Generated macro for main_loop (function)
macro_rules! Depcrate_drivermain_loop {
() => {
// Module: crate::driver
// Provides: {"main_loop"}
// Dependencies: {}
# [doc = " The main loop for the \"async-io\" thread."] fn main_loop (parker : parking :: Parker) { # [cfg (feature = "tracing")] let span = tracing :: trace_span ! ("async_io::main_loop") ; # [cfg (feature = "tracing")] let _enter = span . enter () ; let mut last_tick = 0 ; let mut sleeps = 0u64 ; loop { let tick = Reactor :: get () . ticker () ; if last_tick == tick { let reactor_lock = if sleeps >= 10 { Some (Reactor :: get () . lock ()) } else { Reactor :: get () . try_lock () } ; if let Some (mut reactor_lock) = reactor_lock { # [cfg (feature = "tracing")] tracing :: trace ! ("waiting on I/O") ; reactor_lock . react (None) . ok () ; last_tick = Reactor :: get () . ticker () ; sleeps = 0 ; } } else { last_tick = tick ; } if BLOCK_ON_COUNT . load (Ordering :: SeqCst) > 0 { let delay_us = [50 , 75 , 100 , 250 , 500 , 750 , 1000 , 2500 , 5000] . get (sleeps as usize) . unwrap_or (& 10_000) ; # [cfg (feature = "tracing")] tracing :: trace ! ("sleeping for {} us" , delay_us) ; if parker . park_timeout (Duration :: from_micros (* delay_us)) { # [cfg (feature = "tracing")] tracing :: trace ! ("notified") ; last_tick = Reactor :: get () . ticker () ; sleeps = 0 ; } else { sleeps += 1 ; } } } }
};
}
