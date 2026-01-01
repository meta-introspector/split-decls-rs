# AST Trace: ../rust/library/std/src/os/linux/process.rs

Generated 19 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
//! Linux-specific extensions to primitives in the [`std::process`] module.
//!
//! [`std::process`]: crate::process

#![unstable(feature = "linux_pidfd", issue = "82971")]

use crate::io::Result;
use crate::os::unix::io::{AsFd, AsRawFd, BorrowedFd, FromRawFd, IntoRawFd, OwnedFd, RawFd};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use crate::process::{self, ExitStatus};
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3

```rust
use crate::sealed::Sealed;
#[cfg(not(doc))]
use crate::sys::{fd::FileDesc, linux::pidfd::PidFd as InnerPidFd};
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use crate::sys_common::{AsInner, AsInnerMut, FromInner, IntoInner};
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=STRUCT | NAME=InnerPidFd; | COMPLEXITY=2 | LINES=14

```rust
#[cfg(doc)]
struct InnerPidFd;

/// This type represents a file descriptor that refers to a process.
///
/// A `PidFd` can be obtained by setting the corresponding option on [`Command`]
/// with [`create_pidfd`]. Subsequently, the created pidfd can be retrieved
/// from the [`Child`] by calling [`pidfd`] or [`into_pidfd`].
///
/// Example:
/// ```no_run
/// #![feature(linux_pidfd)]
/// use std::os::linux::process::{CommandExt, ChildExt};
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=STRUCT | NAME=PidFd | COMPLEXITY=5 | LINES=26

