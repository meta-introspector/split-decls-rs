macro_rules! deps {
    () => {
        Atomic!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl < T > From < * const T > for Atomic < T > { # [doc = " Returns a new atomic pointer pointing to `raw`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::ptr;"] # [doc = " use crossbeam_epoch::Atomic;"] # [doc = ""] # [doc = " let a = Atomic::<i32>::from(ptr::null::<i32>());"] # [doc = " ```"] fn from (raw : * const T) -> Self { Self :: from_ptr (raw as * mut ()) } }
    };
}

impl_31!();