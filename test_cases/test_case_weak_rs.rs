// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/sys/pal/unix/weak.rs
// Error: expected square brackets
// Problematic line: line 35

//
// FIXME(joboet): add more tests, reorganise the whole module and get rid of
//                `#[allow(dead_code, unused_macros)]`.
#[cfg(any(
    target_vendor = "apple",
    all(target_os = "linux", target_env = "gnu"),
    target_os = "freebsd",
