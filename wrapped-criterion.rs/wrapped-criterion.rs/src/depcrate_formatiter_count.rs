// Generated macro for iter_count (function)
macro_rules! Depcrate_formatiter_count {
() => {
// Module: crate::format
// Provides: {"iter_count"}
// Dependencies: {}
pub fn iter_count (iterations : u64) -> String { if iterations < 10_000 { format ! ("{} iterations" , iterations) } else if iterations < 1_000_000 { format ! ("{:.0}k iterations" , (iterations as f64) / 1000.0) } else if iterations < 10_000_000 { format ! ("{:.1}M iterations" , (iterations as f64) / (1000.0 * 1000.0)) } else if iterations < 1_000_000_000 { format ! ("{:.0}M iterations" , (iterations as f64) / (1000.0 * 1000.0)) } else if iterations < 10_000_000_000 { format ! ("{:.1}B iterations" , (iterations as f64) / (1000.0 * 1000.0 * 1000.0)) } else { format ! ("{:.0}B iterations" , (iterations as f64) / (1000.0 * 1000.0 * 1000.0)) } }
};
}
