// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_driver_impl/src/signal_handler.rs
// Error: expected square brackets
// Problematic line: line 9


use rustc_interface::util::{DEFAULT_STACK_SIZE, STACK_SIZE};

/// Signals that represent that we have a bug, and our prompt termination has
/// been ordered.
#[rustfmt::skip]
const KILL_SIGNALS: [(libc::c_int, &str); 3] = [
