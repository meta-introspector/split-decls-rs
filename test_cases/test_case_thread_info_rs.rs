// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/sys/pal/unix/stack_overflow/thread_info.rs
// Error: expected square brackets
// Problematic line: line 34

use crate::sync::atomic::{AtomicUsize, Ordering};
use crate::sys::os::errno_location;

pub struct ThreadInfo {
    pub guard_page_range: Range<usize>,
    pub thread_name: Option<Box<str>>,
}
