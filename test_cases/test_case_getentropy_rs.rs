// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/sys/random/getentropy.rs
// Error: expected square brackets
// Problematic line: line 10

//! it where `arc4random_buf` and friends aren't available or secure (currently
//! that's only the case on Emscripten).

pub fn fill_bytes(bytes: &mut [u8]) {
    // GETENTROPY_MAX isn't defined yet on most platforms, but it's mandated
    // to be at least 256, so just use that as limit.
    for chunk in bytes.chunks_mut(256) {
