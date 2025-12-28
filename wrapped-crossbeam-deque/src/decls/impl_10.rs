macro_rules! deps {
    () => {
        Inner!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl < T > Drop for Inner < T > { fn drop (& mut self) { let b = * self . back . get_mut () ; let f = * self . front . get_mut () ; unsafe { let buffer = self . buffer . load (Ordering :: Relaxed , epoch :: unprotected ()) ; let mut i = f ; while i != b { buffer . deref () . at (i) . drop_in_place () ; i = i . wrapping_add (1) ; } buffer . into_owned () . into_box () . dealloc () ; } } }
    };
}

impl_10!();