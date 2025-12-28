macro_rules! deps {
    () => {
        Arc!();
    };
}

macro_rules! impl_139 {
    () => {
        deps!();
        impl < T : ? Sized > Arc < T > { # [doc = " Provides mutable access to the contents _if_ the `Arc` is uniquely owned."] # [inline] pub (crate) fn get_mut (this : & mut Self) -> Option < & mut T > { if this . is_unique () { unsafe { Some (& mut (* this . ptr ()) . data) } } else { None } } # [doc = " Whether or not the `Arc` is uniquely owned (is the refcount 1?)."] pub (crate) fn is_unique (& self) -> bool { self . inner () . count . load (Acquire) == 1 } }
    };
}

impl_139!();