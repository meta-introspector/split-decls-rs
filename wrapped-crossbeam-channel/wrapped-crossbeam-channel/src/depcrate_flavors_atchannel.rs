// Generated macro for Channel (struct)
macro_rules! Depcrate_flavors_atChannel {
() => {
// Module: crate::flavors::at
// Provides: {"Channel"}
// Dependencies: {}
# [doc = " Channel that delivers a message at a certain moment in time"] pub (crate) struct Channel { # [doc = " The instant at which the message will be delivered."] delivery_time : Instant , # [doc = " `true` if the message has been received."] received : AtomicBool , }
};
}
