// Generated macro for write_f64 (function)
macro_rules! Depcrate_datawrite_f64 {
() => {
// Module: crate::data
// Provides: {"write_f64"}
// Dependencies: {}
fn write_f64 (w : & mut impl std :: io :: Write , f : f64) -> std :: io :: Result < () > { w . write_all (& f . to_bits () . to_le_bytes ()) }
};
}
