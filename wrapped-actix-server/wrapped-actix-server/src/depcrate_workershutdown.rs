// Generated macro for Shutdown (struct)
macro_rules! Depcrate_workerShutdown {
() => {
// Module: crate::worker
// Provides: {"Shutdown"}
// Dependencies: {}
# [doc = " State necessary for server shutdown."] struct Shutdown { timer : Pin < Box < Sleep > > , # [doc = " Start time of shutdown."] start_from : Instant , # [doc = " Notify caller of the shutdown outcome (graceful/force)."] tx : oneshot :: Sender < bool > , }
};
}
