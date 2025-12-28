macro_rules! deps {
    () => {
        Arc!();
    };
}

macro_rules! impl_140 {
    () => {
        deps!();
        impl < T : ? Sized > Drop for Arc < T > { # [inline] fn drop (& mut self) { if self . inner () . count . fetch_sub (1 , Release) != 1 { return ; } self . inner () . count . load (Acquire) ; unsafe { self . drop_slow () ; } } }
    };
}

impl_140!();