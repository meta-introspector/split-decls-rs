// Generated macro for log_bits (function)
macro_rules! Depcrate_satlog_bits {
() => {
// Module: crate::sat
// Provides: {"log_bits"}
// Dependencies: {}
fn log_bits (x : usize) -> usize { if x == 0 { return 0 ; } assert ! (x > 0) ; (num_bits :: < usize > () as u32 - x . leading_zeros ()) as usize }
};
}
