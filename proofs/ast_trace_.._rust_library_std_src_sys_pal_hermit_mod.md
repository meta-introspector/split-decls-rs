# AST Trace: ../rust/library/std/src/sys/pal/hermit/mod.rs

Generated 14 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=FUNCTION | NAME=unsupported | COMPLEXITY=14 | LINES=32

```rust
//! System bindings for HermitCore
//!
//! This module contains the facade (aka platform-specific) implementations of
//! OS level functionality for HermitCore.
//!
//! This is all super highly experimental and not actually intended for
//! wide/production use yet, it's still all in the experimental category. This
//! will likely change over time.
//!
//! Currently all functions here are basically stubs that immediately return
//! errors. The hope is that with a portability lint we can turn actually just
//! remove all this and just omit parts of the standard library if we're
//! compiling for wasm. That way it's a compile time error for something that's
//! guaranteed to be a runtime error!

#![deny(unsafe_op_in_unsafe_fn)]
#![allow(missing_docs, nonstandard_style)]

use crate::io::ErrorKind;
use crate::os::hermit::hermit_abi;
use crate::os::raw::c_char;
use crate::sys::env;

pub mod futex;
pub mod os;
#[path = "../unsupported/pipe.rs"]
pub mod pipe;
pub mod time;

pub fn unsupported<T>() -> crate::io::Result<T> {
    Err(unsupported_err())
}
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=FUNCTION | NAME=unsupported_err | COMPLEXITY=2 | LINES=7

```rust
pub fn unsupported_err() -> crate::io::Error {
    crate::io::const_error!(
        crate::io::ErrorKind::Unsupported,
        "operation not supported on HermitCore yet",
    )
}
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=FUNCTION | NAME=abort_internal | COMPLEXITY=7 | LINES=4

```rust
pub fn abort_internal() -> ! {
    unsafe { hermit_abi::abort() }
}
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=13 | LINES=8

```rust
// SAFETY: must be called only once during runtime initialization.
// NOTE: this is not guaranteed to run, for example when Rust code is called externally.
pub unsafe fn init(argc: isize, argv: *const *const u8, _sigpipe: u8) {
    unsafe {
        crate::sys::args::init(argc, argv);
    }
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=8 | LINES=4

```rust
// SAFETY: must be called only once during runtime cleanup.
// NOTE: this is not guaranteed to run, for example when the program aborts.
pub unsafe fn cleanup() {}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=FUNCTION | NAME=main | COMPLEXITY=27 | LINES=26

```rust
#[cfg(not(test))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn runtime_entry(
    argc: i32,
    argv: *const *const c_char,
    env: *const *const c_char,
) -> ! {
    unsafe extern "C" {
        fn main(argc: isize, argv: *const *const c_char) -> i32;
    }

    // initialize environment
    env::init(env);

    let result = unsafe { main(argc as isize, argv) };

    unsafe {
        crate::sys::thread_local::destructors::run();
    }
    crate::rt::thread_cleanup();

    unsafe {
        hermit_abi::exit(result);
    }
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=2 | LINES=5

```rust
#[inline]
pub(crate) fn is_interrupted(errno: i32) -> bool {
    errno == hermit_abi::errno::EINTR
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=FUNCTION | NAME=decode_error_kind | COMPLEXITY=8 | LINES=21

```rust
pub fn decode_error_kind(errno: i32) -> ErrorKind {
    match errno {
        hermit_abi::errno::EACCES => ErrorKind::PermissionDenied,
        hermit_abi::errno::EADDRINUSE => ErrorKind::AddrInUse,
        hermit_abi::errno::EADDRNOTAVAIL => ErrorKind::AddrNotAvailable,
        hermit_abi::errno::EAGAIN => ErrorKind::WouldBlock,
        hermit_abi::errno::ECONNABORTED => ErrorKind::ConnectionAborted,
        hermit_abi::errno::ECONNREFUSED => ErrorKind::ConnectionRefused,
        hermit_abi::errno::ECONNRESET => ErrorKind::ConnectionReset,
        hermit_abi::errno::EEXIST => ErrorKind::AlreadyExists,
        hermit_abi::errno::EINTR => ErrorKind::Interrupted,
        hermit_abi::errno::EINVAL => ErrorKind::InvalidInput,
        hermit_abi::errno::ENOENT => ErrorKind::NotFound,
        hermit_abi::errno::ENOTCONN => ErrorKind::NotConnected,
        hermit_abi::errno::EPERM => ErrorKind::PermissionDenied,
        hermit_abi::errno::EPIPE => ErrorKind::BrokenPipe,
        hermit_abi::errno::ETIMEDOUT => ErrorKind::TimedOut,
        _ => ErrorKind::Uncategorized,
    }
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=FUNCTION | NAME=is_negative | COMPLEXITY=2 | LINES=6

```rust
#[doc(hidden)]
pub trait IsNegative {
    fn is_negative(&self) -> bool;
    fn negate(&self) -> i32;
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=FUNCTION | NAME=is_negative | COMPLEXITY=12 | LINES=12

```rust
macro_rules! impl_is_negative {
    ($($t:ident)*) => ($(impl IsNegative for $t {
        fn is_negative(&self) -> bool {
            *self < 0
        }

        fn negate(&self) -> i32 {
            i32::try_from(-(*self)).unwrap()
        }
    })*)
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=FUNCTION | NAME=is_negative | COMPLEXITY=6 | LINES=10

```rust
impl IsNegative for i32 {
    fn is_negative(&self) -> bool {
        *self < 0
    }

    fn negate(&self) -> i32 {
        -(*self)
    }
}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
impl_is_negative! { i8 i16 i64 isize }
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=FUNCTION | NAME=cvt | COMPLEXITY=6 | LINES=9

```rust
pub fn cvt<T: IsNegative>(t: T) -> crate::io::Result<T> {
    if t.is_negative() {
        let e = decode_error_kind(t.negate());
        Err(crate::io::Error::from(e))
    } else {
        Ok(t)
    }
}
```

## Block 14
**Metadata**: AST_ID=14 | TYPE=FUNCTION | NAME=cvt_r | COMPLEXITY=12 | LINES=13

```rust
pub fn cvt_r<T, F>(mut f: F) -> crate::io::Result<T>
where
    T: IsNegative,
    F: FnMut() -> T,
{
    loop {
        match cvt(f()) {
            Err(ref e) if e.is_interrupted() => {}
            other => return other,
        }
    }
}
```

---
*Generated by AST tracing system*
