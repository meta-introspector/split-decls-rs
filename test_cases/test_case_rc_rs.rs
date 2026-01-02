// MINIMAL TEST CASE for parsing failure in: ../rust/library/alloc/src/rc.rs
// Error: expected square brackets
// Problematic line: line 246


use core::any::Any;
use core::cell::Cell;
#[cfg(not(no_global_oom_handling))]
use core::clone::CloneToUninit;
use core::clone::UseCloned;
use core::cmp::Ordering;
