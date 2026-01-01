# AST Trace: ../rust/library/std/src/sys/process/unix/common.rs

Generated 33 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=4

```rust
#[cfg(all(test, not(target_os = "emscripten")))]
mod tests;

use libc::{EXIT_FAILURE, EXIT_SUCCESS, c_int, gid_t, pid_t, uid_t};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=5

```rust
pub use self::cstring_array::CStringArray;
use self::cstring_array::CStringIter;
use crate::collections::BTreeMap;
use crate::ffi::{CStr, CString, OsStr, OsString};
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
use crate::os::unix::prelude::*;
use crate::path::Path;
use crate::sys::fd::FileDesc;
use crate::sys::fs::File;
#[cfg(not(target_os = "fuchsia"))]
use crate::sys::fs::OpenOptions;
use crate::sys::pipe::{self, AnonPipe};
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use crate::sys::process::env::{CommandEnv, CommandEnvs};
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use crate::sys_common::{FromInner, IntoInner};
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use crate::{fmt, io};
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=MODULE | NAME=UNNAMED | COMPLEXITY=5 | LINES=14

```rust
mod cstring_array;

cfg_select! {
    target_os = "fuchsia" => {
        // fuchsia doesn't have /dev/null
    }
    target_os = "vxworks" => {
        const DEV_NULL: &CStr = c"/null";
    }
    _ => {
        const DEV_NULL: &CStr = c"/dev/null";
    }
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=24 | LINES=46

```rust
// Android with api less than 21 define sig* functions inline, so it is not
// available for dynamic link. Implementing sigemptyset and sigaddset allow us
// to support older Android version (independent of libc version).
// The following implementations are based on
// https://github.com/aosp-mirror/platform_bionic/blob/ad8dcd6023294b646e5a8288c0ed431b0845da49/libc/include/android/legacy_signal_inlines.h
cfg_select! {
    target_os = "android" => {
        #[allow(dead_code)]
        pub unsafe fn sigemptyset(set: *mut libc::sigset_t) -> libc::c_int {
            set.write_bytes(0u8, 1);
            return 0;
        }

        #[allow(dead_code)]
        pub unsafe fn sigaddset(set: *mut libc::sigset_t, signum: libc::c_int) -> libc::c_int {
            use crate::slice;
            use libc::{c_ulong, sigset_t};

            // The implementations from bionic (android libc) type pun `sigset_t` as an
            // array of `c_ulong`. This works, but lets add a smoke check to make sure
            // that doesn't change.
            const _: () = assert!(
                align_of::<c_ulong>() == align_of::<sigset_t>()
                    && (size_of::<sigset_t>() % size_of::<c_ulong>()) == 0
            );

            let bit = (signum - 1) as usize;
            if set.is_null() || bit >= (8 * size_of::<sigset_t>()) {
                crate::sys::pal::os::set_errno(libc::EINVAL);
                return -1;
            }
            let raw = slice::from_raw_parts_mut(
                set as *mut c_ulong,
                size_of::<sigset_t>() / size_of::<c_ulong>(),
            );
            const LONG_BIT: usize = size_of::<c_ulong>() * 8;
            raw[bit / LONG_BIT] |= 1 << (bit % LONG_BIT);
            return 0;
        }
    }
    _ => {
        #[allow(unused_imports)]
        pub use libc::{sigemptyset, sigaddset};
    }
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=STRUCT | NAME=Command | COMPLEXITY=3 | LINES=26

```rust
////////////////////////////////////////////////////////////////////////////////
// Command
////////////////////////////////////////////////////////////////////////////////

pub struct Command {
    program: CString,
    args: CStringArray,
    env: CommandEnv,

    program_kind: ProgramKind,
    cwd: Option<CString>,
    chroot: Option<CString>,
    uid: Option<uid_t>,
    gid: Option<gid_t>,
    saw_nul: bool,
    closures: Vec<Box<dyn FnMut() -> io::Result<()> + Send + Sync>>,
    groups: Option<Box<[gid_t]>>,
    stdin: Option<Stdio>,
    stdout: Option<Stdio>,
    stderr: Option<Stdio>,
    #[cfg(target_os = "linux")]
    create_pidfd: bool,
    pgroup: Option<pid_t>,
    setsid: bool,
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=STRUCT | NAME=StdioPipes | COMPLEXITY=4 | LINES=8

```rust
// passed back to std::process with the pipes connected to the child, if any
// were requested
pub struct StdioPipes {
    pub stdin: Option<AnonPipe>,
    pub stdout: Option<AnonPipe>,
    pub stderr: Option<AnonPipe>,
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=STRUCT | NAME=ChildPipes | COMPLEXITY=2 | LINES=9

```rust
// passed to do_exec() with configuration of what the child stdio should look
// like
#[cfg_attr(target_os = "vita", allow(dead_code))]
pub struct ChildPipes {
    pub stdin: ChildStdio,
    pub stdout: ChildStdio,
    pub stderr: ChildStdio,
}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=11

```rust
pub enum ChildStdio {
    Inherit,
    Explicit(c_int),
    Owned(FileDesc),

    // On Fuchsia, null stdio is the default, so we simply don't specify
    // any actions at the time of spawning.
    #[cfg(target_os = "fuchsia")]
    Null,
}
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
#[derive(Debug)]
pub enum Stdio {
    Inherit,
    Null,
    MakePipe,
    Fd(FileDesc),
    StaticFd(BorrowedFd<'static>),
}
```

## Block 14
**Metadata**: AST_ID=14 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=10

```rust
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ProgramKind {
    /// A program that would be looked up on the PATH (e.g. `ls`)
    PathLookup,
    /// A relative path (e.g. `my-dir/foo`, `../foo`, `./foo`)
    Relative,
    /// An absolute path.
    Absolute,
}
```

## Block 15
**Metadata**: AST_ID=15 | TYPE=FUNCTION | NAME=new | COMPLEXITY=10 | LINES=13

```rust
impl ProgramKind {
    fn new(program: &OsStr) -> Self {
        if program.as_encoded_bytes().starts_with(b"/") {
            Self::Absolute
        } else if program.as_encoded_bytes().contains(&b'/') {
            // If the program has more than one component in it, it is a relative path.
            Self::Relative
        } else {
            Self::PathLookup
        }
    }
}
```

## Block 16
**Metadata**: AST_ID=16 | TYPE=FUNCTION | NAME=new | COMPLEXITY=66 | LINES=205

```rust
impl Command {
    pub fn new(program: &OsStr) -> Command {
        let mut saw_nul = false;
        let program_kind = ProgramKind::new(program.as_ref());
        let program = os2c(program, &mut saw_nul);
        let mut args = CStringArray::with_capacity(1);
        args.push(program.clone());
        Command {
            program,
            args,
            env: Default::default(),
            program_kind,
            cwd: None,
            chroot: None,
            uid: None,
            gid: None,
            saw_nul,
            closures: Vec::new(),
            groups: None,
            stdin: None,
            stdout: None,
            stderr: None,
            #[cfg(target_os = "linux")]
            create_pidfd: false,
            pgroup: None,
            setsid: false,
        }
    }

    pub fn set_arg_0(&mut self, arg: &OsStr) {
        // Set a new arg0
        let arg = os2c(arg, &mut self.saw_nul);
        self.args.write(0, arg);
    }

    pub fn arg(&mut self, arg: &OsStr) {
        let arg = os2c(arg, &mut self.saw_nul);
        self.args.push(arg);
    }

    pub fn cwd(&mut self, dir: &OsStr) {
        self.cwd = Some(os2c(dir, &mut self.saw_nul));
    }
    pub fn uid(&mut self, id: uid_t) {
        self.uid = Some(id);
    }
    pub fn gid(&mut self, id: gid_t) {
        self.gid = Some(id);
    }
    pub fn groups(&mut self, groups: &[gid_t]) {
        self.groups = Some(Box::from(groups));
    }
    pub fn pgroup(&mut self, pgroup: pid_t) {
        self.pgroup = Some(pgroup);
    }
    pub fn chroot(&mut self, dir: &Path) {
        self.chroot = Some(os2c(dir.as_os_str(), &mut self.saw_nul));
        if self.cwd.is_none() {
            self.cwd(&OsStr::new("/"));
        }
    }
    pub fn setsid(&mut self, setsid: bool) {
        self.setsid = setsid;
    }

    #[cfg(target_os = "linux")]
    pub fn create_pidfd(&mut self, val: bool) {
        self.create_pidfd = val;
    }

    #[cfg(not(target_os = "linux"))]
    #[allow(dead_code)]
    pub fn get_create_pidfd(&self) -> bool {
        false
    }

    #[cfg(target_os = "linux")]
    pub fn get_create_pidfd(&self) -> bool {
        self.create_pidfd
    }

    pub fn saw_nul(&self) -> bool {
        self.saw_nul
    }

    pub fn get_program(&self) -> &OsStr {
        OsStr::from_bytes(self.program.as_bytes())
    }

    #[allow(dead_code)]
    pub fn get_program_kind(&self) -> ProgramKind {
        self.program_kind
    }

    pub fn get_args(&self) -> CommandArgs<'_> {
        let mut iter = self.args.iter();
        // argv[0] contains the program name, but we are only interested in the
        // arguments so skip it.
        iter.next();
        CommandArgs { iter }
    }

    pub fn get_envs(&self) -> CommandEnvs<'_> {
        self.env.iter()
    }

    pub fn get_current_dir(&self) -> Option<&Path> {
        self.cwd.as_ref().map(|cs| Path::new(OsStr::from_bytes(cs.as_bytes())))
    }

    pub fn get_argv(&self) -> &CStringArray {
        &self.args
    }

    pub fn get_program_cstr(&self) -> &CStr {
        &self.program
    }

    #[allow(dead_code)]
    pub fn get_cwd(&self) -> Option<&CStr> {
        self.cwd.as_deref()
    }
    #[allow(dead_code)]
    pub fn get_uid(&self) -> Option<uid_t> {
        self.uid
    }
    #[allow(dead_code)]
    pub fn get_gid(&self) -> Option<gid_t> {
        self.gid
    }
    #[allow(dead_code)]
    pub fn get_groups(&self) -> Option<&[gid_t]> {
        self.groups.as_deref()
    }
    #[allow(dead_code)]
    pub fn get_pgroup(&self) -> Option<pid_t> {
        self.pgroup
    }
    #[allow(dead_code)]
    pub fn get_chroot(&self) -> Option<&CStr> {
        self.chroot.as_deref()
    }
    #[allow(dead_code)]
    pub fn get_setsid(&self) -> bool {
        self.setsid
    }

    pub fn get_closures(&mut self) -> &mut Vec<Box<dyn FnMut() -> io::Result<()> + Send + Sync>> {
        &mut self.closures
    }

    pub unsafe fn pre_exec(&mut self, f: Box<dyn FnMut() -> io::Result<()> + Send + Sync>) {
        self.closures.push(f);
    }

    pub fn stdin(&mut self, stdin: Stdio) {
        self.stdin = Some(stdin);
    }

    pub fn stdout(&mut self, stdout: Stdio) {
        self.stdout = Some(stdout);
    }

    pub fn stderr(&mut self, stderr: Stdio) {
        self.stderr = Some(stderr);
    }

    pub fn env_mut(&mut self) -> &mut CommandEnv {
        &mut self.env
    }

    pub fn capture_env(&mut self) -> Option<CStringArray> {
        let maybe_env = self.env.capture_if_changed();
        maybe_env.map(|env| construct_envp(env, &mut self.saw_nul))
    }

    #[allow(dead_code)]
    pub fn env_saw_path(&self) -> bool {
        self.env.have_changed_path()
    }

    #[allow(dead_code)]
    pub fn program_is_path(&self) -> bool {
        self.program.to_bytes().contains(&b'/')
    }

    pub fn setup_io(
        &self,
        default: Stdio,
        needs_stdin: bool,
    ) -> io::Result<(StdioPipes, ChildPipes)> {
        let null = Stdio::Null;
        let default_stdin = if needs_stdin { &default } else { &null };
        let stdin = self.stdin.as_ref().unwrap_or(default_stdin);
        let stdout = self.stdout.as_ref().unwrap_or(&default);
        let stderr = self.stderr.as_ref().unwrap_or(&default);
        let (their_stdin, our_stdin) = stdin.to_child_stdio(true)?;
        let (their_stdout, our_stdout) = stdout.to_child_stdio(false)?;
        let (their_stderr, our_stderr) = stderr.to_child_stdio(false)?;
        let ours = StdioPipes { stdin: our_stdin, stdout: our_stdout, stderr: our_stderr };
        let theirs = ChildPipes { stdin: their_stdin, stdout: their_stdout, stderr: their_stderr };
        Ok((ours, theirs))
    }
}
```

## Block 17
**Metadata**: AST_ID=17 | TYPE=FUNCTION | NAME=os2c | COMPLEXITY=3 | LINES=7

```rust
fn os2c(s: &OsStr, saw_nul: &mut bool) -> CString {
    CString::new(s.as_bytes()).unwrap_or_else(|_e| {
        *saw_nul = true;
        c"<string-with-nul>".to_owned()
    })
}
```

## Block 18
**Metadata**: AST_ID=18 | TYPE=FUNCTION | NAME=construct_envp | COMPLEXITY=12 | LINES=19

```rust
fn construct_envp(env: BTreeMap<OsString, OsString>, saw_nul: &mut bool) -> CStringArray {
    let mut result = CStringArray::with_capacity(env.len());
    for (mut k, v) in env {
        // Reserve additional space for '=' and null terminator
        k.reserve_exact(v.len() + 2);
        k.push("=");
        k.push(&v);

        // Add the new entry into the array
        if let Ok(item) = CString::new(k.into_vec()) {
            result.push(item);
        } else {
            *saw_nul = true;
        }
    }

    result
}
```

## Block 19
**Metadata**: AST_ID=19 | TYPE=FUNCTION | NAME=to_child_stdio | COMPLEXITY=22 | LINES=46

```rust
impl Stdio {
    pub fn to_child_stdio(&self, readable: bool) -> io::Result<(ChildStdio, Option<AnonPipe>)> {
        match *self {
            Stdio::Inherit => Ok((ChildStdio::Inherit, None)),

            // Make sure that the source descriptors are not an stdio
            // descriptor, otherwise the order which we set the child's
            // descriptors may blow away a descriptor which we are hoping to
            // save. For example, suppose we want the child's stderr to be the
            // parent's stdout, and the child's stdout to be the parent's
            // stderr. No matter which we dup first, the second will get
            // overwritten prematurely.
            Stdio::Fd(ref fd) => {
                if fd.as_raw_fd() >= 0 && fd.as_raw_fd() <= libc::STDERR_FILENO {
                    Ok((ChildStdio::Owned(fd.duplicate()?), None))
                } else {
                    Ok((ChildStdio::Explicit(fd.as_raw_fd()), None))
                }
            }

            Stdio::StaticFd(fd) => {
                let fd = FileDesc::from_inner(fd.try_clone_to_owned()?);
                Ok((ChildStdio::Owned(fd), None))
            }

            Stdio::MakePipe => {
                let (reader, writer) = pipe::anon_pipe()?;
                let (ours, theirs) = if readable { (writer, reader) } else { (reader, writer) };
                Ok((ChildStdio::Owned(theirs.into_inner()), Some(ours)))
            }

            #[cfg(not(target_os = "fuchsia"))]
            Stdio::Null => {
                let mut opts = OpenOptions::new();
                opts.read(readable);
                opts.write(!readable);
                let fd = File::open_c(DEV_NULL, &opts)?;
                Ok((ChildStdio::Owned(fd.into_inner()), None))
            }

            #[cfg(target_os = "fuchsia")]
            Stdio::Null => Ok((ChildStdio::Null, None)),
        }
    }
}
```

## Block 20
**Metadata**: AST_ID=20 | TYPE=FUNCTION | NAME=from | COMPLEXITY=5 | LINES=6

```rust
impl From<AnonPipe> for Stdio {
    fn from(pipe: AnonPipe) -> Stdio {
        Stdio::Fd(pipe.into_inner())
    }
}
```

## Block 21
**Metadata**: AST_ID=21 | TYPE=FUNCTION | NAME=from | COMPLEXITY=5 | LINES=6

```rust
impl From<FileDesc> for Stdio {
    fn from(fd: FileDesc) -> Stdio {
        Stdio::Fd(fd)
    }
}
```

## Block 22
**Metadata**: AST_ID=22 | TYPE=FUNCTION | NAME=from | COMPLEXITY=5 | LINES=6

```rust
impl From<File> for Stdio {
    fn from(file: File) -> Stdio {
        Stdio::Fd(file.into_inner())
    }
}
```

## Block 23
**Metadata**: AST_ID=23 | TYPE=FUNCTION | NAME=from | COMPLEXITY=19 | LINES=16

```rust
impl From<io::Stdout> for Stdio {
    fn from(_: io::Stdout) -> Stdio {
        // This ought really to be is Stdio::StaticFd(input_argument.as_fd()).
        // But AsFd::as_fd takes its argument by reference, and yields
        // a bounded lifetime, so it's no use here. There is no AsStaticFd.
        //
        // Additionally AsFd is only implemented for the *locked* versions.
        // We don't want to lock them here.  (The implications of not locking
        // are the same as those for process::Stdio::inherit().)
        //
        // Arguably the hypothetical AsStaticFd and AsFd<'static>
        // should be implemented for io::Stdout, not just for StdoutLocked.
        Stdio::StaticFd(unsafe { BorrowedFd::borrow_raw(libc::STDOUT_FILENO) })
    }
}
```

## Block 24
**Metadata**: AST_ID=24 | TYPE=FUNCTION | NAME=from | COMPLEXITY=10 | LINES=6

```rust
impl From<io::Stderr> for Stdio {
    fn from(_: io::Stderr) -> Stdio {
        Stdio::StaticFd(unsafe { BorrowedFd::borrow_raw(libc::STDERR_FILENO) })
    }
}
```

## Block 25
**Metadata**: AST_ID=25 | TYPE=FUNCTION | NAME=fd | COMPLEXITY=7 | LINES=13

```rust
impl ChildStdio {
    pub fn fd(&self) -> Option<c_int> {
        match *self {
            ChildStdio::Inherit => None,
            ChildStdio::Explicit(fd) => Some(fd),
            ChildStdio::Owned(ref fd) => Some(fd.as_raw_fd()),

            #[cfg(target_os = "fuchsia")]
            ChildStdio::Null => None,
        }
    }
}
```

## Block 26
**Metadata**: AST_ID=26 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=79 | LINES=85

```rust
impl fmt::Debug for Command {
    // show all attributes but `self.closures` which does not implement `Debug`
    // and `self.argv` which is not useful for debugging
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            let mut debug_command = f.debug_struct("Command");
            debug_command.field("program", &self.program).field("args", &self.args);
            if !self.env.is_unchanged() {
                debug_command.field("env", &self.env);
            }

            if self.cwd.is_some() {
                debug_command.field("cwd", &self.cwd);
            }
            if self.uid.is_some() {
                debug_command.field("uid", &self.uid);
            }
            if self.gid.is_some() {
                debug_command.field("gid", &self.gid);
            }

            if self.groups.is_some() {
                debug_command.field("groups", &self.groups);
            }

            if self.stdin.is_some() {
                debug_command.field("stdin", &self.stdin);
            }
            if self.stdout.is_some() {
                debug_command.field("stdout", &self.stdout);
            }
            if self.stderr.is_some() {
                debug_command.field("stderr", &self.stderr);
            }
            if self.pgroup.is_some() {
                debug_command.field("pgroup", &self.pgroup);
            }

            #[cfg(target_os = "linux")]
            {
                debug_command.field("create_pidfd", &self.create_pidfd);
            }

            debug_command.finish()
        } else {
            if let Some(ref cwd) = self.cwd {
                write!(f, "cd {cwd:?} && ")?;
            }
            if self.env.does_clear() {
                write!(f, "env -i ")?;
                // Altered env vars will be printed next, that should exactly work as expected.
            } else {
                // Removed env vars need the command to be wrapped in `env`.
                let mut any_removed = false;
                for (key, value_opt) in self.get_envs() {
                    if value_opt.is_none() {
                        if !any_removed {
                            write!(f, "env ")?;
                            any_removed = true;
                        }
                        write!(f, "-u {} ", key.to_string_lossy())?;
                    }
                }
            }
            // Altered env vars can just be added in front of the program.
            for (key, value_opt) in self.get_envs() {
                if let Some(value) = value_opt {
                    write!(f, "{}={value:?} ", key.to_string_lossy())?;
                }
            }

            if *self.program != self.args[0] {
                write!(f, "[{:?}] ", self.program)?;
            }
            write!(f, "{:?}", &self.args[0])?;

            for arg in self.get_args() {
                write!(f, " {:?}", arg)?;
            }

            Ok(())
        }
    }
}
```

## Block 27
**Metadata**: AST_ID=27 | TYPE=FUNCTION | NAME=ExitCode(u8); | COMPLEXITY=5 | LINES=9

```rust
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct ExitCode(u8);

