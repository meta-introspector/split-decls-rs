// Generated macro for make_timeout (function)
macro_rules! Depcrate_connectionmake_timeout {
() => {
// Module: crate::connection
// Provides: {"make_timeout"}
// Dependencies: {}
fn make_timeout (timeout : Instant) -> pin :: Pin < Box < dyn future :: Future < Output = () > + Send + Sync + 'static > > { let t = tokio :: time :: sleep_until (timeout . into ()) ; Box :: pin (t) }
};
}
