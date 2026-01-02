// MINIMAL TEST CASE for parsing failure in: ../rust/library/backtrace/src/symbolize/dladdr.rs
// Error: expected `,`
// Error type: expected_comma
// Sample #1 of 3
// Problematic line: line 7


use Symbol;

#[repr(C)]
struct Dl_info {
    dli_fname: *const c_char,
    dli_fbase: *mut c_void,
