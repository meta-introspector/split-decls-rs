// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () -> Result < () > { let buffer = MemoryBuffer :: Create (11) ? ; let reference = buffer . CreateReference () ? ; assert_eq ! (reference . Capacity () ?, 11) ; { let slice = unsafe { as_mut_slice (& reference) ? } ; slice . copy_from_slice (b"hello world") ; } { let slice = unsafe { as_mut_slice (& reference) ? } ; assert_eq ! (slice , b"hello world") ; } Ok (()) }
};
}
