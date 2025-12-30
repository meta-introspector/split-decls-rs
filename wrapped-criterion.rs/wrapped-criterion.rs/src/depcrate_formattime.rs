// Generated macro for time (function)
macro_rules! Depcrate_formattime {
() => {
// Module: crate::format
// Provides: {"time"}
// Dependencies: {}
pub fn time (ns : f64) -> String { if ns < 1.0 { format ! ("{:>6} ps" , short (ns * 1e3)) } else if ns < 10f64 . powi (3) { format ! ("{:>6} ns" , short (ns)) } else if ns < 10f64 . powi (6) { format ! ("{:>6} µs" , short (ns / 1e3)) } else if ns < 10f64 . powi (9) { format ! ("{:>6} ms" , short (ns / 1e6)) } else { format ! ("{:>6} s" , short (ns / 1e9)) } }
};
}
