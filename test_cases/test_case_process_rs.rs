// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/os/linux/process.rs
// Error: expected square brackets
// Problematic line: line 11

use crate::os::unix::io::{AsFd, AsRawFd, BorrowedFd, FromRawFd, IntoRawFd, OwnedFd, RawFd};
use crate::process::{self, ExitStatus};
use crate::sealed::Sealed;
#[cfg(not(doc))]
use crate::sys::{fd::FileDesc, linux::pidfd::PidFd as InnerPidFd};
use crate::sys_common::{AsInner, AsInnerMut, FromInner, IntoInner};

