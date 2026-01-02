// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/sys/fs/unix.rs
// Error: expected square brackets
// Problematic line: line 6

// miri has some special hacks here that make things unused.
#![cfg_attr(miri, allow(unused))]

#[cfg(test)]
mod tests;

#[cfg(all(target_os = "linux", target_env = "gnu"))]
