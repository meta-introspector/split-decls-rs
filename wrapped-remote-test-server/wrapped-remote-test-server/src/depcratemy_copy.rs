// Generated macro for my_copy (function)
macro_rules! Depcratemy_copy {
() => {
// Module: crate
// Provides: {"my_copy"}
// Dependencies: {}
fn my_copy (src : & mut dyn Read , which : u8 , dst : & Mutex < dyn Write >) { let mut b = [0 ; 1024] ; loop { let n = t ! (src . read (& mut b)) ; let mut dst = dst . lock () . unwrap () ; t ! (dst . write_all (& create_header (which , n as u64))) ; if n > 0 { t ! (dst . write_all (& b [.. n])) ; } else { break ; } } }
};
}
