macro_rules! deps {
    () => {
        Shared!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl < T > From < * const T > for Shared < '_ , T > { # [doc = " Returns a new pointer pointing to `raw`."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if `raw` is not properly aligned."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use crossbeam_epoch::Shared;"] # [doc = ""] # [doc = " let p = Shared::from(Box::into_raw(Box::new(1234)) as *const _);"] # [doc = " assert!(!p.is_null());"] # [doc = " # unsafe { drop(p.into_owned()); } // avoid leak"] # [doc = " ```"] fn from (raw : * const T) -> Self { let raw = raw as * mut () ; ensure_aligned :: < T > (raw) ; unsafe { Self :: from_ptr (raw) } } }
    };
}

impl_56!()