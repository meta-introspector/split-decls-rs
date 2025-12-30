// Generated macro for fmt_size (function)
macro_rules! Depcrate_progressfmt_size {
() => {
// Module: crate::progress
// Provides: {"fmt_size"}
// Dependencies: {}
# [doc = " Converts a quantity in bytes to a human readable size, \"GiB, MiB, KiB, etc\""] pub fn fmt_size (size_in_bytes : f64) -> String { let units = ["B" , "KiB" , "MiB" , "GiB" , "TiB" , "PiB"] ; let order_of_magnitude = (size_in_bytes) . log10 () as usize ; let upper_bound = 3 ; let unit_index = (order_of_magnitude / upper_bound) . clamp (0 , units . len () - 1) ; let decimal = size_in_bytes / 2_f64 . powi ((unit_index * 10) as i32) ; if unit_index > 0 { format ! ("{:.2}{}" , decimal , units [unit_index]) } else { format ! ("{:.0}{}" , decimal , units [unit_index]) } }
};
}
