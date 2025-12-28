macro_rules! deps {
    () => {
        Shared!();
        Atomic!();
        Owned!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl < T > Shared < '_ , T > { # [doc = " Converts the pointer to a raw pointer (without the tag)."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use crossbeam_epoch::{self as epoch, Atomic, Owned};"] # [doc = " use std::sync::atomic::Ordering::SeqCst;"] # [doc = ""] # [doc = " let o = Owned::new(1234);"] # [doc = " let raw = &*o as *const _;"] # [doc = " let a = Atomic::from(o);"] # [doc = ""] # [doc = " let guard = &epoch::pin();"] # [doc = " let p = a.load(SeqCst, guard);"] # [doc = " assert_eq!(p.as_raw(), raw);"] # [doc = " # unsafe { drop(a.into_owned()); } // avoid leak"] # [doc = " ```"] pub fn as_raw (& self) -> * const T { let (raw , _) = decompose_tag :: < T > (self . data) ; raw as * const _ } }
    };
}

impl_54!()