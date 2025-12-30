// Generated macro for Runner (struct)
macro_rules! DepcrateRunner {
() => {
// Module: crate
// Provides: {"Runner"}
// Dependencies: {}
# [doc = " A worker in a work-stealing executor."] # [doc = ""] # [doc = " This is just a ticker that also has an associated local queue for improved cache locality."] struct Runner < 'a > { # [doc = " The executor state."] state : & 'a State , # [doc = " Inner ticker."] ticker : Ticker < 'a > , # [doc = " The local queue."] local : Arc < ConcurrentQueue < Runnable > > , # [doc = " Bumped every time a runnable task is found."] ticks : usize , }
};
}
