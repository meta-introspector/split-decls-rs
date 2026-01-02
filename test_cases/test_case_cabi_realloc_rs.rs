// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/sys/pal/wasip2/cabi_realloc.rs
// Error: expected square brackets
// Problematic line: line 26

use crate::alloc::{self, Layout};
use crate::ptr;

#[used]
static FORCE_CODEGEN_OF_CABI_REALLOC: unsafe extern "C" fn(
    *mut u8,
    usize,
