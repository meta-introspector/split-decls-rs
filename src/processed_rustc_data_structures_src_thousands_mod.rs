/* FP:mod.rs-0001 */ // This is a bare-bones alternative to the `thousands` crate on crates.io, for
/* FP:mod.rs-0002 */ // printing large numbers in a readable fashion.
/* FP:mod.rs-0003 */ 
/* FP:mod.rs-0004 */ #[cfg(test)]
/* FP:mod.rs-0006 */ 
/* FP:mod.rs-0007 */ fn format_with_underscores(mut s: String) -> String {
/* FP:mod.rs-0008 */     // Ignore a leading '-'.
/* FP:mod.rs-0009 */     let start = if s.starts_with('-') { 1 } else { 0 };
/* FP:mod.rs-0010 */ 
/* FP:mod.rs-0011 */     // Stop after the first non-digit, e.g. '.' or 'e' for floats.
/* FP:mod.rs-0012 */     let non_digit = s[start..].find(|c: char| !c.is_digit(10));
/* FP:mod.rs-0013 */     let end = if let Some(non_digit) = non_digit { start + non_digit } else { s.len() };
/* FP:mod.rs-0014 */ 
/* FP:mod.rs-0015 */     // Insert underscores within `start..end`.
/* FP:mod.rs-0016 */     let mut i = end;
/* FP:mod.rs-0017 */     while i > start + 3 {
/* FP:mod.rs-0018 */         i -= 3;
/* FP:mod.rs-0019 */         s.insert(i, '_');
/* FP:mod.rs-0020 */     }
/* FP:mod.rs-0021 */     s
/* FP:mod.rs-0022 */ }
/* FP:mod.rs-0023 */ 
/* FP:mod.rs-0024 */ /// Print a `usize` with underscore separators.
/* FP:mod.rs-0025 */ pub fn usize_with_underscores(n: usize) -> String {
/* FP:mod.rs-0026 */     format_with_underscores(format!("{n}"))
/* FP:mod.rs-0027 */ }
/* FP:mod.rs-0028 */ 
/* FP:mod.rs-0029 */ /// Print an `isize` with underscore separators.
/* FP:mod.rs-0030 */ pub fn isize_with_underscores(n: isize) -> String {
/* FP:mod.rs-0031 */     format_with_underscores(format!("{n}"))
/* FP:mod.rs-0032 */ }
/* FP:mod.rs-0033 */ 
/* FP:mod.rs-0034 */ /// Print an `f64` with precision 1 (one decimal place) and underscore separators.
/* FP:mod.rs-0035 */ pub fn f64p1_with_underscores(n: f64) -> String {
/* FP:mod.rs-0036 */     format_with_underscores(format!("{n:.1}"))
/* FP:mod.rs-0037 */ }