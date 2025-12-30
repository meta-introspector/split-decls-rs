// Generated macro for would_block (function)
macro_rules! Depcratewould_block {
() => {
// Module: crate
// Provides: {"would_block"}
// Dependencies: {}
fn would_block (err : & io :: Error) -> bool { err . kind () == io :: ErrorKind :: WouldBlock }
};
}
