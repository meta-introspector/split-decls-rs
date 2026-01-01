# AST Trace: ../rust/library/std/src/sync/barrier.rs

Generated 9 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3

```rust
use crate::fmt;
use crate::panic::RefUnwindSafe;
use crate::sync::nonpoison::{Condvar, Mutex};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=7 | LINES=23

```rust
/// A barrier enables multiple threads to synchronize the beginning
/// of some computation.
///
/// # Examples
///
/// ```
/// use std::sync::Barrier;
/// use std::thread;
///
/// let n = 10;
/// let barrier = Barrier::new(n);
/// thread::scope(|s| {
///     for _ in 0..n {
///         // The same messages will be printed together.
///         // You will NOT see any interleaving.
///         s.spawn(|| {
///             println!("before wait");
///             barrier.wait();
///             println!("after wait");
///         });
///     }
/// });
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=STRUCT | NAME=Barrier | COMPLEXITY=2 | LINES=7

```rust
/// ```
#[stable(feature = "rust1", since = "1.0.0")]
pub struct Barrier {
    lock: Mutex<BarrierState>,
    cvar: Condvar,
    num_threads: usize,
}
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=IMPL | NAME=UNNAMED | COMPLEXITY=4 | LINES=3

```rust
#[stable(feature = "unwind_safe_lock_refs", since = "1.12.0")]
impl RefUnwindSafe for Barrier {}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=STRUCT | NAME=BarrierState | COMPLEXITY=2 | LINES=6

```rust
// The inner state of a double barrier
struct BarrierState {
    count: usize,
    generation_id: usize,
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=FUNCTION | NAME=BarrierWaitResult(bool); | COMPLEXITY=6 | LINES=21

```rust
/// A `BarrierWaitResult` is returned by [`Barrier::wait()`] when all threads
/// in the [`Barrier`] have rendezvoused.
///
/// # Examples
///
/// ```
/// use std::sync::Barrier;
///
/// let barrier = Barrier::new(1);
/// let barrier_wait_result = barrier.wait();
/// ```
#[stable(feature = "rust1", since = "1.0.0")]
pub struct BarrierWaitResult(bool);

#[stable(feature = "std_debug", since = "1.16.0")]
impl fmt::Debug for Barrier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Barrier").finish_non_exhaustive()
    }
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=wait | COMPLEXITY=19 | LINES=74

```rust
impl Barrier {
    /// Creates a new barrier that can block a given number of threads.
    ///
    /// A barrier will block `n`-1 threads which call [`wait()`] and then wake
    /// up all threads at once when the `n`th thread calls [`wait()`].
    ///
    /// [`wait()`]: Barrier::wait
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::Barrier;
    ///
    /// let barrier = Barrier::new(10);
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_const_stable(feature = "const_barrier", since = "1.78.0")]
    #[must_use]
    #[inline]
    pub const fn new(n: usize) -> Barrier {
        Barrier {
            lock: Mutex::new(BarrierState { count: 0, generation_id: 0 }),
            cvar: Condvar::new(),
            num_threads: n,
        }
    }

    /// Blocks the current thread until all threads have rendezvoused here.
    ///
    /// Barriers are re-usable after all threads have rendezvoused once, and can
    /// be used continuously.
    ///
    /// A single (arbitrary) thread will receive a [`BarrierWaitResult`] that
    /// returns `true` from [`BarrierWaitResult::is_leader()`] when returning
    /// from this function, and all other threads will receive a result that
    /// will return `false` from [`BarrierWaitResult::is_leader()`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::Barrier;
    /// use std::thread;
    ///
    /// let n = 10;
    /// let barrier = Barrier::new(n);
    /// thread::scope(|s| {
    ///     for _ in 0..n {
    ///         // The same messages will be printed together.
    ///         // You will NOT see any interleaving.
    ///         s.spawn(|| {
    ///             println!("before wait");
    ///             barrier.wait();
    ///             println!("after wait");
    ///         });
    ///     }
    /// });
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn wait(&self) -> BarrierWaitResult {
        let mut lock = self.lock.lock();
        let local_gen = lock.generation_id;
        lock.count += 1;
        if lock.count < self.num_threads {
            let _guard = self.cvar.wait_while(lock, |state| local_gen == state.generation_id);
            BarrierWaitResult(false)
        } else {
            lock.count = 0;
            lock.generation_id = lock.generation_id.wrapping_add(1);
            self.cvar.notify_all();
            BarrierWaitResult(true)
        }
    }
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=5 | LINES=7

```rust
#[stable(feature = "std_debug", since = "1.16.0")]
impl fmt::Debug for BarrierWaitResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BarrierWaitResult").field("is_leader", &self.is_leader()).finish()
    }
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=FUNCTION | NAME=is_leader | COMPLEXITY=9 | LINES=23

```rust
impl BarrierWaitResult {
    /// Returns `true` if this thread is the "leader thread" for the call to
    /// [`Barrier::wait()`].
    ///
    /// Only one thread will have `true` returned from their result, all other
    /// threads will have `false` returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::Barrier;
    ///
    /// let barrier = Barrier::new(1);
    /// let barrier_wait_result = barrier.wait();
    /// println!("{:?}", barrier_wait_result.is_leader());
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    #[must_use]
    pub fn is_leader(&self) -> bool {
        self.0
    }
}
```

---
*Generated by AST tracing system*
