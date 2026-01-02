// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_session/src/config/sigpipe.rs
// Error: expected square brackets
// Problematic line: line 3

//! NOTE: Keep these constants in sync with `library/std/src/sys/pal/unix/mod.rs`!

/// The default value if `-Zon-broken-pipe=...` is not specified. This resolves
/// to `SIG_IGN` in `library/std/src/sys/pal/unix/mod.rs`.
///
/// Note that `SIG_IGN` has been the Rust default since 2014. See
