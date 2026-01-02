// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/sys/thread_local/key/racy.rs
// Error: expected square brackets
// Problematic line: line 11


use crate::sync::atomic::{Atomic, AtomicUsize, Ordering};

/// A type for TLS keys that are statically allocated.
///
/// This is basically a `LazyLock<Key>`, but avoids blocking and circular
/// dependencies with the rest of `std`.
