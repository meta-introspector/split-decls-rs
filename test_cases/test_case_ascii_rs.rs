// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/slice/ascii.rs
// Error: expected square brackets
// Problematic line: line 6

use core::ascii::EscapeDefault;

use crate::fmt::{self, Write};
#[cfg(not(all(target_arch = "x86_64", target_feature = "sse2")))]
use crate::intrinsics::const_eval_select;
use crate::{ascii, iter, ops};

