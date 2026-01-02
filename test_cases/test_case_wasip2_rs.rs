// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/sys/net/connection/socket/wasip2.rs
// Error: expected square brackets
// Problematic line: line 16

use crate::time::{Duration, Instant};
use crate::{cmp, mem, str};

#[allow(non_camel_case_types)]
pub type wrlen_t = size_t;

#[doc(hidden)]
