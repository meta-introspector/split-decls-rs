// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/sys/platform_version/darwin/core_foundation.rs
// Error: expected square brackets
// Problematic line: line 32

// CoreFoundation/CFDictionary.h
pub(super) type CFDictionaryRef = CFTypeRef;

/// An open handle to the dynamically loaded CoreFoundation framework.
///
/// This is `dlopen`ed, and later `dlclose`d. This is done to try to avoid
/// "leaking" the CoreFoundation symbols to the rest of the user's binary if
