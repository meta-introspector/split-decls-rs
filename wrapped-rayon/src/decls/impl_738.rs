macro_rules! deps {
    () => {
        Fuse!();
    };
}

macro_rules! impl_738 {
    () => {
        deps!();
        impl < 'a > Drop for Fuse < 'a > { # [inline] fn drop (& mut self) { if thread :: panicking () { self . 0 . store (true , Ordering :: Relaxed) ; } } }
    };
}

impl_738!();