# AST Trace: ../rust/library/std/src/sys/process/unix/vxworks.rs

Generated 11 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
#![forbid(unsafe_op_in_unsafe_fn)]
use libc::{self, RTP_ID, c_char, c_int};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3

```rust
use super::common::*;
use crate::io::{self, ErrorKind};
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=4

```rust
use crate::num::NonZero;
use crate::sys::cvt;
use crate::sys::pal::thread;
use crate::{fmt, sys};
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=FUNCTION | NAME=spawn | COMPLEXITY=69 | LINES=123

```rust
////////////////////////////////////////////////////////////////////////////////
// Command
////////////////////////////////////////////////////////////////////////////////

impl Command {
    pub fn spawn(
        &mut self,
        default: Stdio,
        needs_stdin: bool,
    ) -> io::Result<(Process, StdioPipes)> {
        use crate::sys::cvt_r;
        let envp = self.capture_env();

        if self.saw_nul() {
            return Err(io::const_error!(
                ErrorKind::InvalidInput,
                "nul byte found in provided data",
            ));
        }
        if self.get_chroot().is_some() {
            return Err(io::const_error!(
                ErrorKind::Unsupported,
                "chroot not supported by vxworks",
            ));
        }
        let (ours, theirs) = self.setup_io(default, needs_stdin)?;
        let mut p = Process { pid: 0, status: None };

        unsafe {
            macro_rules! t {
                ($e:expr) => {
                    match $e {
                        Ok(e) => e,
                        Err(e) => return Err(e.into()),
                    }
                };
            }

            let mut orig_stdin = libc::STDIN_FILENO;
            let mut orig_stdout = libc::STDOUT_FILENO;
            let mut orig_stderr = libc::STDERR_FILENO;

            if let Some(fd) = theirs.stdin.fd() {
                orig_stdin = t!(cvt_r(|| libc::dup(libc::STDIN_FILENO)));
                t!(cvt_r(|| libc::dup2(fd, libc::STDIN_FILENO)));
            }
            if let Some(fd) = theirs.stdout.fd() {
                orig_stdout = t!(cvt_r(|| libc::dup(libc::STDOUT_FILENO)));
                t!(cvt_r(|| libc::dup2(fd, libc::STDOUT_FILENO)));
            }
            if let Some(fd) = theirs.stderr.fd() {
                orig_stderr = t!(cvt_r(|| libc::dup(libc::STDERR_FILENO)));
                t!(cvt_r(|| libc::dup2(fd, libc::STDERR_FILENO)));
            }

            if let Some(cwd) = self.get_cwd() {
                t!(cvt(libc::chdir(cwd.as_ptr())));
            }

            // pre_exec closures are ignored on VxWorks
            let _ = self.get_closures();

            let c_envp = envp
                .as_ref()
                .map(|c| c.as_ptr())
                .unwrap_or_else(|| *sys::env::environ() as *const _);
            let stack_size = crate::cmp::max(
                crate::env::var_os("RUST_MIN_STACK")
                    .and_then(|s| s.to_str().and_then(|s| s.parse().ok()))
                    .unwrap_or(thread::DEFAULT_MIN_STACK_SIZE),
                libc::PTHREAD_STACK_MIN,
            );

            // ensure that access to the environment is synchronized
            let _lock = sys::env::env_read_lock();

            let ret = libc::rtpSpawn(
                self.get_program_cstr().as_ptr(),
                self.get_argv().as_ptr() as *mut *const c_char, // argv
                c_envp as *mut *const c_char,
                100 as c_int, // initial priority
                stack_size,   // initial stack size.
                0,            // options
                0,            // task options
            );

            // Because FileDesc was not used, each duplicated file descriptor
            // needs to be closed manually
            if orig_stdin != libc::STDIN_FILENO {
                t!(cvt_r(|| libc::dup2(orig_stdin, libc::STDIN_FILENO)));
                libc::close(orig_stdin);
            }
            if orig_stdout != libc::STDOUT_FILENO {
                t!(cvt_r(|| libc::dup2(orig_stdout, libc::STDOUT_FILENO)));
                libc::close(orig_stdout);
            }
            if orig_stderr != libc::STDERR_FILENO {
                t!(cvt_r(|| libc::dup2(orig_stderr, libc::STDERR_FILENO)));
                libc::close(orig_stderr);
            }

            if ret != libc::RTP_ID_ERROR {
                p.pid = ret;
                Ok((p, ours))
            } else {
                Err(io::Error::last_os_error())
            }
        }
    }

    pub fn exec(&mut self, default: Stdio) -> io::Error {
        let ret = Command::spawn(self, default, false);
        match ret {
            Ok(t) => unsafe {
                let mut status = 0 as c_int;
                libc::waitpid(t.0.pid, &mut status, 0);
                libc::exit(0);
            },
            Err(e) => e,
        }
    }
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=STRUCT | NAME=Process | COMPLEXITY=2 | LINES=10

```rust
////////////////////////////////////////////////////////////////////////////////
// Processes
////////////////////////////////////////////////////////////////////////////////

