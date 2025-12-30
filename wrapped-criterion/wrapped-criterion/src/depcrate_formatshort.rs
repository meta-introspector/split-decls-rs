// Generated macro for short (function)
macro_rules! Depcrate_formatshort {
() => {
// Module: crate::format
// Provides: {"short"}
// Dependencies: {}
pub fn short (n : f64) -> String { if n < 10.0 { format ! ("{:.4}" , n) } else if n < 100.0 { format ! ("{:.3}" , n) } else if n < 1000.0 { format ! ("{:.2}" , n) } else if n < 10000.0 { format ! ("{:.1}" , n) } else { format ! ("{:.0}" , n) } }
};
}
