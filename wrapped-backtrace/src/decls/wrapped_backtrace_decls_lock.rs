use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(feature = "std")]
mod lock {
    use std::cell::Cell;
    use std::sync::{Mutex, MutexGuard};
    /// A "Maybe" LockGuard
    pub struct LockGuard(Option<MutexGuard<'static, ()>>);
    /// The global lock
    static LOCK: Mutex<()> = Mutex::new(());
    thread_local!(static LOCK_HELD : Cell < bool > = const { Cell::new(false) });
    impl Drop for LockGuard {
        fn drop(&mut self) {
            if self.0.is_some() {
                LOCK_HELD
                    .with(|slot| {
                        assert!(slot.get());
                        slot.set(false);
                    });
            }
        }
    }
    /// Acquire a partially unsound(!!!) global re-entrant lock over
    /// backtrace's internals.
    ///
    /// That is, this lock can be acquired as many times as you want
    /// on a single thread without deadlocking, allowing one thread
    /// to acquire exclusive access to the ability to make backtraces.
    /// Calls to this locking function are freely sprinkled in every place
    /// where that needs to be enforced.
    ///
    ///
    /// # Why
    ///
    /// This was first introduced to guard uses of Windows' dbghelp API,
    /// which isn't threadsafe. It's unclear if other things now rely on
    /// this locking.
    ///
    ///
    /// # How
    ///
    /// The basic idea is to have a single global mutex, and a thread_local
    /// boolean saying "yep this is the thread that acquired the mutex".
    ///
    /// The first time a thread acquires the lock, it is handed a
    /// `LockGuard(Some(..))` that will actually release the lock on Drop.
    /// All subsequence attempts to lock on the same thread will see
    /// that their thread acquired the lock, and get `LockGuard(None)`
    /// which will do nothing when dropped.
    ///
    ///
    /// # Safety
    ///
    /// As long as you only ever assign the returned LockGuard to a freshly
    /// declared local variable, it will do its job correctly, as the "first"
    /// LockGuard will strictly outlive all subsequent LockGuards and
    /// properly release the lock when the thread is done with backtracing.
    ///
    /// However if you ever attempt to store a LockGuard beyond the scope
    /// it was acquired in, it might actually be a `LockGuard(None)` that
    /// doesn't actually hold the lock! In this case another thread might
    /// acquire the lock and you'll get races this system was intended to
    /// avoid!
    ///
    /// This is why this is "partially unsound". As a public API this would
    /// be unacceptable, but this is crate-private, and if you use this in
    /// the most obvious and simplistic way it Just Works™.
    ///
    /// Note however that std specifically bypasses this lock, and uses
    /// the `*_unsynchronized` backtrace APIs. This is "fine" because
    /// it wraps its own calls to backtrace in a non-reentrant Mutex
    /// that prevents two backtraces from getting interleaved during printing.
    pub fn lock() -> LockGuard {
        if LOCK_HELD.with(|l| l.get()) {
            return LockGuard(None);
        }
        LOCK_HELD.with(|s| s.set(true));
        LockGuard(Some(LOCK.lock().unwrap()))
    }
}