/// The unique id of the process (this should never be negative).
pub struct Process {
    pid: RTP_ID,
    status: Option<ExitStatus>,
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=FUNCTION | NAME=id | COMPLEXITY=40 | LINES=46

```rust
impl Process {
    pub fn id(&self) -> u32 {
        self.pid as u32
    }

    pub fn kill(&self) -> io::Result<()> {
        self.send_signal(libc::SIGKILL)
    }

    pub fn send_signal(&self, signal: i32) -> io::Result<()> {
        // If we've already waited on this process then the pid can be recycled and
        // used for another process, and we probably shouldn't be sending signals to
        // random processes, so return Ok because the process has exited already.
        if self.status.is_some() {
            Ok(())
        } else {
            cvt(unsafe { libc::kill(self.pid, signal) }).map(drop)
        }
    }

    pub fn wait(&mut self) -> io::Result<ExitStatus> {
        use crate::sys::cvt_r;
        if let Some(status) = self.status {
            return Ok(status);
        }
        let mut status = 0 as c_int;
        cvt_r(|| unsafe { libc::waitpid(self.pid, &mut status, 0) })?;
        self.status = Some(ExitStatus::new(status));
        Ok(ExitStatus::new(status))
    }

    pub fn try_wait(&mut self) -> io::Result<Option<ExitStatus>> {
        if let Some(status) = self.status {
            return Ok(Some(status));
        }
        let mut status = 0 as c_int;
        let pid = cvt(unsafe { libc::waitpid(self.pid, &mut status, libc::WNOHANG) })?;
        if pid == 0 {
            Ok(None)
        } else {
            self.status = Some(ExitStatus::new(status));
            Ok(Some(ExitStatus::new(status)))
        }
    }
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=ExitStatus(c_int); | COMPLEXITY=32 | LINES=52

```rust
/// Unix exit statuses
#[derive(PartialEq, Eq, Clone, Copy, Debug, Default)]
pub struct ExitStatus(c_int);

impl ExitStatus {
    pub fn new(status: c_int) -> ExitStatus {
        ExitStatus(status)
    }

    fn exited(&self) -> bool {
        libc::WIFEXITED(self.0)
    }

    pub fn exit_ok(&self) -> Result<(), ExitStatusError> {
        // This assumes that WIFEXITED(status) && WEXITSTATUS==0 corresponds to status==0. This is
        // true on all actual versions of Unix, is widely assumed, and is specified in SuS
        // https://pubs.opengroup.org/onlinepubs/9699919799/functions/wait.html. If it is not
        // true for a platform pretending to be Unix, the tests (our doctests, and also
        // unix/tests.rs) will spot it. `ExitStatusError::code` assumes this too.
        match NonZero::try_from(self.0) {
            Ok(failure) => Err(ExitStatusError(failure)),
            Err(_) => Ok(()),
        }
    }

    pub fn code(&self) -> Option<i32> {
        if self.exited() { Some(libc::WEXITSTATUS(self.0)) } else { None }
    }

    pub fn signal(&self) -> Option<i32> {
        if !self.exited() { Some(libc::WTERMSIG(self.0)) } else { None }
    }

    pub fn core_dumped(&self) -> bool {
        // This method is not yet properly implemented on VxWorks
        false
    }

    pub fn stopped_signal(&self) -> Option<i32> {
        if libc::WIFSTOPPED(self.0) { Some(libc::WSTOPSIG(self.0)) } else { None }
    }

    pub fn continued(&self) -> bool {
        // This method is not yet properly implemented on VxWorks
        false
    }

    pub fn into_raw(&self) -> c_int {
        self.0
    }
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=FUNCTION | NAME=from | COMPLEXITY=5 | LINES=7

```rust
/// Converts a raw `c_int` to a type-safe `ExitStatus` by wrapping it without copying.
impl From<c_int> for ExitStatus {
    fn from(a: c_int) -> ExitStatus {
        ExitStatus(a)
    }
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=11 | LINES=11

```rust
impl fmt::Display for ExitStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(code) = self.code() {
            write!(f, "exit code: {code}")
        } else {
            let signal = self.signal().unwrap();
            write!(f, "signal: {signal}")
        }
    }
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=FUNCTION | NAME=ExitStatusError(NonZero | COMPLEXITY=5 | LINES=9

```rust
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct ExitStatusError(NonZero<c_int>);

impl Into<ExitStatus> for ExitStatusError {
    fn into(self) -> ExitStatus {
        ExitStatus(self.0.into())
    }
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=FUNCTION | NAME=code | COMPLEXITY=3 | LINES=6

```rust
impl ExitStatusError {
    pub fn code(self) -> Option<NonZero<i32>> {
        ExitStatus(self.0.into()).code().map(|st| st.try_into().unwrap())
    }
}
```

---
*Generated by AST tracing system*
