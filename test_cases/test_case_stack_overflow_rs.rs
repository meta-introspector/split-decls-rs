// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/sys/pal/unix/stack_overflow.rs
// Error: expected square brackets
// Problematic line: line 6

pub use self::imp::{cleanup, init};
use self::imp::{drop_handler, make_handler};

pub struct Handler {
    data: *mut libc::c_void,
}

