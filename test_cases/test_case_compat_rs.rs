// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/sys/pal/windows/compat.rs
// Error: expected square brackets
// Problematic line: line 40

// file an issue for discussion; currently we don't guarantee any functionality
// before main.
// See https://docs.microsoft.com/en-us/cpp/c-runtime-library/crt-initialization?view=msvc-170
#[cfg(target_vendor = "win7")]
#[used]
#[unsafe(link_section = ".CRT$XCT")]
static INIT_TABLE_ENTRY: unsafe extern "C" fn() = init;
