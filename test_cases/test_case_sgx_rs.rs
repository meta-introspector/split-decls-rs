// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/sys/args/sgx.rs
// Error: expected square brackets
// Problematic line: line 14

use crate::{fmt, slice};

// Specifying linkage/symbol name is solely to ensure a single instance between this crate and its unit tests
#[cfg_attr(test, linkage = "available_externally")]
#[unsafe(export_name = "_ZN16__rust_internals3std3sys3sgx4args4ARGSE")]
static ARGS: Atomic<usize> = AtomicUsize::new(0);
type ArgsStore = Vec<OsString>;
