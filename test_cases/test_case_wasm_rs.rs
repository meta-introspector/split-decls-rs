// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/sys/alloc/wasm.rs
// Error: expected square brackets
// Problematic line: line 26

struct SyncDlmalloc(dlmalloc::Dlmalloc);
unsafe impl Sync for SyncDlmalloc {}

static DLMALLOC: SyncUnsafeCell<SyncDlmalloc> =
    SyncUnsafeCell::new(SyncDlmalloc(dlmalloc::Dlmalloc::new()));

#[stable(feature = "alloc_system_type", since = "1.28.0")]
