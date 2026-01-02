// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/os/unix/thread.rs
// Error: expected square brackets
// Problematic line: line 7


#![stable(feature = "thread_extensions", since = "1.9.0")]

#[allow(deprecated)]
use crate::os::unix::raw::pthread_t;
use crate::sys_common::{AsInner, IntoInner};
use crate::thread::JoinHandle;
