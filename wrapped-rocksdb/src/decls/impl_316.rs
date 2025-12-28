macro_rules! deps {
    () => {
        PropName!();
        PropertyName!();
    };
}

macro_rules! impl_316 {
    () => {
        deps!();
        impl std :: ops :: Deref for PropertyName { type Target = PropName ; # [inline] fn deref (& self) -> & Self :: Target { unsafe { & * (ptr :: from_ref :: < CStr > (self . 0 . as_c_str ()) as * const PropName) } } }
    };
}

impl_316!();