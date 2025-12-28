macro_rules! deps {
    () => {
        Unparker!();
    };
}

macro_rules! Parker {
    () => {
        deps!();
        # [doc = " A thread parking primitive."] # [doc = ""] # [doc = " Conceptually, each `Parker` has an associated token which is initially not present:"] # [doc = ""] # [doc = " * The [`park`] method blocks the current thread unless or until the token is available, at"] # [doc = "   which point it automatically consumes the token."] # [doc = ""] # [doc = " * The [`park_timeout`] and [`park_deadline`] methods work the same as [`park`], but block for"] # [doc = "   a specified maximum time."] # [doc = ""] # [doc = " * The [`unpark`] method atomically makes the token available if it wasn't already. Because the"] # [doc = "   token is initially absent, [`unpark`] followed by [`park`] will result in the second call"] # [doc = "   returning immediately."] # [doc = ""] # [doc = " In other words, each `Parker` acts a bit like a spinlock that can be locked and unlocked using"] # [doc = " [`park`] and [`unpark`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::thread;"] # [doc = " use std::time::Duration;"] # [doc = " use crossbeam_utils::sync::Parker;"] # [doc = ""] # [doc = " let p = Parker::new();"] # [doc = " let u = p.unparker().clone();"] # [doc = ""] # [doc = " // Make the token available."] # [doc = " u.unpark();"] # [doc = " // Wakes up immediately and consumes the token."] # [doc = " p.park();"] # [doc = ""] # [doc = " # let t ="] # [doc = " thread::spawn(move || {"] # [doc = "     thread::sleep(Duration::from_millis(500));"] # [doc = "     u.unpark();"] # [doc = " });"] # [doc = ""] # [doc = " // Wakes up when `u.unpark()` provides the token."] # [doc = " p.park();"] # [doc = " # t.join().unwrap(); // join thread to avoid https://github.com/rust-lang/miri/issues/1371"] # [doc = " ```"] # [doc = ""] # [doc = " [`park`]: Parker::park"] # [doc = " [`park_timeout`]: Parker::park_timeout"] # [doc = " [`park_deadline`]: Parker::park_deadline"] # [doc = " [`unpark`]: Unparker::unpark"] pub struct Parker { unparker : Unparker , _marker : PhantomData < * const () > , }
    };
}

Parker!();