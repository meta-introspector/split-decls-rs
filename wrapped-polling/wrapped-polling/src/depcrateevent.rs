// Generated macro for Event (struct)
macro_rules! DepcrateEvent {
() => {
// Module: crate
// Provides: {"Event"}
// Dependencies: {}
# [doc = " Indicates that a file descriptor or socket can read or write without blocking."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct Event { # [doc = " Key identifying the file descriptor or socket."] pub key : usize , # [doc = " Can it do a read operation without blocking?"] pub readable : bool , # [doc = " Can it do a write operation without blocking?"] pub writable : bool , # [doc = " System-specific event data."] extra : sys :: EventExtra , }
};
}
