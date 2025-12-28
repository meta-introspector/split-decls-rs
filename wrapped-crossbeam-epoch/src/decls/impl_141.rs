macro_rules! deps {
    () => {
        Queue!();
    };
}

macro_rules! impl_141 {
    () => {
        deps!();
        impl < T > Drop for Queue < T > { fn drop (& mut self) { unsafe { let guard = unprotected () ; while self . try_pop (guard) . is_some () { } let sentinel = self . head . load (Relaxed , guard) ; drop (sentinel . into_owned ()) ; } } }
    };
}

impl_141!();