```rust
/// use std::process::Command;
///
/// let mut child = Command::new("echo")
///     .create_pidfd(true)
///     .spawn()
///     .expect("Failed to spawn child");
///
/// let pidfd = child
///     .into_pidfd()
///     .expect("Failed to retrieve pidfd");
///
/// // The file descriptor will be closed when `pidfd` is dropped.
/// ```
/// Refer to the man page of [`pidfd_open(2)`] for further details.
///
/// [`Command`]: process::Command
/// [`create_pidfd`]: CommandExt::create_pidfd
/// [`Child`]: process::Child
/// [`pidfd`]: fn@ChildExt::pidfd
/// [`into_pidfd`]: ChildExt::into_pidfd
/// [`pidfd_open(2)`]: https://man7.org/linux/man-pages/man2/pidfd_open.2.html
#[derive(Debug)]
#[repr(transparent)]
pub struct PidFd {
    inner: InnerPidFd,
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=kill | COMPLEXITY=15 | LINES=34

```rust
impl PidFd {
    /// Forces the child process to exit.
    ///
    /// Unlike [`Child::kill`] it is possible to attempt to kill
    /// reaped children since PidFd does not suffer from pid recycling
    /// races. But doing so will return an Error.
    ///
    /// [`Child::kill`]: process::Child::kill
    pub fn kill(&self) -> Result<()> {
        self.inner.kill()
    }

    /// Waits for the child to exit completely, returning the status that it exited with.
    ///
    /// Unlike [`Child::wait`] it does not ensure that the stdin handle is closed.
    /// Additionally it will not return an `ExitStatus` if the child
    /// has already been reaped. Instead an error will be returned.
    ///
    /// [`Child::wait`]: process::Child::wait
    pub fn wait(&self) -> Result<ExitStatus> {
        self.inner.wait().map(FromInner::from_inner)
    }

    /// Attempts to collect the exit status of the child if it has already exited.
    ///
    /// Unlike [`Child::try_wait`] this method will return an Error
    /// if the child has already been reaped.
    ///
    /// [`Child::try_wait`]: process::Child::try_wait
    pub fn try_wait(&self) -> Result<Option<ExitStatus>> {
        Ok(self.inner.try_wait()?.map(FromInner::from_inner))
    }
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=FUNCTION | NAME=as_inner | COMPLEXITY=5 | LINES=7

```rust
impl AsInner<InnerPidFd> for PidFd {
    #[inline]
    fn as_inner(&self) -> &InnerPidFd {
        &self.inner
    }
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=FUNCTION | NAME=from_inner | COMPLEXITY=6 | LINES=6

```rust
impl FromInner<InnerPidFd> for PidFd {
    fn from_inner(inner: InnerPidFd) -> PidFd {
        PidFd { inner }
    }
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=FUNCTION | NAME=into_inner | COMPLEXITY=5 | LINES=6

```rust
impl IntoInner<InnerPidFd> for PidFd {
    fn into_inner(self) -> InnerPidFd {
        self.inner
    }
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=FUNCTION | NAME=as_raw_fd | COMPLEXITY=5 | LINES=7

```rust
impl AsRawFd for PidFd {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.as_inner().as_inner().as_raw_fd()
    }
}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=9 | LINES=6

```rust
impl FromRawFd for PidFd {
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        Self::from_inner(InnerPidFd::from_raw_fd(fd))
    }
}
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=FUNCTION | NAME=into_raw_fd | COMPLEXITY=5 | LINES=6

```rust
impl IntoRawFd for PidFd {
    fn into_raw_fd(self) -> RawFd {
        self.into_inner().into_inner().into_raw_fd()
    }
}
```

## Block 14
**Metadata**: AST_ID=14 | TYPE=FUNCTION | NAME=as_fd | COMPLEXITY=5 | LINES=6

```rust
impl AsFd for PidFd {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.as_inner().as_inner().as_fd()
    }
}
```

## Block 15
**Metadata**: AST_ID=15 | TYPE=FUNCTION | NAME=from | COMPLEXITY=5 | LINES=6

```rust
impl From<OwnedFd> for PidFd {
    fn from(fd: OwnedFd) -> Self {
        Self::from_inner(InnerPidFd::from_inner(FileDesc::from_inner(fd)))
    }
}
```

## Block 16
**Metadata**: AST_ID=16 | TYPE=FUNCTION | NAME=from | COMPLEXITY=5 | LINES=6

```rust
impl From<PidFd> for OwnedFd {
    fn from(pid_fd: PidFd) -> Self {
        pid_fd.into_inner().into_inner().into_inner()
    }
}
```

## Block 17
**Metadata**: AST_ID=17 | TYPE=FUNCTION | NAME=pidfd | COMPLEXITY=27 | LINES=39

```rust
/// Os-specific extensions for [`Child`]
///
/// [`Child`]: process::Child
pub trait ChildExt: Sealed {
    /// Obtains a reference to the [`PidFd`] created for this [`Child`], if available.
    ///
    /// A pidfd will only be available if its creation was requested with
    /// [`create_pidfd`] when the corresponding [`Command`] was created.
    ///
    /// Even if requested, a pidfd may not be available due to an older
    /// version of Linux being in use, or if some other error occurred.
    ///
    /// [`Command`]: process::Command
    /// [`create_pidfd`]: CommandExt::create_pidfd
    /// [`Child`]: process::Child
    fn pidfd(&self) -> Result<&PidFd>;

    /// Returns the [`PidFd`] created for this [`Child`], if available.
    /// Otherwise self is returned.
    ///
    /// A pidfd will only be available if its creation was requested with
    /// [`create_pidfd`] when the corresponding [`Command`] was created.
    ///
    /// Taking ownership of the PidFd consumes the Child to avoid pid reuse
    /// races. Use [`pidfd`] and [`BorrowedFd::try_clone_to_owned`] if
    /// you don't want to disassemble the Child yet.
    ///
    /// Even if requested, a pidfd may not be available due to an older
    /// version of Linux being in use, or if some other error occurred.
    ///
    /// [`Command`]: process::Command
    /// [`create_pidfd`]: CommandExt::create_pidfd
    /// [`pidfd`]: ChildExt::pidfd
    /// [`Child`]: process::Child
    fn into_pidfd(self) -> crate::result::Result<PidFd, Self>
    where
        Self: Sized;
}
```

## Block 18
**Metadata**: AST_ID=18 | TYPE=FUNCTION | NAME=create_pidfd | COMPLEXITY=10 | LINES=26

```rust
/// Os-specific extensions for [`Command`]
///
/// [`Command`]: process::Command
pub trait CommandExt: Sealed {
    /// Sets whether a [`PidFd`](struct@PidFd) should be created for the [`Child`]
    /// spawned by this [`Command`].
    /// By default, no pidfd will be created.
    ///
    /// The pidfd can be retrieved from the child with [`pidfd`] or [`into_pidfd`].
    ///
    /// A pidfd will only be created if it is possible to do so
    /// in a guaranteed race-free manner. Otherwise, [`pidfd`] will return an error.
    ///
    /// If a pidfd has been successfully created and not been taken from the `Child`
    /// then calls to `kill()`, `wait()` and `try_wait()` will use the pidfd
    /// instead of the pid. This can prevent pid recycling races, e.g.
    /// those  caused by rogue libraries in the same process prematurely reaping
    /// zombie children via `waitpid(-1, ...)` calls.
    ///
    /// [`Command`]: process::Command
    /// [`Child`]: process::Child
    /// [`pidfd`]: fn@ChildExt::pidfd
    /// [`into_pidfd`]: ChildExt::into_pidfd
    fn create_pidfd(&mut self, val: bool) -> &mut process::Command;
}
```

## Block 19
**Metadata**: AST_ID=19 | TYPE=FUNCTION | NAME=create_pidfd | COMPLEXITY=5 | LINES=7

```rust
impl CommandExt for process::Command {
    fn create_pidfd(&mut self, val: bool) -> &mut process::Command {
        self.as_inner_mut().create_pidfd(val);
        self
    }
}
```

---
*Generated by AST tracing system*
