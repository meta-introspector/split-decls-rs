macro_rules! deps {
    () => {
        Owned!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl < T > Owned < T > { # [doc = " Returns a new owned pointer pointing to `raw`."] # [doc = ""] # [doc = " This function is unsafe because improper use may lead to memory problems. Argument `raw`"] # [doc = " must be a valid pointer. Also, a double-free may occur if the function is called twice on"] # [doc = " the same raw pointer."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if `raw` is not properly aligned."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The given `raw` should have been derived from `Owned`, and one `raw` should not be converted"] # [doc = " back by `Owned::from_raw()` multiple times."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use crossbeam_epoch::Owned;"] # [doc = ""] # [doc = " let o = unsafe { Owned::from_raw(Box::into_raw(Box::new(1234))) };"] # [doc = " ```"] pub unsafe fn from_raw (raw : * mut T) -> Self { let raw = raw . cast :: < () > () ; ensure_aligned :: < T > (raw) ; unsafe { Self :: from_ptr (raw) } } # [doc = " Converts the owned pointer into a `Box`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use crossbeam_epoch::Owned;"] # [doc = ""] # [doc = " let o = Owned::new(1234);"] # [doc = " let b: Box<i32> = o.into_box();"] # [doc = " assert_eq!(*b, 1234);"] # [doc = " ```"] pub fn into_box (self) -> Box < T > { let (raw , _) = decompose_tag :: < T > (self . data) ; mem :: forget (self) ; unsafe { Box :: from_raw (raw . cast :: < T > ()) } } # [doc = " Allocates `value` on the heap and returns a new owned pointer pointing to it."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use crossbeam_epoch::Owned;"] # [doc = ""] # [doc = " let o = Owned::new(1234);"] # [doc = " ```"] pub fn new (init : T) -> Self { Self :: init (init) } }
    };
}

impl_36!();