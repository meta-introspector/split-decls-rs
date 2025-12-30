// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let file = File :: open ("hash_output-295253") . unwrap () ; let rand = ahash :: RandomState :: new () ; let start_time = SystemTime :: now () ; let result = test_hasher (file , rand) . unwrap () ; println ! ("Completed after {:?} with result: {:x}" , SystemTime :: now () . duration_since (start_time) . unwrap () , result) }
};
}
