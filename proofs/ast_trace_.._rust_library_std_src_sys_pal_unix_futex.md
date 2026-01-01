# AST Trace: ../rust/library/std/src/sys/pal/unix/futex.rs

Generated 18 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=FUNCTION | NAME=futex_wait | COMPLEXITY=41 | LINES=95

```rust
#![cfg(any(
    target_os = "linux",
    target_os = "android",
    all(target_os = "emscripten", target_feature = "atomics"),
    target_os = "freebsd",
    target_os = "openbsd",
    target_os = "dragonfly",
    target_os = "fuchsia",
))]

use crate::sync::atomic::Atomic;
use crate::time::Duration;

/// An atomic for use as a futex that is at least 32-bits but may be larger
pub type Futex = Atomic<Primitive>;
/// Must be the underlying type of Futex
pub type Primitive = u32;

/// An atomic for use as a futex that is at least 8-bits but may be larger.
pub type SmallFutex = Atomic<SmallPrimitive>;
/// Must be the underlying type of SmallFutex
pub type SmallPrimitive = u32;

/// Waits for a `futex_wake` operation to wake us.
///
/// Returns directly if the futex doesn't hold the expected value.
///
/// Returns false on timeout, and true in all other cases.
#[cfg(any(target_os = "linux", target_os = "android", target_os = "freebsd"))]
pub fn futex_wait(futex: &Atomic<u32>, expected: u32, timeout: Option<Duration>) -> bool {
    use super::time::Timespec;
    use crate::ptr::null;
    use crate::sync::atomic::Ordering::Relaxed;

    // Calculate the timeout as an absolute timespec.
    //
    // Overflows are rounded up to an infinite timeout (None).
    let timespec = timeout
        .and_then(|d| Timespec::now(libc::CLOCK_MONOTONIC).checked_add_duration(&d))
        .and_then(|t| t.to_timespec());

    loop {
        // No need to wait if the value already changed.
        if futex.load(Relaxed) != expected {
            return true;
        }

        let r = unsafe {
            cfg_select! {
                target_os = "freebsd" => {
                    // FreeBSD doesn't have futex(), but it has
                    // _umtx_op(UMTX_OP_WAIT_UINT_PRIVATE), which is nearly
                    // identical. It supports absolute timeouts through a flag
                    // in the _umtx_time struct.
                    let umtx_timeout = timespec.map(|t| libc::_umtx_time {
                        _timeout: t,
                        _flags: libc::UMTX_ABSTIME,
                        _clockid: libc::CLOCK_MONOTONIC as u32,
                    });
                    let umtx_timeout_ptr = umtx_timeout.as_ref().map_or(null(), |t| t as *const _);
                    let umtx_timeout_size = umtx_timeout.as_ref().map_or(0, |t| size_of_val(t));
                    libc::_umtx_op(
                        futex as *const Atomic<u32> as *mut _,
                        libc::UMTX_OP_WAIT_UINT_PRIVATE,
                        expected as libc::c_ulong,
                        crate::ptr::without_provenance_mut(umtx_timeout_size),
                        umtx_timeout_ptr as *mut _,
                    )
                }
                any(target_os = "linux", target_os = "android") => {
                    // Use FUTEX_WAIT_BITSET rather than FUTEX_WAIT to be able to give an
                    // absolute time rather than a relative time.
                    libc::syscall(
                        libc::SYS_futex,
                        futex as *const Atomic<u32>,
                        libc::FUTEX_WAIT_BITSET | libc::FUTEX_PRIVATE_FLAG,
                        expected,
                        timespec.as_ref().map_or(null(), |t| t as *const libc::timespec),
                        null::<u32>(), // This argument is unused for FUTEX_WAIT_BITSET.
                        !0u32,         // A full bitmask, to make it behave like a regular FUTEX_WAIT.
                    )
                }
                _ => {
                    compile_error!("unknown target_os");
                }
            }
        };

        match (r < 0).then(super::os::errno) {
            Some(libc::ETIMEDOUT) => return false,
            Some(libc::EINTR) => continue,
            _ => return true,
        }
    }
}
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=FUNCTION | NAME=futex_wake | COMPLEXITY=12 | LINES=13

```rust
/// Wakes up one thread that's blocked on `futex_wait` on this futex.
///
/// Returns true if this actually woke up such a thread,
/// or false if no thread was waiting on this futex.
///
/// On some platforms, this always returns false.
#[cfg(any(target_os = "linux", target_os = "android"))]
pub fn futex_wake(futex: &Atomic<u32>) -> bool {
    let ptr = futex as *const Atomic<u32>;
    let op = libc::FUTEX_WAKE | libc::FUTEX_PRIVATE_FLAG;
    unsafe { libc::syscall(libc::SYS_futex, ptr, op, 1) > 0 }
}
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=FUNCTION | NAME=futex_wake_all | COMPLEXITY=7 | LINES=10

