# AST Trace: ../rust/library/std/src/sys/pal/wasip1/os.rs

Generated 23 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3

```rust
#![forbid(unsafe_op_in_unsafe_fn)]

use crate::ffi::{CStr, OsStr, OsString};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3

```rust
use crate::marker::PhantomData;
use crate::os::wasi::prelude::*;
use crate::path::{self, PathBuf};
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3

```rust
use crate::sys::common::small_c_string::run_path_with_cstr;
use crate::sys::unsupported;
use crate::{fmt, io, str};
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=FUNCTION | NAME=getcwd | COMPLEXITY=7 | LINES=11

```rust
// Add a few symbols not in upstream `libc` just yet.
pub mod libc {
    pub use libc::*;

    unsafe extern "C" {
        pub fn getcwd(buf: *mut c_char, size: size_t) -> *mut c_char;
        pub fn chdir(dir: *const c_char) -> c_int;
        pub fn __wasilibc_get_environ() -> *mut *mut c_char;
    }
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=FUNCTION | NAME=errno | COMPLEXITY=12 | LINES=9

```rust
pub fn errno() -> i32 {
    unsafe extern "C" {
        #[thread_local]
        static errno: libc::c_int;
    }

    unsafe { errno as i32 }
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=FUNCTION | NAME=error_string | COMPLEXITY=10 | LINES=12

```rust
pub fn error_string(errno: i32) -> String {
    let mut buf = [0 as libc::c_char; 1024];

    let p = buf.as_mut_ptr();
    unsafe {
        if libc::strerror_r(errno as libc::c_int, p, buf.len()) < 0 {
            panic!("strerror_r failure");
        }
        str::from_utf8(CStr::from_ptr(p).to_bytes()).unwrap().to_owned()
    }
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=getcwd | COMPLEXITY=18 | LINES=26

```rust
pub fn getcwd() -> io::Result<PathBuf> {
    let mut buf = Vec::with_capacity(512);
    loop {
        unsafe {
            let ptr = buf.as_mut_ptr() as *mut libc::c_char;
            if !libc::getcwd(ptr, buf.capacity()).is_null() {
                let len = CStr::from_ptr(buf.as_ptr() as *const libc::c_char).to_bytes().len();
                buf.set_len(len);
                buf.shrink_to_fit();
                return Ok(PathBuf::from(OsString::from_vec(buf)));
            } else {
                let error = io::Error::last_os_error();
                if error.raw_os_error() != Some(libc::ERANGE) {
                    return Err(error);
                }
            }

            // Trigger the internal buffer resizing logic of `Vec` by requiring
            // more space than the current capacity.
            let cap = buf.capacity();
            buf.set_len(cap);
            buf.reserve(1);
        }
    }
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=FUNCTION | NAME=chdir | COMPLEXITY=11 | LINES=8

```rust
pub fn chdir(p: &path::Path) -> io::Result<()> {
    let result = run_path_with_cstr(p, &|p| unsafe { Ok(libc::chdir(p.as_ptr())) })?;
    match result == (0 as libc::c_int) {
        true => Ok(()),
        false => Err(io::Error::last_os_error()),
    }
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=FUNCTION | NAME=SplitPaths | COMPLEXITY=2 | LINES=6

```rust
pub struct SplitPaths<'a>(!, PhantomData<&'a ()>);

pub fn split_paths(_unparsed: &OsStr) -> SplitPaths<'_> {
    panic!("unsupported")
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=FUNCTION | NAME=next | COMPLEXITY=5 | LINES=7

```rust
impl<'a> Iterator for SplitPaths<'a> {
    type Item = PathBuf;
    fn next(&mut self) -> Option<PathBuf> {
        self.0
    }
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=FUNCTION | NAME=JoinPathsError; | COMPLEXITY=2 | LINES=11

```rust
#[derive(Debug)]
pub struct JoinPathsError;

pub fn join_paths<I, T>(_paths: I) -> Result<OsString, JoinPathsError>
where
    I: Iterator<Item = T>,
    T: AsRef<OsStr>,
{
    Err(JoinPathsError)
}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=5 | LINES=6

```rust
impl fmt::Display for JoinPathsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "not supported on wasm yet".fmt(f)
    }
}
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=IMPL | NAME=UNNAMED | COMPLEXITY=4 | LINES=2

```rust
impl crate::error::Error for JoinPathsError {}
```

## Block 14
**Metadata**: AST_ID=14 | TYPE=FUNCTION | NAME=current_exe | COMPLEXITY=2 | LINES=4

```rust
pub fn current_exe() -> io::Result<PathBuf> {
    unsupported()
}
```

## Block 15
**Metadata**: AST_ID=15 | TYPE=FUNCTION | NAME=page_size | COMPLEXITY=7 | LINES=5

```rust
#[allow(dead_code)]
pub fn page_size() -> usize {
    unsafe { libc::sysconf(libc::_SC_PAGESIZE) as usize }
}
```

## Block 16
**Metadata**: AST_ID=16 | TYPE=FUNCTION | NAME=temp_dir | COMPLEXITY=2 | LINES=4

```rust
pub fn temp_dir() -> PathBuf {
    panic!("no filesystem on wasm")
}
```

## Block 17
**Metadata**: AST_ID=17 | TYPE=FUNCTION | NAME=home_dir | COMPLEXITY=2 | LINES=4

```rust
pub fn home_dir() -> Option<PathBuf> {
    None
}
```

## Block 18
**Metadata**: AST_ID=18 | TYPE=FUNCTION | NAME=exit | COMPLEXITY=7 | LINES=4

```rust
pub fn exit(code: i32) -> ! {
    unsafe { libc::exit(code) }
}
```

## Block 19
**Metadata**: AST_ID=19 | TYPE=FUNCTION | NAME=getpid | COMPLEXITY=2 | LINES=4

```rust
pub fn getpid() -> u32 {
    panic!("unsupported");
}
```

## Block 20
**Metadata**: AST_ID=20 | TYPE=FUNCTION | NAME=is_minus_one | COMPLEXITY=2 | LINES=5

```rust
#[doc(hidden)]
pub trait IsMinusOne {
    fn is_minus_one(&self) -> bool;
}
```

## Block 21
**Metadata**: AST_ID=21 | TYPE=FUNCTION | NAME=is_minus_one | COMPLEXITY=11 | LINES=8

```rust
macro_rules! impl_is_minus_one {
    ($($t:ident)*) => ($(impl IsMinusOne for $t {
        fn is_minus_one(&self) -> bool {
            *self == -1
        }
    })*)
}
```

## Block 22
**Metadata**: AST_ID=22 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
impl_is_minus_one! { i8 i16 i32 i64 isize }
```

## Block 23
**Metadata**: AST_ID=23 | TYPE=FUNCTION | NAME=cvt | COMPLEXITY=6 | LINES=4

```rust
pub fn cvt<T: IsMinusOne>(t: T) -> io::Result<T> {
    if t.is_minus_one() { Err(io::Error::last_os_error()) } else { Ok(t) }
}
```

---
*Generated by AST tracing system*
