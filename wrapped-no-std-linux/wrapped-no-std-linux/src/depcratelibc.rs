// Generated macro for libc (module)
macro_rules! Depcratelibc {
() => {
// Module: crate
// Provides: {"libc"}
// Dependencies: {}
mod libc { # [link (name = "c")] extern "C" { pub fn abort () -> ! ; pub fn exit (status : i32) -> ! ; pub fn puts (s : * const i8) -> i32 ; } }
};
}
