// Generated macro for InterruptHandle (struct)
macro_rules! DepcrateInterruptHandle {
() => {
// Module: crate
// Provides: {"InterruptHandle"}
// Dependencies: {}
# [doc = " Allows interrupting a long-running computation."] pub struct InterruptHandle { db_lock : Arc < Mutex < * mut ffi :: sqlite3 > > , }
};
}
