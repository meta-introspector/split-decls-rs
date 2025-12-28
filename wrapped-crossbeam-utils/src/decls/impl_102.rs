macro_rules! deps {
    () => {
        Inner!();
        Unparker!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        impl Unparker { # [doc = " Atomically makes the token available if it is not already."] # [doc = ""] # [doc = " This method will wake up the thread blocked on [`park`] or [`park_timeout`], if there is"] # [doc = " any."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::thread;"] # [doc = " use std::time::Duration;"] # [doc = " use crossbeam_utils::sync::Parker;"] # [doc = ""] # [doc = " let p = Parker::new();"] # [doc = " let u = p.unparker().clone();"] # [doc = ""] # [doc = " # let t ="] # [doc = " thread::spawn(move || {"] # [doc = "     thread::sleep(Duration::from_millis(500));"] # [doc = "     u.unpark();"] # [doc = " });"] # [doc = ""] # [doc = " // Wakes up when `u.unpark()` provides the token."] # [doc = " p.park();"] # [doc = " # t.join().unwrap(); // join thread to avoid https://github.com/rust-lang/miri/issues/1371"] # [doc = " ```"] # [doc = ""] # [doc = " [`park`]: Parker::park"] # [doc = " [`park_timeout`]: Parker::park_timeout"] pub fn unpark (& self) { self . inner . unpark () } # [doc = " Converts an `Unparker` into a raw pointer."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use crossbeam_utils::sync::{Parker, Unparker};"] # [doc = ""] # [doc = " let p = Parker::new();"] # [doc = " let u = p.unparker().clone();"] # [doc = " let raw = Unparker::into_raw(u);"] # [doc = " # let _ = unsafe { Unparker::from_raw(raw) };"] # [doc = " ```"] pub fn into_raw (this : Self) -> * const () { Arc :: into_raw (this . inner) . cast :: < () > () } # [doc = " Converts a raw pointer into an `Unparker`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This method is safe to use only with pointers returned by [`Unparker::into_raw`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use crossbeam_utils::sync::{Parker, Unparker};"] # [doc = ""] # [doc = " let p = Parker::new();"] # [doc = " let u = p.unparker().clone();"] # [doc = ""] # [doc = " let raw = Unparker::into_raw(u);"] # [doc = " let u = unsafe { Unparker::from_raw(raw) };"] # [doc = " ```"] pub unsafe fn from_raw (ptr : * const ()) -> Self { Self { inner : unsafe { Arc :: from_raw (ptr . cast :: < Inner > ()) } , } } }
    };
}

impl_102!();