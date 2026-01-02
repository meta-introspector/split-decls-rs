// MINIMAL TEST CASE for parsing failure in: ../rust/library/panic_unwind/src/miri.rs
// Error: expected square brackets
// Problematic line: line 10

// Must be pointer-sized.
type Payload = Box<Box<dyn Any + Send>>;

unsafe extern "Rust" {
    /// Miri-provided extern function to begin unwinding.
    fn miri_start_unwind(payload: *mut u8) -> !;
}
