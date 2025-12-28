macro_rules! deps {
    () => {
        Fuse!();
    };
}

macro_rules! impl_739 {
    () => {
        deps!();
        impl < 'a > Fuse < 'a > { # [inline] fn panicked (& self) -> bool { self . 0 . load (Ordering :: Relaxed) } }
    };
}

impl_739!()