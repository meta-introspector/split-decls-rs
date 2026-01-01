# AST Trace: ../rust/library/std/src/io/stdio/tests.rs

Generated 19 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
use super::*;
use crate::panic::{RefUnwindSafe, UnwindSafe};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=FUNCTION | NAME=stdout_unwind_safe | COMPLEXITY=2 | LINES=7

```rust
use crate::sync::mpsc::sync_channel;
use crate::thread;

#[test]
fn stdout_unwind_safe() {
    assert_unwind_safe::<Stdout>();
}
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=FUNCTION | NAME=stdoutlock_unwind_safe | COMPLEXITY=2 | LINES=5

```rust
#[test]
fn stdoutlock_unwind_safe() {
    assert_unwind_safe::<StdoutLock<'_>>();
    assert_unwind_safe::<StdoutLock<'static>>();
}
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=FUNCTION | NAME=stderr_unwind_safe | COMPLEXITY=2 | LINES=4

```rust
#[test]
fn stderr_unwind_safe() {
    assert_unwind_safe::<Stderr>();
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=FUNCTION | NAME=stderrlock_unwind_safe | COMPLEXITY=2 | LINES=5

```rust
#[test]
fn stderrlock_unwind_safe() {
    assert_unwind_safe::<StderrLock<'_>>();
    assert_unwind_safe::<StderrLock<'static>>();
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=FUNCTION | NAME=assert_unwind_safe | COMPLEXITY=2 | LINES=2

```rust
fn assert_unwind_safe<T: UnwindSafe + RefUnwindSafe>() {}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=panic_doesnt_poison | COMPLEXITY=4 | LINES=23

```rust
#[test]
#[cfg_attr(any(target_os = "emscripten", target_os = "wasi"), ignore)] // no threads
fn panic_doesnt_poison() {
    thread::spawn(|| {
        let _a = stdin();
        let _a = _a.lock();
        let _a = stdout();
        let _a = _a.lock();
        let _a = stderr();
        let _a = _a.lock();
        panic!();
    })
    .join()
    .unwrap_err();

    let _a = stdin();
    let _a = _a.lock();
    let _a = stdout();
    let _a = _a.lock();
    let _a = stderr();
    let _a = _a.lock();
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=FUNCTION | NAME=test_lock_stderr | COMPLEXITY=2 | LINES=6

```rust
#[test]
#[cfg_attr(any(target_os = "emscripten", target_os = "wasi"), ignore)] // no threads
fn test_lock_stderr() {
    test_lock(stderr, || stderr().lock());
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=FUNCTION | NAME=test_lock_stdin | COMPLEXITY=2 | LINES=5

```rust
#[test]
#[cfg_attr(any(target_os = "emscripten", target_os = "wasi"), ignore)] // no threads
fn test_lock_stdin() {
    test_lock(stdin, || stdin().lock());
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=FUNCTION | NAME=test_lock_stdout | COMPLEXITY=2 | LINES=5

```rust
#[test]
#[cfg_attr(any(target_os = "emscripten", target_os = "wasi"), ignore)] // no threads
fn test_lock_stdout() {
    test_lock(stdout, || stdout().lock());
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=FUNCTION | NAME=lock | COMPLEXITY=2 | LINES=9

```rust
// Helper trait to make lock testing function generic.
trait Stdio<'a>: 'static
where
    Self::Lock: 'a,
{
    type Lock;
    fn lock(&'a self) -> Self::Lock;
}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=FUNCTION | NAME=lock | COMPLEXITY=5 | LINES=6

```rust
impl<'a> Stdio<'a> for Stderr {
    type Lock = StderrLock<'a>;
    fn lock(&'a self) -> StderrLock<'a> {
        self.lock()
    }
}
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=FUNCTION | NAME=lock | COMPLEXITY=5 | LINES=6

```rust
impl<'a> Stdio<'a> for Stdin {
    type Lock = StdinLock<'a>;
    fn lock(&'a self) -> StdinLock<'a> {
        self.lock()
    }
}
```

## Block 14
**Metadata**: AST_ID=14 | TYPE=FUNCTION | NAME=lock | COMPLEXITY=5 | LINES=6

```rust
impl<'a> Stdio<'a> for Stdout {
    type Lock = StdoutLock<'a>;
    fn lock(&'a self) -> StdoutLock<'a> {
        self.lock()
    }
}
```

## Block 15
**Metadata**: AST_ID=15 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=3

```rust
// Helper trait to make lock testing function generic.
trait StdioOwnedLock: 'static {}
```

## Block 16
**Metadata**: AST_ID=16 | TYPE=IMPL | NAME=UNNAMED | COMPLEXITY=4 | LINES=1

```rust
impl StdioOwnedLock for StderrLock<'static> {}
```

## Block 17
**Metadata**: AST_ID=17 | TYPE=IMPL | NAME=UNNAMED | COMPLEXITY=4 | LINES=1

```rust
impl StdioOwnedLock for StdinLock<'static> {}
```

## Block 18
**Metadata**: AST_ID=18 | TYPE=IMPL | NAME=UNNAMED | COMPLEXITY=4 | LINES=1

```rust
impl StdioOwnedLock for StdoutLock<'static> {}
```

## Block 19
**Metadata**: AST_ID=19 | TYPE=FUNCTION | NAME=test_lock | COMPLEXITY=26 | LINES=69

```rust
// Tests locking on stdio handles by starting two threads and checking that
// they block each other appropriately.
fn test_lock<T, U>(get_handle: fn() -> T, get_locked: fn() -> U)
where
    T: for<'a> Stdio<'a>,
    U: StdioOwnedLock,
{
    // State enum to track different phases of the test, primarily when
    // each lock is acquired and released.
    #[derive(Debug, PartialEq)]
    enum State {
        Start1,
        Acquire1,
        Start2,
        Release1,
        Acquire2,
        Release2,
    }
    use State::*;
    // Logging vector to be checked to make sure lock acquisitions and
    // releases happened in the correct order.
    let log = Arc::new(Mutex::new(Vec::new()));
    let ((tx1, rx1), (tx2, rx2)) = (sync_channel(0), sync_channel(0));
    let th1 = {
        let (log, tx) = (Arc::clone(&log), tx1);
        thread::spawn(move || {
            log.lock().unwrap().push(Start1);
            let handle = get_handle();
            {
                let locked = handle.lock();
                log.lock().unwrap().push(Acquire1);
                tx.send(Acquire1).unwrap(); // notify of acquisition
                tx.send(Release1).unwrap(); // wait for release command
                log.lock().unwrap().push(Release1);
            }
            tx.send(Acquire1).unwrap(); // wait for th2 acquire
            {
                let locked = handle.lock();
                log.lock().unwrap().push(Acquire1);
            }
            log.lock().unwrap().push(Release1);
        })
    };
    let th2 = {
        let (log, tx) = (Arc::clone(&log), tx2);
        thread::spawn(move || {
            tx.send(Start2).unwrap(); // wait for start command
            let locked = get_locked();
            log.lock().unwrap().push(Acquire2);
            tx.send(Acquire2).unwrap(); // notify of acquisition
            tx.send(Release2).unwrap(); // wait for release command
            log.lock().unwrap().push(Release2);
        })
    };
    assert_eq!(rx1.recv().unwrap(), Acquire1); // wait for th1 acquire
    log.lock().unwrap().push(Start2);
    assert_eq!(rx2.recv().unwrap(), Start2); // block th2
    assert_eq!(rx1.recv().unwrap(), Release1); // release th1
    assert_eq!(rx2.recv().unwrap(), Acquire2); // wait for th2 acquire
    assert_eq!(rx1.recv().unwrap(), Acquire1); // block th1
    assert_eq!(rx2.recv().unwrap(), Release2); // release th2
    th2.join().unwrap();
    th1.join().unwrap();
    assert_eq!(
        *log.lock().unwrap(),
        [Start1, Acquire1, Start2, Release1, Acquire2, Release2, Acquire1, Release1]
    );
}
```

---
*Generated by AST tracing system*
