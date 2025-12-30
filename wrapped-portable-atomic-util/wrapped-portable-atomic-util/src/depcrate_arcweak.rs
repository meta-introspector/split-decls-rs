// Generated macro for Weak (struct)
macro_rules! Depcrate_arcWeak {
() => {
// Module: crate::arc
// Provides: {"Weak"}
// Dependencies: {}
# [doc = " `Weak` is a version of [`Arc`] that holds a non-owning reference to the"] # [doc = " managed allocation."] # [doc = ""] # [doc = " The allocation is accessed by calling [`upgrade`] on the `Weak`"] # [doc = " pointer, which returns an <code>[Option]<[Arc]\\<T>></code>."] # [doc = ""] # [doc = " This is an equivalent to [`std::sync::Weak`], but using [portable-atomic] for synchronization."] # [doc = " See the documentation for [`std::sync::Weak`] for more details."] # [doc = ""] # [doc = " <!-- TODO: support coercing `Weak<T>` to `Weak<U>` with testing, if optional cfg `portable_atomic_unstable_coerce_unsized` is enabled -->"] # [doc = " **Note:** Unlike `std::sync::Weak`, coercing `Weak<T>` to `Weak<U>` is not possible, not even if"] # [doc = " the optional cfg `portable_atomic_unstable_coerce_unsized` is enabled."] # [doc = ""] # [doc = " [`upgrade`]: Weak::upgrade"] # [doc = " [portable-atomic]: https://crates.io/crates/portable-atomic"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use portable_atomic_util::Arc;"] # [doc = " use std::thread;"] # [doc = ""] # [doc = " let five = Arc::new(5);"] # [doc = " let weak_five = Arc::downgrade(&five);"] # [doc = ""] # [doc = " # let t ="] # [doc = " thread::spawn(move || {"] # [doc = "     let five = weak_five.upgrade().unwrap();"] # [doc = "     assert_eq!(*five, 5);"] # [doc = " });"] # [doc = " # t.join().unwrap(); // join thread to avoid https://github.com/rust-lang/miri/issues/1371"] # [doc = " ```"] pub struct Weak < T : ? Sized > { ptr : NonNull < ArcInner < T > > , }
};
}
