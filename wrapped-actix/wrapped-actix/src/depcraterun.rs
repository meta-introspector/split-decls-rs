// Generated macro for run (function)
macro_rules! Depcraterun {
() => {
// Module: crate
// Provides: {"run"}
// Dependencies: {}
# [doc = " Starts the system and executes the supplied future."] # [doc = ""] # [doc = " This function does the following:"] # [doc = ""] # [doc = " * Creates and starts the actix system with default configuration."] # [doc = " * Spawns the given future onto the current arbiter."] # [doc = " * Blocks the current thread until the system shuts down."] # [doc = ""] # [doc = " The `run` function returns when the `System::current().stop()`"] # [doc = " method gets called."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::time::{Duration, Instant};"] # [doc = " use actix_rt::time::sleep;"] # [doc = ""] # [doc = " fn main() {"] # [doc = "   actix::run(async move {"] # [doc = "       sleep(Duration::from_millis(100)).await;"] # [doc = "       actix::System::current().stop();"] # [doc = "   });"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " This function panics if the actix system is already running."] # [allow (clippy :: unit_arg , clippy :: needless_doctest_main)] pub fn run < R > (f : R) -> std :: io :: Result < () > where R : std :: future :: Future < Output = () > + 'static , { Ok (actix_rt :: System :: new () . block_on (f)) }
};
}
