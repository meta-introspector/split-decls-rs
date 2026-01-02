// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/sys/random/arc4random.rs
// Error: expected square brackets
// Problematic line: line 12

//! wrapper for `getrandom`. Since we need to hook into `getrandom` directly
//! for `HashMap` keys anyway, we just keep our version).

#[cfg(not(any(
    target_os = "haiku",
    target_os = "illumos",
    target_os = "solaris",
