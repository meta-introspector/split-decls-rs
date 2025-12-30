// Generated macro for IOResourceRegistration (enum)
macro_rules! Depcrate_connectionIOResourceRegistration {
() => {
// Module: crate::connection
// Provides: {"IOResourceRegistration"}
// Dependencies: {}
enum IOResourceRegistration { Unregistered (RawFd , tokio :: io :: Interest) , Registered (AsyncFd < RawFd >) , }
};
}
