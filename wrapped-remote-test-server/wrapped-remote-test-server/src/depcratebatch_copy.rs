// Generated macro for batch_copy (function)
macro_rules! Depcratebatch_copy {
() => {
// Module: crate
// Provides: {"batch_copy"}
// Dependencies: {}
fn batch_copy (buf : & [u8] , which : u8 , dst : & Mutex < dyn Write >) { let n = buf . len () ; let mut dst = dst . lock () . unwrap () ; t ! (dst . write_all (& create_header (which , n as u64))) ; if n > 0 { t ! (dst . write_all (buf)) ; t ! (dst . write_all (& [which , 0 , 0 , 0 , 0 ,])) ; } }
};
}