```rust
/// Wakes up all threads that are waiting on `futex_wait` on this futex.
#[cfg(any(target_os = "linux", target_os = "android"))]
pub fn futex_wake_all(futex: &Atomic<u32>) {
    let ptr = futex as *const Atomic<u32>;
    let op = libc::FUTEX_WAKE | libc::FUTEX_PRIVATE_FLAG;
    unsafe {
        libc::syscall(libc::SYS_futex, ptr, op, i32::MAX);
    }
}
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=FUNCTION | NAME=futex_wake | COMPLEXITY=7 | LINES=16

```rust
// FreeBSD doesn't tell us how many threads are woken up, so this always returns false.
#[cfg(target_os = "freebsd")]
pub fn futex_wake(futex: &Atomic<u32>) -> bool {
    use crate::ptr::null_mut;
    unsafe {
        libc::_umtx_op(
            futex as *const Atomic<u32> as *mut _,
            libc::UMTX_OP_WAKE_PRIVATE,
            1,
            null_mut(),
            null_mut(),
        )
    };
    false
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=FUNCTION | NAME=futex_wake_all | COMPLEXITY=7 | LINES=14

```rust
#[cfg(target_os = "freebsd")]
pub fn futex_wake_all(futex: &Atomic<u32>) {
    use crate::ptr::null_mut;
    unsafe {
        libc::_umtx_op(
            futex as *const Atomic<u32> as *mut _,
            libc::UMTX_OP_WAKE_PRIVATE,
            i32::MAX as libc::c_ulong,
            null_mut(),
            null_mut(),
        )
    };
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=FUNCTION | NAME=futex_wait | COMPLEXITY=9 | LINES=23

```rust
#[cfg(target_os = "openbsd")]
pub fn futex_wait(futex: &Atomic<u32>, expected: u32, timeout: Option<Duration>) -> bool {
    use super::time::Timespec;
    use crate::ptr::{null, null_mut};

    // Overflows are rounded up to an infinite timeout (None).
    let timespec = timeout
        .and_then(|d| Timespec::zero().checked_add_duration(&d))
        .and_then(|t| t.to_timespec());

    let r = unsafe {
        libc::futex(
            futex as *const Atomic<u32> as *mut u32,
            libc::FUTEX_WAIT,
            expected as i32,
            timespec.as_ref().map_or(null(), |t| t as *const libc::timespec),
            null_mut(),
        )
    };

    r == 0 || super::os::errno() != libc::ETIMEDOUT
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=futex_wake | COMPLEXITY=8 | LINES=14

```rust
#[cfg(target_os = "openbsd")]
pub fn futex_wake(futex: &Atomic<u32>) -> bool {
    use crate::ptr::{null, null_mut};
    unsafe {
        libc::futex(
            futex as *const Atomic<u32> as *mut u32,
            libc::FUTEX_WAKE,
            1,
            null(),
            null_mut(),
        ) > 0
    }
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=FUNCTION | NAME=futex_wake_all | COMPLEXITY=8 | LINES=14

```rust
#[cfg(target_os = "openbsd")]
pub fn futex_wake_all(futex: &Atomic<u32>) {
    use crate::ptr::{null, null_mut};
    unsafe {
        libc::futex(
            futex as *const Atomic<u32> as *mut u32,
            libc::FUTEX_WAKE,
            i32::MAX,
            null(),
            null_mut(),
        );
    }
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=FUNCTION | NAME=futex_wait | COMPLEXITY=8 | LINES=15

```rust
#[cfg(target_os = "dragonfly")]
pub fn futex_wait(futex: &Atomic<u32>, expected: u32, timeout: Option<Duration>) -> bool {
    // A timeout of 0 means infinite.
    // We round smaller timeouts up to 1 millisecond.
    // Overflows are rounded up to an infinite timeout.
    let timeout_ms =
        timeout.and_then(|d| Some(i32::try_from(d.as_millis()).ok()?.max(1))).unwrap_or(0);

    let r = unsafe {
        libc::umtx_sleep(futex as *const Atomic<u32> as *const i32, expected as i32, timeout_ms)
    };

    r == 0 || super::os::errno() != libc::ETIMEDOUT
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=FUNCTION | NAME=futex_wake | COMPLEXITY=7 | LINES=7

```rust
// DragonflyBSD doesn't tell us how many threads are woken up, so this always returns false.
#[cfg(target_os = "dragonfly")]
pub fn futex_wake(futex: &Atomic<u32>) -> bool {
    unsafe { libc::umtx_wakeup(futex as *const Atomic<u32> as *const i32, 1) };
    false
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=FUNCTION | NAME=futex_wake_all | COMPLEXITY=7 | LINES=5

```rust
#[cfg(target_os = "dragonfly")]
pub fn futex_wake_all(futex: &Atomic<u32>) {
    unsafe { libc::umtx_wakeup(futex as *const Atomic<u32> as *const i32, i32::MAX) };
}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=FUNCTION | NAME=emscripten_futex_wake | COMPLEXITY=6 | LINES=10

```rust
#[cfg(target_os = "emscripten")]
unsafe extern "C" {
    fn emscripten_futex_wake(addr: *const Atomic<u32>, count: libc::c_int) -> libc::c_int;
    fn emscripten_futex_wait(
        addr: *const Atomic<u32>,
        val: libc::c_uint,
        max_wait_ms: libc::c_double,
    ) -> libc::c_int;
}
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=FUNCTION | NAME=futex_wait | COMPLEXITY=7 | LINES=11

```rust
#[cfg(target_os = "emscripten")]
pub fn futex_wait(futex: &Atomic<u32>, expected: u32, timeout: Option<Duration>) -> bool {
    unsafe {
        emscripten_futex_wait(
            futex,
            expected,
            timeout.map_or(f64::INFINITY, |d| d.as_secs_f64() * 1000.0),
        ) != -libc::ETIMEDOUT
    }
}
```

## Block 14
**Metadata**: AST_ID=14 | TYPE=FUNCTION | NAME=futex_wake | COMPLEXITY=7 | LINES=5

```rust
#[cfg(target_os = "emscripten")]
pub fn futex_wake(futex: &Atomic<u32>) -> bool {
    unsafe { emscripten_futex_wake(futex, 1) > 0 }
}
```

## Block 15
**Metadata**: AST_ID=15 | TYPE=FUNCTION | NAME=futex_wake_all | COMPLEXITY=7 | LINES=5

```rust
#[cfg(target_os = "emscripten")]
pub fn futex_wake_all(futex: &Atomic<u32>) {
    unsafe { emscripten_futex_wake(futex, i32::MAX) };
}
```

## Block 16
**Metadata**: AST_ID=16 | TYPE=FUNCTION | NAME=futex_wait | COMPLEXITY=10 | LINES=15

```rust
#[cfg(target_os = "fuchsia")]
pub fn futex_wait(futex: &Atomic<u32>, expected: u32, timeout: Option<Duration>) -> bool {
    use super::fuchsia::*;

    // Sleep forever if the timeout is longer than fits in a i64.
    let deadline = timeout
        .and_then(|d| i64::try_from(d.as_nanos()).ok()?.checked_add(zx_clock_get_monotonic()))
        .unwrap_or(ZX_TIME_INFINITE);

    unsafe {
        zx_futex_wait(futex, zx_futex_t::new(expected), ZX_HANDLE_INVALID, deadline)
            != ZX_ERR_TIMED_OUT
    }
}
```

## Block 17
**Metadata**: AST_ID=17 | TYPE=FUNCTION | NAME=futex_wake | COMPLEXITY=7 | LINES=7

```rust
// Fuchsia doesn't tell us how many threads are woken up, so this always returns false.
#[cfg(target_os = "fuchsia")]
pub fn futex_wake(futex: &Atomic<u32>) -> bool {
    unsafe { super::fuchsia::zx_futex_wake(futex, 1) };
    false
}
```

## Block 18
**Metadata**: AST_ID=18 | TYPE=FUNCTION | NAME=futex_wake_all | COMPLEXITY=7 | LINES=5

```rust
#[cfg(target_os = "fuchsia")]
pub fn futex_wake_all(futex: &Atomic<u32>) {
    unsafe { super::fuchsia::zx_futex_wake(futex, u32::MAX) };
}
```

---
*Generated by AST tracing system*
