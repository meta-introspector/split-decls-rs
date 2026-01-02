// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/sys/pal/sgx/libunwind_integration.rs
// Error: expected square brackets
// Problematic line: line 12

// Verify that the byte pattern libunwind uses to initialize an RwLock is
// equivalent to the value of RwLock::new(). If the value changes,
// `src/UnwindRustSgx.h` in libunwind needs to be changed too.
const _: () = unsafe {
    let bits_rust: usize = crate::mem::transmute(RwLock::new());
    assert!(bits_rust == 0);
};
