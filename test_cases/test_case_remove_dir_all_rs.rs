// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/sys/fs/windows/remove_dir_all.rs
// Error: expected square brackets
// Problematic line: line 42

// The maximum number of times to spin when waiting for deletes to complete.
const MAX_RETRIES: usize = 50;

/// A wrapper around a raw NtOpenFile call.
///
/// This isn't completely safe because `OBJECT_ATTRIBUTES` contains raw pointers.
unsafe fn nt_open_file(
