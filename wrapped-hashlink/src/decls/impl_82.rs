macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_82 {
    () => {
        deps!();
        impl < K , V > Drop for IntoIter < K , V > { # [inline] fn drop (& mut self) { for _ in 0 .. self . remaining { unsafe { let tail = self . tail . as_ptr () ; self . tail = Some ((* tail) . links . value . prev) ; (* tail) . take_entry () ; let _ = Box :: from_raw (tail) ; } } } }
    };
}

impl_82!();