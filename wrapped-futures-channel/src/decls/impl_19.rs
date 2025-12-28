macro_rules! deps {
    () => {
        Queue!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl < T > Drop for Queue < T > { fn drop (& mut self) { unsafe { let mut cur = * self . tail . get () ; while ! cur . is_null () { let next = (* cur) . next . load (Ordering :: Relaxed) ; drop (Box :: from_raw (cur)) ; cur = next ; } } } }
    };
}

impl_19!()