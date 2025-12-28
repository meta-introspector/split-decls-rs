macro_rules! deps {
    () => {
        Finder!();
        PrefilterState!();
        FindIter!();
    };
}

macro_rules! impl_367 {
    () => {
        deps!();
        impl < 'h , 'n > FindIter < 'h , 'n > { # [inline (always)] pub (crate) fn new (haystack : & 'h [u8] , finder : Finder < 'n > ,) -> FindIter < 'h , 'n > { let prestate = PrefilterState :: new () ; FindIter { haystack , prestate , finder , pos : 0 } } # [doc = " Convert this iterator into its owned variant, such that it no longer"] # [doc = " borrows the finder and needle."] # [doc = ""] # [doc = " If this is already an owned iterator, then this is a no-op. Otherwise,"] # [doc = " this copies the needle."] # [doc = ""] # [doc = " This is only available when the `alloc` feature is enabled."] # [cfg (feature = "alloc")] # [inline] pub fn into_owned (self) -> FindIter < 'h , 'static > { FindIter { haystack : self . haystack , prestate : self . prestate , finder : self . finder . into_owned () , pos : self . pos , } } }
    };
}

impl_367!();