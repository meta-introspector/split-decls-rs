// Generated macro for Channel (struct)
macro_rules! Depcrate_flavors_tickChannel {
() => {
// Module: crate::flavors::tick
// Provides: {"Channel"}
// Dependencies: {}
# [doc = " Channel that delivers messages periodically."] pub (crate) struct Channel { # [doc = " The instant at which the next message will be delivered."] delivery_time : AtomicCell < Instant > , # [doc = " The time interval in which messages get delivered."] duration : Duration , }
};
}
