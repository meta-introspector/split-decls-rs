// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/num/flt2dec/strategy/dragon.rs
// Error: expected square brackets
// Problematic line: line 13

use crate::num::flt2dec::estimator::estimate_scaling_factor;
use crate::num::flt2dec::{Decoded, MAX_SIG_DIGITS, round_up};

static POW10: [Digit; 10] =
    [1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, 1000000000];
// precalculated arrays of `Digit`s for 5^(2^n).
static POW5TO16: [Digit; 2] = [0x86f26fc1, 0x23];
