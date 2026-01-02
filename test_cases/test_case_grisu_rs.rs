// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/num/flt2dec/strategy/grisu.rs
// Error: expected square brackets
// Problematic line: line 13

use crate::num::flt2dec::{Decoded, MAX_SIG_DIGITS, round_up};

// see the comments in `format_shortest_opt` for the rationale.
#[doc(hidden)]
pub const ALPHA: i16 = -60;
#[doc(hidden)]
pub const GAMMA: i16 = -32;
