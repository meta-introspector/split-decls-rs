// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/sys/pal/unix/fuchsia.rs
// Error: expected square brackets
// Problematic line: line 16


pub const ZX_TIME_INFINITE: zx_time_t = i64::MAX;

unsafe extern "C" {
    pub safe fn zx_clock_get_monotonic() -> zx_time_t;
}