impl fmt::Debug for ExitCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("unix_exit_status").field(&self.0).finish()
    }
}
```

## Block 28
**Metadata**: AST_ID=28 | TYPE=FUNCTION | NAME=as_i32 | COMPLEXITY=3 | LINES=10

```rust
impl ExitCode {
    pub const SUCCESS: ExitCode = ExitCode(EXIT_SUCCESS as _);
    pub const FAILURE: ExitCode = ExitCode(EXIT_FAILURE as _);

    #[inline]
    pub fn as_i32(&self) -> i32 {
        self.0 as i32
    }
}
```

## Block 29
**Metadata**: AST_ID=29 | TYPE=FUNCTION | NAME=from | COMPLEXITY=5 | LINES=6

```rust
impl From<u8> for ExitCode {
    fn from(code: u8) -> Self {
        Self(code)
    }
}
```

## Block 30
**Metadata**: AST_ID=30 | TYPE=STRUCT | NAME=CommandArgs | COMPLEXITY=2 | LINES=4

```rust
pub struct CommandArgs<'a> {
    iter: CStringIter<'a>,
}
```

## Block 31
**Metadata**: AST_ID=31 | TYPE=FUNCTION | NAME=next | COMPLEXITY=6 | LINES=12

```rust
impl<'a> Iterator for CommandArgs<'a> {
    type Item = &'a OsStr;

    fn next(&mut self) -> Option<&'a OsStr> {
        self.iter.next().map(|cs| OsStr::from_bytes(cs.to_bytes()))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
```

## Block 32
**Metadata**: AST_ID=32 | TYPE=FUNCTION | NAME=len | COMPLEXITY=6 | LINES=10

```rust
impl<'a> ExactSizeIterator for CommandArgs<'a> {
    fn len(&self) -> usize {
        self.iter.len()
    }

    fn is_empty(&self) -> bool {
        self.iter.is_empty()
    }
}
```

## Block 33
**Metadata**: AST_ID=33 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=5 | LINES=6

```rust
impl<'a> fmt::Debug for CommandArgs<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter.clone()).finish()
    }
}
```

---
*Generated by AST tracing system*
