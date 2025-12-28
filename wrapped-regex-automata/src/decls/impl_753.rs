macro_rules! deps {
    () => {
        NonMaxUsize!();
    };
}

macro_rules! impl_753 {
    () => {
        deps!();
        impl NonMaxUsize { # [doc = " Create a new `NonMaxUsize` from the given value."] # [doc = ""] # [doc = " This returns `None` only when the given value is equal to `usize::MAX`."] # [inline] pub fn new (value : usize) -> Option < NonMaxUsize > { NonZeroUsize :: new (value . wrapping_add (1)) . map (NonMaxUsize) } # [doc = " Return the underlying `usize` value. The returned value is guaranteed"] # [doc = " to not equal `usize::MAX`."] # [inline] pub fn get (self) -> usize { self . 0 . get () . wrapping_sub (1) } }
    };
}

impl_753!();