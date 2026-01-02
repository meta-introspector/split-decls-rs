// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/sys/pal/zkvm/abi.rs
// Error: expected square brackets
// Problematic line: line 13

#![allow(dead_code)]
pub const DIGEST_WORDS: usize = 8;

/// Standard IO file descriptors for use with sys_read and sys_write.
pub mod fileno {
    pub const STDIN: u32 = 0;
    pub const STDOUT: u32 = 1;
